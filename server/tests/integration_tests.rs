use hmac::{Hmac, Mac};
use server;
use actix_web::{test, App, web};
use reqwest::StatusCode;

use std::collections::HashMap;

use cucumber::{given, when, then, World};
use serde_json::Value;

#[derive(Debug, Default, World)]
pub struct AuthWorld {
    req_data: HashMap<String, Value>,
    status_code: StatusCode,
    res_data: HashMap<String, Value>,
}

#[given(
    expr = "I need path access {word} and server access {word}"
)]
async fn need_access(
    world: &mut AuthWorld,
    paths: String,
    servers: String,
    ) {
    let paths = vec![paths];
    let servers = vec![servers];
    world.req_data.insert("paths".to_string(), Value::from(paths));
    world.req_data.insert("servers".to_string(), Value::from(servers));
}

#[when("I request access")]
async fn request_access(world: &mut AuthWorld) {
    let key = Hmac::new_from_slice(
            "testing-key".as_bytes(),
    ).unwrap();

    let config = server::conf::AppConfig {
        key,
        registry_url: "".to_string(),
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

    //serde_json::from_slice
    world.res_data = test::read_body_json(resp).await;
}

#[then(expr = "I receive status code {int}")]
async fn compare_status_code(world: &mut AuthWorld, value: u16) {
    assert_eq!(
        world.status_code,
        StatusCode::from_u16(value).unwrap(),
    );
}

#[then(expr = "I receive a {word}")]
async fn contains_key(world: &mut AuthWorld, key: String) {
    assert!(world.res_data.contains_key(&key));
}

#[tokio::main]
async fn main() {
    AuthWorld::run("tests/features").await;
}
