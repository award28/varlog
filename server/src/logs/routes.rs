use std::{fs, path::Path, io};

use actix_web::{web, get, HttpResponse, Responder};
use serde::Deserialize;

use crate::{auth::claims::Claims, http::Error, conf};
use super::rev_log_reader::RevLogReader;

#[derive(Debug, Deserialize)]
pub struct LogsRequest {
    pub take: Option<usize>,
    pub skip: Option<usize>,
    pub pattern: Option<String>,
}

#[get("/logs")]
async fn logs(
    config: web::Data<conf::AppConfig>,
    claims: web::ReqData<Claims>,
) -> impl Responder {
    let log_dir = config.log_dir.clone();
    let paths_res = visit_dirs(Path::new(log_dir.as_str()));
    let files = {
        match paths_res {
            Ok(files) => files,
            Err(e) => {
                error!("Error walking {} dir: {e}", log_dir);
                return Error::InternalServiceError.to_http_response();
            },
        }
    };
    let files = files
        .iter()
        .filter_map(|filename| {
            match filename.strip_prefix(log_dir.as_str()) {
                Some(filename) => Some(filename.to_owned()),
                None => None,
            }
        })
        .filter(|filename| {
            claims.data.file_access_authorized(filename)
        })
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(files)
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

#[get("/logs/{filename:.*}")]
async fn log(
    config: web::Data<conf::AppConfig>,
    claims: web::ReqData<Claims>,
    path: web::Path<(String,)>,
    logs_req: web::Query<LogsRequest>,
) -> impl Responder {
    let (filename,) = path.into_inner();

    if filename.contains("..") {
        return Error::Forbidden(
            "The `..` operator is forbidden.".to_owned(),
        ).to_http_response();
    }

    if !claims.data.file_access_authorized(&filename) {
        return Error::Forbidden(
            format!("You do not have access to file {filename}"),
        ).to_http_response();
    }

    let log_dir = config.log_dir.clone();
    let filename = if filename.starts_with(log_dir.as_str()) {
        filename.to_owned()
    } else {
        format!("{log_dir}/{filename}")
    };

    if !Path::new(&filename).is_file() {
        return Error::BadRequest(
            format!("{filename} does not exist."),
        ).to_http_response();
    }

    let pat = logs_req.pattern.to_owned()
        .unwrap_or(String::new());

    let mut rev_log_reader = {
        match RevLogReader::new(filename) {
            Ok(r) => r,
            Err(e) => {
                error!("Error reading from log file: {e}");
                return Error::InternalServiceError.to_http_response();
            },
        }
    };
    let lines = rev_log_reader
        .iter()
        .filter(|line| line.contains(pat.as_str()))
        .skip(logs_req.skip.unwrap_or(0))
        .take(logs_req.take.unwrap_or(10))
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(
        lines,
    )
}
