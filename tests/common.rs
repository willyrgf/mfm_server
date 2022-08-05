use std::net::TcpListener;

use mfm_server::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::PgPool;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

const APP_NAME: &str = "integration_test";
const DEFAULT_LOG_LEVEL: &str = "debug";

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber(APP_NAME.into(), DEFAULT_LOG_LEVEL.into(), std::io::stdout);
    init_subscriber(subscriber);
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind the random local port");
    let db_pool = mfm_server::establish_db_pool().await;

    let address = {
        let port = listener.local_addr().unwrap().port();
        format!("http://127.0.0.1:{}", port)
    };

    let server = mfm_server::start_http_server(listener, db_pool.clone())
        .expect("failed to start the http server");

    actix_web::rt::spawn(server);

    //TODO: add a cleanup in the db server
    TestApp { address, db_pool }
}
