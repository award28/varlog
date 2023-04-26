use std::{fmt, collections::HashMap};
use serde::{de::{Deserializer, Visitor, MapAccess}, Deserialize};


use actix_web::{web, get, HttpResponse, Responder, HttpRequest};

use crate::{logs::routes::LogsRequest, auth::claims::Claims};

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
                let mut servers: Vec<String> = Vec::default();
                while let Some(key) = map.next_key()? {
                    match key {
                        "server" => {
                            servers.push(map.next_value::<String>()?)
                        }
                        _ => ()
                    }
                }
                Ok(ServersLogsRequest {
                    servers
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
async fn available_servers(
    config: web::Data<crate::conf::AppConfig>,
    claims: web::ReqData<Claims>,
) -> impl Responder {
    let registry_url = (*config.registry_url).to_owned();
    let res: Vec<String> = reqwest::get(format!("{registry_url}/registered"))
        .await
        .unwrap()
        .json::<ServersResponse>()
        .await
        .unwrap()
        .hostnames
        .into_iter()
        .filter(|server| {
            claims.data.server_access_authorized(server.as_str())
        })
        .collect();
    HttpResponse::Ok().json(res)
}


#[get("/servers/logs")]
async fn servers_logs(
    req: HttpRequest,
    claims: web::ReqData<Claims>,
    servers_logs_req: web::Query<ServersLogsRequest>,
) -> impl Responder {
    if let Some(server) = servers_logs_req.servers.iter().filter(|server| {
        !claims.data.server_access_authorized(server)
    }).next() {
        return HttpResponse::Forbidden().json(
            crate::http::HttpError {
                error: format!("Unauthorized. You can not access server `{server}`."),
            }
        )
    }

    let auth_header = get_auth_header(&req)
        .expect("Authorization header is required for this endpoint.");

    let mut server_logs_map = HashMap::new();

    for server in servers_logs_req.servers.to_owned() {
        let logs = logs_from_server(
            server.as_str(),
            auth_header,
            ).await.unwrap();
        server_logs_map.insert(server, logs);
    }

    // TODO: Gather available log files from servers using reqwest
    HttpResponse::Ok().json(
        server_logs_map,
    )
}

fn get_auth_header<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Authorization")?.to_str().ok()
}

async fn logs_from_server(
    server: &str,
    auth_header: &str,
) -> Result<Vec<String>, String> {
    // TODO: Handle unwraps in this request properly.
    let client = reqwest::Client::new();
    let logs = client
        .get(format!("http://{server}/v1/logs"))
        .header(reqwest::header::AUTHORIZATION, auth_header)
        .send()
        .await
        .unwrap()
        .json::<Vec<String>>()
        .await
        .unwrap();
    Ok(logs)
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

    if let Some(server) = servers_logs_req.servers.iter().filter(|server| {
        !claims.data.server_access_authorized(server)
    }).next() {
        return HttpResponse::Forbidden().json(
            crate::http::HttpError {
                error: format!("Unauthorized. You can not access server `{server}`."),
            }
        )
    }

    let auth_header = get_auth_header(&req)
        .expect("Authorization header is required for this endpoint.");

    let mut server_logs_map = HashMap::new();

    for server in servers_logs_req.servers.to_owned() {
        let logs = log_from_server(
            server.as_str(),
            filename.as_str(),
            auth_header,
            &logs_req,
            ).await.unwrap();
        server_logs_map.insert(server, logs);
    }

    // TODO: Gather available log files from servers using reqwest
    HttpResponse::Ok().json(
        server_logs_map,
    )
}

async fn log_from_server(
    server: &str,
    filename: &str,
    auth_header: &str,
    logs_req: &LogsRequest,
) -> Result<Vec<String>, String> {
    // TODO: Handle unwraps in this request properly.
    // TODO: Passthrough query args.
    let client = reqwest::Client::new();
   let pat = logs_req.pattern.to_owned().unwrap_or(String::new());

    let lines = client
        .get(format!("http://{server}/v1/logs/{filename}"))
        .header(reqwest::header::AUTHORIZATION, auth_header)
        .query(&[
            ("pattern", pat),
            ("skip", format!("{}", logs_req.skip.unwrap_or(0))),
            ("take", format!("{}", logs_req.take.unwrap_or(10)))
        ])
        .send()
        .await
        .unwrap()
        .json::<Vec<String>>()
        .await
        .unwrap();
    Ok(lines)
}
