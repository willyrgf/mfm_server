use std::net::TcpListener;

use sqlx::PgPool;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind the random local port");
    let db_pool = mfm_server::establish_db_pool().await;

    let address = {
        let port = listener.local_addr().unwrap().port();
        format!("http://127.0.0.1:{}", port)
    };

    let server = mfm_server::start_http_server(listener, db_pool.clone())
        .expect("failed to start the http server");

    actix_web::rt::spawn(server);

    TestApp { address, db_pool }
}
