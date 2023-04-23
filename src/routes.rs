use std::{fs, path::Path, io};

use actix_web::{web, get, post, HttpResponse, Responder};
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
    config: web::Data<crate::AppConfig>,
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

    let access_data = crate::auth::AccessData { 
        paths, 
        servers: auth_data.servers.clone()
    };

    // TODO: Respond with JSON Message
    match crate::auth::Claims::sign(&config.key, access_data) {
        Ok(token_str) => HttpResponse::Ok().body(token_str),
        Err(e) => HttpResponse::BadRequest().body(format!("{e}")),
    }
}

#[derive(Debug, Deserialize)]
struct LogsRequest {
    take: Option<usize>,
    skip: Option<usize>,
    pattern: Option<String>,
}

#[get("/logs")]
async fn logs(
    claims: web::ReqData<crate::auth::Claims>,
) -> impl Responder {
    let paths_res = visit_dirs(Path::new("/var/log"));
    if paths_res.is_err() {
        return HttpResponse::InternalServerError().body(
            "Internal Server Error",
        );
    }
    let files = paths_res.unwrap();
    let files = files
        .iter()
        .filter_map(|filename| {
            match filename.strip_prefix("/var/log/") {
                Some(filename) => Some(filename.to_owned()),
                None => None,
            }
        })
        .filter(|filename| claims.data.file_access_authorized(filename))
        .collect::<Vec<_>>();

    HttpResponse::Ok().body(
        format!("{:#?}", files),
    )
}

fn visit_dirs(dir: &Path) -> io::Result<Vec<String>> {
    let mut filenames = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                filenames.append(&mut visit_dirs(&path)?);
            } else {
                filenames.push(format!("{}", path.display()));
            }
        }
    }
    Ok(filenames)
}

#[get("/logs/{filename}")]
async fn log(
    claims: web::ReqData<crate::auth::Claims>,
    path: web::Path<(String,)>,
    logs_req: web::Query<LogsRequest>,
) -> impl Responder {
    let (filename,) = path.into_inner();
    // TODO: Make sure path exists and is a file.
    match claims.data.file_access_authorized(&filename) {
        true => {
            let filename = if filename.starts_with("/var/log") {
                filename.to_owned()
            } else {
                format!("/var/log/{filename}")
            };

            let pat = logs_req.pattern.to_owned().unwrap_or(String::new());
            let lines = crate::logs::RevLogReader::new(filename)
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
            format!("You do not have access to file {}", filename),
        ),
    }
}
