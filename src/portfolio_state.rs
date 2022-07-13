// use diesel::prelude::*;
// // use diesel_citext::types::CiString;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use warp::{self, Filter};

// use super::schema::portfolio_states;
// use crate::establish_connection;

// // #[derive(Deserialize, Serialize, Clone, Debug, Insertable)]
// // #[table_name = "portfolio_states"]
// pub struct PortfolioState {
//     token_id: uuid::Uuid,
//     rebalancer_label: String,
//     data: Value,
// }

// fn portfolio_state_json_body(
// ) -> impl Filter<Extract = (PortfolioState,), Error = warp::Rejection> + Clone {
//     log::debug!("portfolio_state_json_body()");
//     warp::body::content_length_limit(1024 * 16).and(warp::body::json())
// }

// pub async fn add_portfolio_state(
//     portfolio_state: PortfolioState,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let s = format!("{:?}", portfolio_state);

//     let conn = establish_connection();
//     let r = diesel::insert_into(crate::schema::portfolio_states::table)
//         .values(portfolio_state)
//         .get_result(conn)
//         .unwrap();

//     Ok(warp::reply::with_status(s, warp::http::StatusCode::CREATED))
// }

// pub async fn get_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
// {
//     warp::post()
//         .and(warp::path("v1"))
//         .and(warp::path("portfolio_state"))
//         .and(warp::path::end())
//         .and(portfolio_state_json_body())
//         .and_then(add_portfolio_state)
// }
