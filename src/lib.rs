pub mod portfolio_state;

pub async fn start_http_server() {
    let routes = portfolio_state::get_route().await;
    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
