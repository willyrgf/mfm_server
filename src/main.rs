#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    mfm_server::run().await?.await
}
