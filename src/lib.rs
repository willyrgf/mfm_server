#[macro_use]
extern crate diesel;

use diesel::{Connection, PgConnection};

pub mod portfolio_state;
pub mod schema;

pub async fn start_http_server() {
    let routes = portfolio_state::get_route().await;
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
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
