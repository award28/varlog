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
        let mut scope = web::scope("");
        scope = logs::register_services(scope);
        scope = servers::register_services(scope);

        let mut v1_endpoints = web::scope("v1");
        v1_endpoints = auth::register_services(v1_endpoints);

        App::new().app_data(web::Data::new(config.clone()))
            .wrap(middleware::Logger::default())
            .service(v1_endpoints.service(
                    scope.wrap(AuthRequired::default(),
            )))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
