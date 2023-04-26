use actix_web::Scope;

pub mod access_data;
pub mod middleware;
pub mod claims;
mod routes;

pub fn register_services(scope: Scope) -> Scope {
    scope.service(routes::register)
}
