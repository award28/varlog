use std::env;

use actix_web::{web, get, post, middleware, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use hmac::{Hmac, Mac};
use sha2::Sha256;

mod auth;

#[post("/auth/register")]
async fn register(
    config: web::Data<AppConfig>,
    access_data: web::Json<auth::AccessData>,
) -> impl Responder {
    // TODO: Respond with JSON Message
    match auth::Claims::sign(&config.key,access_data.into_inner()) {
        Ok(token_str) => HttpResponse::Ok().body(token_str),
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)),
    }
}

#[get("/logs")]
async fn log(claims: web::ReqData<auth::Claims>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:#?}", claims.data))
}

#[derive(Clone)]
struct AppConfig {
    key: Hmac<Sha256>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let key = Hmac::new_from_slice(
            env::var("JWT_SIGNING_KEY")
            .unwrap_or(String::from("horse-battery-staple-gun"))
            .as_bytes(),
        ).expect("Key should be parsable");

        App::new()
            .app_data(web::Data::new(AppConfig{ key }))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("v1")
                .service(register)
                .service(
                    web::scope("")
                    .wrap(auth::middleware::AuthRequired::default())
                    .service(log)
                )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
