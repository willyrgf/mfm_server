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
    let request_id = uuid::Uuid::new_v4();
    tracing::info!(
        "portofolio_state::handler(): request_id: {}, body: {:?}",
        request_id,
        body
    );

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
        Ok(v) => {
            tracing::info!(
                "portofolio_state::handler(): request_id: {}, portfolio_state saved, result: {:?}",
                request_id,
                v
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "portofolio_state::handler(): request_id: {}, failed to execute query: {}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
