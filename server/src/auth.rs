use actix_web::{Scope, web};

pub mod access_data;
pub mod middleware;
pub mod claims;
pub mod routes;

pub fn service() -> Scope {
    web::scope("")
        .service(routes::register)
}
