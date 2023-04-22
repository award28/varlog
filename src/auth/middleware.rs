use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::{future::LocalBoxFuture, FutureExt};


// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthRequired;

impl AuthRequired {
    pub fn default() -> Self {
        AuthRequired{}
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
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
        match super::Claims::try_from(&req) {
            Err(e) => Box::pin(async move {
                if e.eq("Internal Server Error") {
                    return Err(actix_web::error::ErrorInternalServerError(e));
                }
                Err(actix_web::error::ErrorUnauthorized(e))
            }),
            Ok(claims) => {
                req.extensions_mut().insert(claims);
                self.service.call(req).boxed_local()
            }
        }
    }
}
