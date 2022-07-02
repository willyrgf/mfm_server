use serde::{Deserialize, Serialize};
use serde_json::Value;
use warp::{self, Filter};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PortfolioState {
    token_id: uuid::Uuid,
    rebalancer_label: String,
    data: Value,
}

fn portfolio_state_json_body(
) -> impl Filter<Extract = (PortfolioState,), Error = warp::Rejection> + Clone {
    log::debug!("portfolio_state_json_body()");
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn add_portfolio_state(
    portfolio_state: PortfolioState,
) -> Result<impl warp::Reply, warp::Rejection> {
    //TODO: continue here
    let s = format!("{:?}", portfolio_state);
    Ok(warp::reply::with_status(s, warp::http::StatusCode::CREATED))
}

pub async fn get_route() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::post()
        .and(warp::path("v1"))
        .and(warp::path("portfolio_state"))
        .and(warp::path::end())
        .and(portfolio_state_json_body())
        .and_then(add_portfolio_state)
}
