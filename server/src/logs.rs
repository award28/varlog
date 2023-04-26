use actix_web::Scope;

mod rev_log_reader;
pub mod routes;

pub fn register_services(scope: Scope) -> Scope {
    scope
        .service(routes::log)
        .service(routes::logs)
}
