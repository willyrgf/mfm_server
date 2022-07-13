use env_logger::Env;
use mfm_server::start_http_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    start_http_server().await
}
