use hmac::{Hmac, Mac};
use server;
use actix_web::{test, App, web, http::header};
use reqwest::StatusCode;

use std::{collections::HashMap, fs::File, io::Write};

use cucumber::{given, when, then, World};
use serde_json::Value;

#[derive(Debug, Default, World)]
pub struct VarlogWorld {
    status_code: StatusCode,
    token: String,
    req_data: HashMap<String, Value>,
    res_data: Value,
}

#[given(
    regex = "there is a log file named '([^']*)' with content '([^']*)'"
)]
async fn create_log_file(
    _world: &mut VarlogWorld,
    filename: String,
    content: String
) {
    let mut file = File::create(
        format!("/tmp/{filename}"),
    ).unwrap();
    file.write(content.as_bytes()).unwrap();
}

#[given(
    expr = "I have path access {word} and server access {word}"
)]
async fn have_access(
    world: &mut VarlogWorld,
    paths: String,
    servers: String,
    ) {
    let paths = vec![paths];
    let servers = vec![servers];
    let mut req_data: HashMap<String, Value> = Default::default();
    req_data.insert("paths".to_string(), Value::from(paths));
    req_data.insert("servers".to_string(), Value::from(servers));

    let key = Hmac::new_from_slice(
            "testing-key".as_bytes(),
    ).unwrap();

    let config = server::conf::AppConfig {
        key,
        registry_url: "".to_string(),
        log_dir: "/tmp".to_string(),
    };
    let mut app = test::init_service(
        App::new()
        .app_data(web::Data::new(config))
        .configure(server::init_routes),
    ).await;
    let resp = test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(req_data.clone())
        .send_request(&mut app)
        .await;

    let hm: HashMap<String, Value> = test::read_body_json(resp).await;
    world.token = serde_json::from_value(
        hm.get("token").unwrap().to_owned(),
    ).unwrap();
}

#[given(
    expr = "I need path access {word} and server access {word}"
)]
async fn need_access(
    world: &mut VarlogWorld,
    paths: String,
    servers: String,
    ) {
    let paths = vec![paths];
    let servers = vec![servers];
    world.req_data.insert("paths".to_string(), Value::from(paths));
    world.req_data.insert("servers".to_string(), Value::from(servers));
}

#[when("I request access")]
async fn request_access(world: &mut VarlogWorld) {
    let key = Hmac::new_from_slice(
            "testing-key".as_bytes(),
    ).unwrap();

    let config = server::conf::AppConfig {
        key,
        registry_url: "".to_string(),
        log_dir: "/tmp".to_string(),
    };
    let mut app = test::init_service(
        App::new()
        .app_data(web::Data::new(config))
        .configure(server::init_routes),
    ).await;
    let resp = test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(world.req_data.clone())
        .send_request(&mut app)
        .await;
    world.status_code = resp.status();

    world.res_data = test::read_body_json(resp).await;
}


#[when("I request logs")]
async fn request_logs(world: &mut VarlogWorld) {
    let key = Hmac::new_from_slice(
            "testing-key".as_bytes(),
    ).unwrap();

    let config = server::conf::AppConfig {
        key,
        registry_url: "".to_string(),
        log_dir: "/tmp".to_string(),
    };
    let mut app = test::init_service(
        App::new()
        .app_data(web::Data::new(config))
        .configure(server::init_routes),
    ).await;

    let token = format!("Bearer {}", world.token);
    let resp = test::TestRequest::get()
        .uri("/v1/logs")
        .insert_header(
            (header::AUTHORIZATION, token),
        )
        .send_request(&mut app)
        .await;
    world.status_code = resp.status();
    world.res_data = test::read_body_json(resp).await;
}

#[when(regex = "I request the contents of '([^']*)'")]
async fn request_logs_content(
    world: &mut VarlogWorld,
    logfile: String,
) {
    let key = Hmac::new_from_slice(
            "testing-key".as_bytes(),
    ).unwrap();

    let config = server::conf::AppConfig {
        key,
        registry_url: "".to_string(),
        log_dir: "/tmp".to_string(),
    };
    let mut app = test::init_service(
        App::new()
        .app_data(web::Data::new(config))
        .configure(server::init_routes),
    ).await;

    let token = format!("Bearer {}", world.token);
    let resp = test::TestRequest::get()
        .uri(format!("/v1/logs/{}", logfile).as_str())
        .insert_header(
            (header::AUTHORIZATION, token),
        )
        .send_request(&mut app)
        .await;
    world.status_code = resp.status();
    world.res_data = test::read_body_json(resp).await;
}

#[then(expr = "I receive status code {int}")]
async fn compare_status_code(world: &mut VarlogWorld, value: u16) {
    assert_eq!(
        world.status_code,
        StatusCode::from_u16(value).unwrap(),
    );
}

#[then(regex = "I receive an? (\\w+)")]
async fn contains_key(world: &mut VarlogWorld, key: String) {
    let hm: HashMap<String, Value> = serde_json::from_value(
        world.res_data.to_owned(),
    ).unwrap();
    assert!(hm.contains_key(&key));
}

#[then(regex = "the (\\w+) contains '([^']*)'")]
async fn contains_substr(
    world: &mut VarlogWorld,
    key: String,
    substr: String,
) {
    let hm: HashMap<String, Value> = serde_json::from_value(
        world.res_data.to_owned(),
    ).unwrap();

    let value: String = serde_json::from_value(
        hm.get(&key).unwrap().to_owned(),
    ).unwrap();
    assert!(value.contains(&substr));
}

#[then(regex = "the logs list contains '([^']*)'")]
async fn contains_item(
    world: &mut VarlogWorld,
    item: String,
) {
    let list: Vec<String> = serde_json::from_value(
        world.res_data.to_owned(),
    ).unwrap();

    assert!(list.contains(&item));
}

#[then(regex = "the logs list does not contain '([^']*)'")]
async fn not_contains_item(
    world: &mut VarlogWorld,
    item: String,
) {
    let list: Vec<String> = serde_json::from_value(
        world.res_data.to_owned(),
    ).unwrap();

    assert!(!list.contains(&item));
}

#[tokio::main]
async fn main() {
    VarlogWorld::run("tests/features").await;
}
