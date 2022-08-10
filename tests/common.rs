use anyhow::Context;
use mfm_server::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{postgres::PgConnectOptions, Executor, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

const APP_NAME: &str = "integration_test";
const DEFAULT_LOG_LEVEL: &str = "debug";

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber(APP_NAME.into(), DEFAULT_LOG_LEVEL.into(), std::io::stdout);
    init_subscriber(subscriber);
});

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

impl TestApp {
    #[tracing::instrument(name = "cleaning up the test app")]
    pub async fn cleanup(&self) -> Result<(), anyhow::Error> {
        cleaning_db_with_isolation(&self.db_pool).await
    }
}

#[tracing::instrument(name = "cleaning up the isolated db")]
async fn cleaning_db_with_isolation(db_pool: &PgPool) -> Result<(), anyhow::Error> {
    let database_name = db_pool
        .connect_options()
        .get_database()
        .context("database name is not configured")?;

    db_pool.close().await;

    let setup_db_pool = mfm_server::establish_db_pool().await;

    setup_db_pool
        .execute(format!(r#"drop database "{}";"#, database_name).as_str())
        .await
        .context("failed on drop database")?;

    Ok(())
}

#[tracing::instrument(name = "setup a new db connection with isolation")]
async fn new_db_with_isolation() -> Result<PgPool, anyhow::Error> {
    let setup_db_pool = mfm_server::establish_db_pool().await;
    let database_name = Uuid::new_v4().to_string();

    let mut options: PgConnectOptions = mfm_server::database_url().parse().unwrap();
    options = options.database(&database_name);

    setup_db_pool
        .execute(format!(r#"create database "{}";"#, database_name).as_str())
        .await
        .context("failed on create new database")?;

    let db_pool = mfm_server::connect_pg_pool(options).await;

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .context("failed to migrate the new database")?;

    Ok(db_pool)
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind the random local port");
    let db_pool = new_db_with_isolation().await.unwrap();

    let address = {
        let port = listener.local_addr().unwrap().port();
        format!("http://127.0.0.1:{}", port)
    };

    let server = mfm_server::start_http_server(listener, db_pool.clone())
        .expect("failed to start the http server");

    actix_web::rt::spawn(server);

    TestApp { address, db_pool }
}
