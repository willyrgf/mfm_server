fn spawn_app() {
    let server = mfm_server::start_http_server().expect("failed to start the http server");
    actix_web::rt::spawn(server);
}

#[actix_web::test]
async fn health_check_works() {
    spawn_app();

    let response = {
        let client = reqwest::Client::new();

        client
            .get("http://127.0.0.1:8000/health_check")
            .send()
            .await
            .expect("failed to execute the request")
    };

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
