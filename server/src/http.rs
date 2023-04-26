use std::fmt;

use actix_web::HttpResponse;
use serde::Serialize;

const SERVICE_UNAVAILABLE_ERR: &str = "Temporarily Unavailable.";
const INTERNAL_SERVER_ERR: &str = "Internal Server Error.";

#[derive(Serialize)]
pub struct HttpMessage {
    pub message: String,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Debug)]
pub enum Error {
    InternalServiceError,
    ServiceUnavailable,
    BadRequest(String),
    Forbidden(String),
}

#[derive(Serialize)]
struct HttpErrorResponse {
    pub error: String,
}

impl Error {
    pub fn to_http_response(&self) -> HttpResponse {
        let mut res = match self {
            Error::Forbidden(_) => HttpResponse::Forbidden(),
            Error::BadRequest(_) => HttpResponse::BadRequest(),
            Error::ServiceUnavailable => HttpResponse::ServiceUnavailable(),
            Error::InternalServiceError => HttpResponse::InternalServerError(),
        };

        res.json(HttpErrorResponse {
            error: format!("{self}"),
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match (*self).clone() {
            Error::Forbidden(s) => s,
            Error::BadRequest(s) => s,
            Error::ServiceUnavailable => String::from(SERVICE_UNAVAILABLE_ERR),
            Error::InternalServiceError => String::from(INTERNAL_SERVER_ERR),
        };
        write!(f, "{}", output)
    }
}

impl std::error::Error for Error {}
