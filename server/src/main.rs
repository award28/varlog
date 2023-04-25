use std::env;

use actix_web::{web, middleware, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;

mod servers;
mod http;
mod auth;
mod logs;

#[derive(Clone)]
struct AppConfig {
    key: Hmac<Sha256>,
    registry_url: String,
}

#[derive(Serialize)]
struct RegistryRequest {
    hostname: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let key = Hmac::new_from_slice(
        env::var("JWT_SIGNING_KEY")
        .expect("JWT signing key should be found in the environment.")
        .as_bytes(),
    )
        .expect("JWT signing key should be parsable.");


    let registry_url = env::var("REGISTRY_URL")
        .expect("Registry should be found in the environment.");

    let hostname = env::var("HOSTNAME")
        .expect("Hostname should be found in the environment.");

    println!("Registering hostname...");
    {
        let registry_url = registry_url.clone();
        let client = reqwest::Client::new();
        let resp = client.post(format!("{registry_url}/register"))
            .json(&RegistryRequest { hostname })
            .send().await;

        if let Err(e) = resp {
            println!("Error while registering hostname: {}.", e);
        } else {
            println!("Successfully registered hostname.");
        }
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppConfig{ 
                key: key.clone(), 
                registry_url: registry_url.clone(),
            }))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("v1")
                .service(auth::routes::register)
                .service(
                    web::scope("")
                    .wrap(auth::middleware::AuthRequired::default())
                    .service(logs::routes::log)
                    .service(logs::routes::logs)
                    .service(servers::routes::available_servers)
                    .service(servers::routes::servers_log)
                    .service(servers::routes::servers_logs)
                )
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
