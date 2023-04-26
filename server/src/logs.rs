use actix_web::{Scope, web};

pub mod routes;
pub mod rev_log_reader;

pub fn service() -> Scope {
    web::scope("")
        .service(routes::log)
        .service(routes::logs)
}
