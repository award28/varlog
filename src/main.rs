use std::env;

use actix_web::{web, middleware, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use hmac::{Hmac, Mac};
use sha2::Sha256;

mod servers;
mod http;
mod auth;
mod logs;

#[derive(Clone)]
struct AppConfig {
    key: Hmac<Sha256>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let key = Hmac::new_from_slice(
            env::var("JWT_SIGNING_KEY")
            .expect("JWT signing key should be found in the environment.")
            .as_bytes(),
        ).expect("JWT signing key should be parsable.");

        App::new()
            .app_data(web::Data::new(AppConfig{ key }))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("v1")
                .service(auth::routes::register)
                .service(
                    web::scope("")
                    .wrap(auth::middleware::AuthRequired::default())
                    .service(logs::routes::logs)
                    .service(logs::routes::log)
                    .service(servers::routes::servers_logs)
                    .service(servers::routes::servers_log)
                )
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
