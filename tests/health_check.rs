use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind the random local port");
    let port = listener.local_addr().unwrap().port();
    let server = mfm_server::start_http_server(listener).expect("failed to start the http server");

    actix_web::rt::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[actix_web::test]
async fn health_check_works() {
    let addr = spawn_app();

    let response = {
        let client = reqwest::Client::new();

        client
            .get(&format!("{}/health_check", addr))
            .send()
            .await
            .expect("failed to execute the request")
    };

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
