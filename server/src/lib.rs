#[macro_use] extern crate log;

use anyhow::Result;
use actix_web::{web, middleware, App, HttpServer};
use auth::middleware::AuthRequired;

pub mod conf;
mod servers;
mod logs;
mod auth;
mod http;

pub async fn run(config: conf::AppConfig) -> Result<()> {
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(config.clone()))
            .wrap(middleware::Logger::default())
            .configure(init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let mut scope = web::scope("");
    scope = logs::register_services(scope);
    scope = servers::register_services(scope);

    let mut v1_endpoints = web::scope("v1");
    v1_endpoints = auth::register_services(v1_endpoints);
    cfg.service(v1_endpoints.service(
        scope.wrap(AuthRequired::default()),
    ));
}
