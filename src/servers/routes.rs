use std::fmt;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};


use actix_web::{web, get, HttpResponse, Responder};

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
    claims: web::ReqData<crate::auth::Claims>,
    servers_logs_req: web::Query<ServersLogsRequest>,
) -> impl Responder {
    if let Some(server) = servers_logs_req.servers.iter().filter(|server| {
        !claims.data.server_access_authorized(server)
    }).next() {
        return HttpResponse::Forbidden().body(
            format!("Unauthorized access to server `{server}`.")
        )
    }

    // TODO: Gather available log files from servers using reqwest
    HttpResponse::Ok().body(
        format!("{:#?}", servers_logs_req),
    )
}
