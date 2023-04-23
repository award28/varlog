use actix_web::{web, post, HttpResponse, Responder};
use regex::RegexSet;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct AuthRequest {
    #[validate(length(min = 1), custom = "validate_paths")]
    pub paths: Vec<String>,
    pub servers: Vec<String>,
}

fn validate_paths(paths: &Vec<String>) -> Result<(), ValidationError> {
    if paths.iter().any(|path| path.contains("..")) {
        return Err(ValidationError::new(
                "Parent directory access is forbidden."
        ));
    }

    // Performs validation that the regex is properly formatted.
    RegexSet::new(paths.clone())
        .map_err(|_| ValidationError::new(
            "Invalid file paths"
        )
    )?;

    Ok(())
}
use serde::Serialize;

#[derive(Serialize)]
struct TokenResponse {
    token: String,
    expires: i64,
}

#[post("/auth/register")]
async fn register(
    config: web::Data<crate::AppConfig>,
    auth_data: web::Json<AuthRequest>,
) -> impl Responder {
    if let Err(e) = auth_data.validate() {
        return HttpResponse::BadRequest().json(
            crate::http::HttpError {
                error: format!("{e}"),
            }
        );
    }

    let paths = auth_data.paths.iter()
        .map(|path| {
            match (path.starts_with('^'), path.ends_with('$')) {
                (true, true) => path.to_owned(),
                (true, false) => format!("{path}$"),
                (false, true) => format!("^{path}"),
                _ => format!("^{path}$"),
            }
        })
    .collect::<Vec<_>>();

    let access_data = crate::auth::AccessData { 
        paths, 
        servers: auth_data.servers.clone()
    };

    match crate::auth::Claims::sign(&config.key, access_data) {
        Ok((claim, token)) => HttpResponse::Ok().json(TokenResponse {
            token,
            expires: claim.exp,
        }),
        Err(e) => HttpResponse::BadRequest().json(
            crate::http::HttpError {
                error: format!("{e}"),
            }
        ),
    }
}
