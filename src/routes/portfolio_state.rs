use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PortfolioState {
    token_id: Uuid,
    rebalancer_label: String,
    data: serde_json::Value,
}

#[tracing::instrument(
    name = "adding a new portfolio_sate",
    skip(body, db_pool),
    fields(
        token_id = %body.token_id,
        rebalancer_label = %body.rebalancer_label,
        data = %body.data,
    )
)]
pub async fn handler(body: web::Json<PortfolioState>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match insert_portfolio_state(&db_pool, &body).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "saving new portfolio_state details in the database"
    skip(body, db_pool)
)]
pub async fn insert_portfolio_state(
    db_pool: &PgPool,
    body: &PortfolioState,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        insert into portfolio_states (token_id, rebalancer_label, data)
        values ($1, $2, $3)
        "#,
        body.token_id,
        body.rebalancer_label,
        body.data
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
