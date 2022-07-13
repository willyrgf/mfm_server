#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::{Connection, PgConnection};

pub mod portfolio_state;
pub mod schema;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn start_http_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await
}

pub fn establish_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|e| {
        log::error!(
            "establish_connection(): DATABASE_URL must be set, error: {}",
            e
        );
        panic!()
    });

    PgConnection::establish(&database_url).unwrap_or_else(|e| {
        log::error!(
            "establish_connection(): PgConnection::establish() ConnectionError: {}",
            e
        );
        panic!()
    })
}
