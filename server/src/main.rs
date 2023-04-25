#[macro_use] extern crate log;

use anyhow::Result;
use actix_web::{web, middleware, App, HttpServer};
use auth::middleware::AuthRequired;

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
        let auth_scoped_service = web::scope("")
            .wrap(AuthRequired::default())
            .service(logs::service())
            .service(servers::service());

        let v1_endpoints = web::scope("v1")
            .service(auth::service())
            .service(auth_scoped_service);

        App::new().app_data(web::Data::new(config.clone()))
            .wrap(middleware::Logger::default())
            .service(v1_endpoints)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
