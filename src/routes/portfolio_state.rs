use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, ResponseError, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::{get_api_token_header, ApiToken};

#[derive(thiserror::Error, Debug)]
pub enum PortfolioStateError {
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for PortfolioStateError {
    fn status_code(&self) -> StatusCode {
        match &self {
            &PortfolioStateError::AuthError(_) => StatusCode::UNAUTHORIZED,
            PortfolioStateError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PortfolioStateBody {
    rebalancer_label: String,
    data: serde_json::Value,
}

#[tracing::instrument(
    name = "adding a new portfolio_sate",
    skip(body, db_pool),
    fields(
        rebalancer_label = %body.rebalancer_label,
        data = %body.data,
    )
)]
pub async fn handler(
    request: HttpRequest,
    body: web::Json<PortfolioStateBody>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, PortfolioStateError> {
    let api_token = {
        let auth_token =
            get_api_token_header(request.headers()).map_err(PortfolioStateError::AuthError)?;

        let record = sqlx::query!(
            r#"
            select
                id
            from auth_tokens
            where token = $1
            "#,
            auth_token
        )
        .fetch_one(db_pool.as_ref())
        .await
        .expect("failed on fetch saved portfolio_state");

        ApiToken::new(record.id, auth_token)
    };

    match insert_portfolio_state(&db_pool, &body, &api_token).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[tracing::instrument(
    name = "saving new portfolio_state details in the database"
    skip(body, db_pool)
)]
pub async fn insert_portfolio_state(
    db_pool: &PgPool,
    body: &PortfolioStateBody,
    api_token: &ApiToken,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        insert into portfolio_states (auth_token_id, rebalancer_label, data)
        values ($1, $2, $3)
        "#,
        api_token.auth_token_id(),
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
