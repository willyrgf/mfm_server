use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use sqlx::{postgres::PgConnectOptions, PgPool};

pub mod portfolio_state;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn start_http_server(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/portfolio_state", web::post().to(portfolio_state::handler))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn connect_pg_pool(options: PgConnectOptions) -> PgPool {
    PgPool::connect_with(options).await.unwrap_or_else(|e| {
        log::error!(
            "connect_pg_pool(): PgPool::connect() ConnectionError: {}",
            e
        );
        panic!()
    })
}

pub async fn connect_db_pool(database_url: String) -> PgPool {
    let options: PgConnectOptions = database_url.parse().unwrap_or_else(|e| {
        log::error!(
            "connect_db_pool(): database_url cannot be parsed to PgConnectOptions, error: {}",
            e
        );
        panic!()
    });
    connect_pg_pool(options).await
}

pub fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|e| {
        log::error!("database_url(): DATABASE_URL must be set, error: {}", e);
        panic!()
    })
}

pub async fn establish_db_pool() -> PgPool {
    connect_db_pool(database_url()).await
}

pub async fn run() -> Result<Server, std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string());

    let listener = TcpListener::bind(addr).expect("failed to bind the addr:port");
    let db_pool = establish_db_pool().await;

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

        assert_eq!(expected, database_url());
    }
}
