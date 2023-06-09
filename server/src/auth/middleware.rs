use std::future::{ready, Ready};
use actix_web::{
    dev::{
        forward_ready,
        Service,
        ServiceRequest,
        ServiceResponse,
        Transform
    },
    Error,
    HttpMessage,
    error::ErrorUnauthorized,
};
use futures_util::{future::LocalBoxFuture, FutureExt};

use super::claims::Claims;

#[derive(Default)]
pub struct AuthRequired;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
impl<S, B> Transform<S, ServiceRequest> for AuthRequired
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthRequiredMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthRequiredMiddleware { service }))
    }
}

pub struct AuthRequiredMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthRequiredMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match Claims::try_from(&req) {
            Err(e) => Box::pin(async move {
                Err(ErrorUnauthorized(e))
            }),
            Ok(claims) => {
                let path = req.path();
                let client_id = &claims.jti;
                info!("{{\"path\": {path}, \"client_id\": {client_id}}}");
                req.extensions_mut().insert(claims);
                self.service.call(req).boxed_local()
            }
        }
    }
}
