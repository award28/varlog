use actix_web::{Scope, web};

pub mod routes;

pub fn service() -> Scope {
    web::scope("")
        .service(routes::servers)
        .service(routes::servers_log)
        .service(routes::servers_logs)
}
