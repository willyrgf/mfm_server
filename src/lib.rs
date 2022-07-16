#[macro_use]
extern crate diesel;

use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use sqlx::{Connection, PgConnection};

pub mod portfolio_state;
pub mod schema;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn start_http_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/portfolio_state", web::post().to(portfolio_state::handler))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub async fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
        log::error!(
            "establish_connection(): DATABASE_URL must be set, error: {}",
            e
        );
        panic!()
    });

    PgConnection::connect(&database_url)
        .await
        .unwrap_or_else(|e| {
            log::error!(
                "establish_connection(): PgConnection::establish() ConnectionError: {}",
                e
            );
            panic!()
        })
}

pub fn run() -> Result<Server, std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let addr = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string());

    let listener = TcpListener::bind(addr).expect("failed to bind the addr:port");

    start_http_server(listener)
}
