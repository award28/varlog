use std::env;

use actix_web::{web, get, post, middleware, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use hmac::{Hmac, Mac};
use regex::RegexSet;
use serde::Deserialize;
use sha2::Sha256;
use validator::{Validate, ValidationError};

mod auth;
mod logs;

#[derive(Debug, Validate, Deserialize)]
pub struct AuthRequest {
    #[validate(length(min = 1), custom = "validate_paths")]
    pub paths: Vec<String>,
    pub servers: Vec<String>,
}

fn validate_paths(paths: &Vec<String>) -> Result<(), ValidationError> {
    if paths.iter().any(|path| path.contains("..")) {
        return Err(ValidationError::new("Parent directory access is forbidden."));
    }

    // Performs validation that the regex is properly formatted.
    RegexSet::new(paths.clone())
        .map_err(|_| ValidationError::new(
            "Invalid file paths"
        )
    )?;


    Ok(())
}

#[post("/auth/register")]
async fn register(
    config: web::Data<AppConfig>,
    auth_data: web::Json<AuthRequest>,
) -> impl Responder {
    if let Err(e) = auth_data.validate() {
        return HttpResponse::BadRequest().body(format!("{e}"));
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

    let access_data = auth::AccessData { 
        paths, 
        servers: auth_data.servers.clone()
    };

    // TODO: Respond with JSON Message
    match auth::Claims::sign(&config.key, access_data) {
        Ok(token_str) => HttpResponse::Ok().body(token_str),
        Err(e) => HttpResponse::BadRequest().body(format!("{e}")),
    }
}

#[derive(Debug, Deserialize)]
struct LogsRequest {
    filename: String,
    take: Option<usize>,
    skip: Option<usize>,
    pattern: Option<String>,
}

#[get("/logs")]
async fn log(
    claims: web::ReqData<auth::Claims>,
    logs_req: web::Query<LogsRequest>,
) -> impl Responder {
    match claims.data.file_access_authorized(&logs_req.filename) {
        true => {
            let filename = if logs_req.filename.starts_with("/var/log") {
                logs_req.filename.to_owned()
            } else {
                format!("/var/log/{}", logs_req.filename)
            };

            let pat = logs_req.pattern.to_owned().unwrap_or(String::new());
            let lines = logs::RevLogReader::new(filename)
                .iter()
                .filter(|line| line.contains(pat.as_str()))
                .skip(logs_req.skip.unwrap_or(0))
                .take(logs_req.take.unwrap_or(10))
                .collect::<Vec<_>>();

            HttpResponse::Ok().body(
                lines.join("\n"),
            )
        },
        false => HttpResponse::Forbidden().body(
            format!("You do not have access to file {}", logs_req.filename),
        ),
    }
}

#[derive(Clone)]
struct AppConfig {
    key: Hmac<Sha256>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let key = Hmac::new_from_slice(
            env::var("JWT_SIGNING_KEY")
            .expect("JWT Signing Key to be found in the environment.")
            .as_bytes(),
        ).expect("Key should be parsable");

        App::new()
            .app_data(web::Data::new(AppConfig{ key }))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("v1")
                .service(register)
                .service(
                    web::scope("")
                    .wrap(auth::middleware::AuthRequired::default())
                    .service(log)
                )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
