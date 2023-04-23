use std::{fmt, collections::HashMap};
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};


use actix_web::{web, get, HttpResponse, Responder, HttpRequest};
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
                let mut servers:  Vec<String> = Vec::default();
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

#[get("/servers/logs")]
async fn servers_logs(
    req: HttpRequest,
    claims: web::ReqData<crate::auth::Claims>,
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
    claims: web::ReqData<crate::auth::Claims>,
    servers_logs_req: web::Query<ServersLogsRequest>,
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
) -> Result<Vec<String>, String> {
    // TODO: Handle unwraps in this request properly.
    // TODO: Passthrough query args.
    let client = reqwest::Client::new();
    let lines = client
        .get(format!("http://{server}/v1/logs/{filename}"))
        .header(reqwest::header::AUTHORIZATION, auth_header)
        .send()
        .await
        .unwrap()
        .json::<Vec<String>>()
        .await
        .unwrap();
    Ok(lines)
}
