use actix_web::{dev::Server, web, App, HttpServer};
use anyhow::Context;
use routes::portfolio_state;
use sqlx::{postgres::PgConnectOptions, PgPool};
use std::net::TcpListener;
use telemetry::{get_subscriber, init_subscriber};
use tracing_actix_web::TracingLogger;

pub mod authentication;
pub mod routes;
pub mod telemetry;

const APP_NAME: &str = "mfm_server";
const DEFAULT_LOG_LEVEL: &str = "info";

pub async fn connect_pg_pool(options: PgConnectOptions) -> Result<PgPool, anyhow::Error> {
    PgPool::connect_with(options)
        .await
        .context("failed to connect in pg")
}

pub async fn connect_db_pool(database_url: String) -> Result<PgPool, anyhow::Error> {
    let options: PgConnectOptions = database_url
        .parse()
        .context("database_url cannot be parsed to PgConnectOptions")?;

    connect_pg_pool(options).await
}

pub fn database_url() -> Result<String, anyhow::Error> {
    std::env::var("DATABASE_URL").context("DATABASE_URL must be set, error")
}

pub async fn establish_db_pool() -> Result<PgPool, anyhow::Error> {
    connect_db_pool(database_url()?).await
}

pub fn start_http_server(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/portfolio_state", web::post().to(portfolio_state::handler))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn run() -> Result<Server, std::io::Error> {
    let subscriber = get_subscriber(APP_NAME.into(), DEFAULT_LOG_LEVEL.into(), std::io::stdout);
    init_subscriber(subscriber);

    let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string());

    let listener = TcpListener::bind(addr).expect("failed to bind the addr:port");
    let db_pool = establish_db_pool().await.unwrap_or_else(|e| {
        tracing::error!(error = %e);
        panic!()
    });

    start_http_server(listener, db_pool)
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn check_database_url_from_env() {
        let expected =
            "postgres://postgres:example@127.0.0.1:5445/mfmserver_development".to_string();
        env::set_var("DATABASE_URL", expected.clone());

        assert_eq!(expected, database_url().unwrap());
    }
}
