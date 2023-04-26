use actix_web::{web, post, HttpResponse, Responder};
use regex::RegexSet;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::http::Error;

use super::claims::Claims;
use super::access_data::AccessData;

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterRequest {
    #[validate(length(min = 1), custom = "validate_paths")]
    pub paths: Vec<String>,
    pub servers: Vec<String>,
}

fn validate_paths(paths: &Vec<String>) -> Result<(), ValidationError> {
    // Validates the user is in the /var/log directory
    // and isn't trying to access a parent directory
    if paths.iter().any(|path| path.contains("..")) {
        return Err(ValidationError::new(
            "Parent directory access is forbidden."
        ));
    }

    // Performs validation that the regex is properly formatted
    RegexSet::new(paths.clone())
        .map_err(|_| ValidationError::new(
            "Invalid file paths"
        )
    )?;

    Ok(())
}

#[derive(Serialize)]
struct TokenResponse {
    token: String,
    expires: i64,
}

#[post("/auth/register")]
async fn register(
    config: web::Data<crate::conf::AppConfig>,
    register_req: web::Json<RegisterRequest>,
) -> impl Responder {
    if let Err(e) = register_req.validate() {
        return Error::BadRequest(format!("{e}")).to_http_response();
    }

    let paths = register_req.paths.iter()
        .map(|path| {
            match (path.starts_with('^'), path.ends_with('$')) {
                (true, true) => path.to_owned(),
                (true, false) => format!("{path}$"),
                (false, true) => format!("^{path}"),
                _ => format!("^{path}$"),
            }
        })
    .collect::<Vec<_>>();

    let access_data = AccessData { 
        paths, 
        servers: register_req.servers.clone()
    };

    let claim = Claims::new(access_data);
    match claim.sign(&config.key) {
        Ok(token) => HttpResponse::Ok()
            .json(TokenResponse {
                token,
                expires: claim.exp,
            }),
        Err(e) => Error::BadRequest(format!("{e}")).to_http_response(),
    }
}
