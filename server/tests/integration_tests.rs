use server;

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};
    use reqwest::StatusCode;

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(
            App::new().configure(server::init_routes),
        ).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
