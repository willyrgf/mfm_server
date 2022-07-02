use env_logger::Env;
use mfm_server::start_http_server;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    start_http_server().await
}
