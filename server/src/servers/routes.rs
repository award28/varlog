use std::{fmt, collections::HashMap};

use anyhow::Result;
use serde::{de::{Deserializer, Visitor, MapAccess}, Deserialize};
use actix_web::{web, get, HttpResponse, Responder, HttpRequest};

use crate::{
    logs::routes::LogsRequest,
    auth::{
        claims::Claims,
        access_data::AccessData
    }, http::Error,
};

const SERVER_PAYLOAD_KEY: &str = "server";

#[derive(Debug)]
struct ServersLogsRequest {
    servers: Vec<String>,
}


impl<'de> Deserialize<'de> for ServersLogsRequest {
    fn deserialize<D>(deserializer: D) -> Result<ServersLogsRequest, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = ServersLogsRequest;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`id`")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ServersLogsRequest, V::Error>
            where
                V: MapAccess<'de>
            {
                let mut req_servers: Vec<String> = Vec::default();
                while let Some((key, value)) = map.next_entry()? {
                    match key {
                        SERVER_PAYLOAD_KEY => req_servers.push(value),
                        _ => ()
                    }
                }
                Ok(ServersLogsRequest {
                    servers: req_servers
                })
            }
        }
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

#[derive(Deserialize)]
struct ServersResponse {
    hostnames: Vec<String>,
}

#[get("/servers")]
async fn servers(
    config: web::Data<crate::conf::AppConfig>,
    claims: web::ReqData<Claims>,
) -> impl Responder {
    match servers_handle(
        config.registry_url.as_ref(),
        &claims.data,
    ).await {
        Ok(accessible_servers) => HttpResponse::Ok()
            .json(accessible_servers),
        Err(e) => {
            e.to_http_response()
        },
    }
}

async fn servers_handle<'a>(
    registry_url: &'a str,
    access_data: &'a AccessData,
) -> crate::http::Result<Vec<String>> {
     let res: Vec<String> = reqwest::get(
        format!("{registry_url}/registered"),
    )
        .await
        .map_err(|e| {
            error!("Registry is down: {e}.");
            crate::http::Error::ServiceUnavailable
        })?
        .json::<ServersResponse>()
        .await
        .map_err(|e| {
            error!("Could not serialize registry response: {e}.");
            crate::http::Error::ServiceUnavailable
        })?
        .hostnames
        .into_iter()
        .filter(|server| {
            access_data.server_access_authorized(server.as_str())
        })
    .collect();
    Ok(res)
}

#[get("/servers/logs")]
async fn servers_logs(
    req: HttpRequest,
    claims: web::ReqData<Claims>,
    servers_logs_req: web::Query<ServersLogsRequest>,
) -> impl Responder {
    match servers_logs_handle(
        &req,
        servers_logs_req.into_inner().servers,
        &claims.data,
    ).await {
        Ok(accessible_servers_logfiles) => HttpResponse::Ok()
            .json(accessible_servers_logfiles),
        Err(e) => {
            e.to_http_response()
        },
    }
}
async fn servers_logs_handle(
    req: &HttpRequest,
    req_servers: Vec<String>,
    access_data: &AccessData,
) -> crate::http::Result<HashMap<String, Vec<String>>> {
    validate_server_access(&req_servers, &access_data)?;
    let auth_header = get_auth_header(&req)?;

    let mut server_logfiles_map = HashMap::new();

    for server in req_servers {
        let logs = logs_from_server(
            server.as_str(),
            auth_header.as_str(),
            )
            .await;

        let logs = match logs {
            Ok(logs) => logs,
            Err(_) => vec![format!("Could not find log file.")],
        };
        server_logfiles_map.insert(server, logs);
    }
    Ok(server_logfiles_map)
}

#[get("/servers/logs/{filename:.*}")]
async fn servers_log(
    req: HttpRequest,
    path: web::Path<(String,)>,
    claims: web::ReqData<Claims>,
    servers_logs_req: web::Query<ServersLogsRequest>,
    logs_req: web::Query<crate::logs::routes::LogsRequest>,
) -> impl Responder {
    let (filename,) = path.into_inner();
    match servers_log_handle(
        &req,
        filename.as_str(),
        servers_logs_req.into_inner().servers,
        &claims.data,
        &logs_req.into_inner(),
    ).await {
        Ok(accessible_servers_logs) => HttpResponse::Ok()
            .json(accessible_servers_logs),
        Err(e) => {
            e.to_http_response()
        },
    }
}

async fn servers_log_handle(
    req: &HttpRequest,
    filename: &str,
    req_servers: Vec<String>,
    access_data: &AccessData,
    logs_req: &crate::logs::routes::LogsRequest,
) -> crate::http::Result<HashMap<String, Vec<String>>> {
    validate_server_access(&req_servers, &access_data)?;
    let auth_header = get_auth_header(&req)?;

    let mut server_logs_map = HashMap::new();

    for server in req_servers {
        let logs = log_from_server(
            server.as_str(),
            filename,
            auth_header.as_str(),
            logs_req,
            )
            .await;

        let logs = match logs {
            Ok(logs) => logs,
            Err(_) => vec![format!("Could not find log file.")],
        };
        server_logs_map.insert(server, logs);
    }

    Ok(server_logs_map)
}

fn validate_server_access(
    req_servers: &Vec<String>,
    access_data: &AccessData,
) -> crate::http::Result<()> {
    if let Some(server) = req_servers.iter().filter(|server| {
        !access_data.server_access_authorized(server)
    }).next() {
        return Err(crate::http::Error::Forbidden(
            format!("You do not have access to server `{server}`.")
        ));
    }
    Ok(())
}

fn get_auth_header<'a>(req: &'a HttpRequest) -> crate::http::Result<String> {
    let header = req.headers().get("Authorization").ok_or(
        Error::BadRequest(
            format!("Authorization header is required for this endpoint.")
        )
    )?.to_str().map_err(|_|
        Error::BadRequest(
            format!("Authorization header must only contain ASCII characters.")
        )
    )?;
    Ok(header.to_owned())
}

async fn logs_from_server(
    server: &str,
    auth_header: &str,
) -> Result<Vec<String>, String> {
    let logs = reqwest::Client::new()
        .get(format!("http://{server}/v1/logs"))
        .header(reqwest::header::AUTHORIZATION, auth_header)
        .send()
        .await
        .map_err(|e| {
            let msg = format!("Error retrieving log files from server: {e}.");
            warn!("{}", msg);
            msg
        })?
        .json::<Vec<String>>()
        .await
        .map_err(|e| {
            let msg = format!("Error deserializing log files: {e}.");
            info!("{}", msg);
            msg
        })?;
    Ok(logs)
}


async fn log_from_server(
    server: &str,
    filename: &str,
    auth_header: &str,
    logs_req: &LogsRequest,
) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
   let pat = logs_req.pattern.to_owned().unwrap_or(String::new());

    let logs = client
        .get(format!("http://{server}/v1/logs/{filename}"))
        .header(reqwest::header::AUTHORIZATION, auth_header)
        .query(&[
            ("pattern", pat),
            ("skip", format!("{}", logs_req.skip.unwrap_or(0))),
            ("take", format!("{}", logs_req.take.unwrap_or(10)))
        ])
        .send()
        .await
        .map_err(|e| {
            let msg = format!("Error retrieving log: {e}.");
            warn!("{}", msg);
            msg
        })?
        .json::<Vec<String>>()
        .await
        .map_err(|e| {
            let msg = format!("Error deserializing log: {e}.");
            info!("{}", msg);
            msg
        })?;
    Ok(logs)
}
