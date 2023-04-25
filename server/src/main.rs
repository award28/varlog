#[macro_use] extern crate log;

use anyhow::Result;
use actix_web::{web, middleware, App, HttpServer};

mod servers;
mod http;
mod auth;
mod logs;
mod conf;


#[actix_web::main]
async fn main() -> Result<()> {
    let config = conf::AppConfig::new()?;
    config.configure().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
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
    .await?;
    Ok(())
}
