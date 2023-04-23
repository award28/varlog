use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpMessage {
    pub message: String,
}

#[derive(Serialize)]
pub struct HttpError {
    pub error: String,
}

pub fn internal_err() -> HttpResponse {
    HttpResponse::InternalServerError().json(
        HttpError {
            error: String::from("Internal Server Error"),
        }
    )
}
