use actix_web::{web, get, post, middleware, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;

mod auth;

#[post("/auth/register")]
async fn register(
    access_data: web::Json<auth::AccessData>,
) -> impl Responder {
    let copy_data = auth::AccessData{
        paths: access_data.paths.to_owned(),
        servers: access_data.servers.to_owned(),
    };
    let token_str = auth::Claims::signed(access_data.into_inner()).unwrap();

    HttpResponse::Ok().body(
        format!(
            "Claim: {:?}\nToken: {token_str}",
            auth::Claims::new(copy_data),
        )
    )
}

#[get("/logs")]
async fn log(claims: web::ReqData<auth::Claims>) -> impl Responder {
    HttpResponse::Ok().body(format!("{:#?}", claims.data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
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
