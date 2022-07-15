use std::net::TcpListener;

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind the random local port");
    let port = listener.local_addr().unwrap().port();
    let server = mfm_server::start_http_server(listener).expect("failed to start the http server");

    actix_web::rt::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
