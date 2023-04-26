use actix_web::Scope;

mod routes;

pub fn register_services(scope: Scope) -> Scope {
    scope
        .service(routes::servers)
        .service(routes::servers_log)
        .service(routes::servers_logs)
}
