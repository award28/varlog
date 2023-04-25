use actix_web::{web, middleware, App, HttpServer, Responder, post, HttpResponse};

use env_logger::Env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    hostname: String,
}

#[post("/register")]
async fn register(
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    println!("Registering {}", req.hostname);
    HttpResponse::NoContent()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| App::new()
        .wrap(middleware::Logger::default())
        .service(register)
    )
    .bind(("0.0.0.0", 8080))?
    .run()
    .await

}
