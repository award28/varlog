use actix_web::{web, middleware, App, HttpServer, Responder, post, get, HttpResponse};

use env_logger::Env;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    hostnames: Mutex<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    hostname: String,
}

#[post("/register")]
async fn register(
    app_state: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    if let Ok(mut hostnames) = app_state.hostnames.lock() {
        (*hostnames).push(req.hostname.clone());
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[derive(Debug, Serialize)]
struct RegisteredResponse {
    hostnames: Vec<String>,
}

#[get("/registered")]
async fn registered(
    app_state: web::Data<AppState>,
) -> impl Responder {
    if let Ok(hostnames) = app_state.hostnames.lock() {
        HttpResponse::Ok().json(RegisteredResponse {
            hostnames: (*hostnames).clone(),
        })
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        hostnames: Mutex::new(vec![]),
    });

    HttpServer::new(move || App::new()
        .app_data(app_state.clone())
        .wrap(middleware::Logger::default())
        .service(register)
        .service(registered)
    )
    .bind(("0.0.0.0", 8080))?
    .run()
    .await

}
