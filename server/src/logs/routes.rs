use std::{fs, path::Path, io};

use actix_web::{web, get, HttpResponse, Responder};
use serde::Deserialize;

use crate::auth::claims::Claims;

#[derive(Debug, Deserialize)]
pub struct LogsRequest {
    pub take: Option<usize>,
    pub skip: Option<usize>,
    pub pattern: Option<String>,
}

#[get("/logs")]
async fn logs(
    claims: web::ReqData<Claims>,
) -> impl Responder {
    let paths_res = visit_dirs(Path::new("/var/log"));
    if paths_res.is_err() {
        return crate::http::internal_err();
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
    claims: web::ReqData<Claims>,
    path: web::Path<(String,)>,
    logs_req: web::Query<LogsRequest>,
) -> impl Responder {
    let (filename,) = path.into_inner();

    if filename.contains("..") {
        return HttpResponse::Forbidden().json(
            crate::http::HttpError {
                error: String::from("The use of the `..` operator is forbidden."),
            }
        );
    }

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

            HttpResponse::Ok().json(
                lines,
            )
        },
        false => HttpResponse::Forbidden().json(
            crate::http::HttpError {
                error: format!("You do not have access to file {filename}"),
            }
        ),
    }
}
