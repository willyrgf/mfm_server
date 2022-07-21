use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PortfolioState {
    token_id: uuid::Uuid,
    rebalancer_label: String,
    data: serde_json::Value,
}

pub async fn handler(body: web::Json<PortfolioState>, db_pool: web::Data<PgPool>) -> HttpResponse {
    log::debug!("portofolio_state(): body: {:?}", body);

    match sqlx::query!(
        r#"
        insert into portfolio_states (token_id, rebalancer_label, data)
        values ($1, $2, $3)
        "#,
        body.token_id,
        body.rebalancer_label,
        body.data
    )
    .execute(db_pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            log::error!("failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
