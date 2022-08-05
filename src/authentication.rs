use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

pub const API_TOKEN_HEADER: &str = "X-Api-Token";

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("invalid auth_token.")]
    InvalidAuthToken(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct ApiToken(Uuid);

impl ApiToken {
    pub fn new(auth_token: Uuid) -> ApiToken {
        ApiToken(auth_token)
    }

    pub fn auth_token(&self) -> Uuid {
        self.0
    }

    #[tracing::instrument(name = "validate auth_token", skip(self, db_pool))]
    pub async fn validate(&self, db_pool: &PgPool) -> Result<Uuid, AuthError> {
        get_stored_token(self, db_pool)
            .await
            .map_err(AuthError::InvalidAuthToken)
    }
}

#[tracing::instrument(name = "get stored auth_token", skip(api_token, db_pool))]
async fn get_stored_token(api_token: &ApiToken, db_pool: &PgPool) -> Result<Uuid, anyhow::Error> {
    let auth_token_id = sqlx::query!(
        r#"
            select
                id
            from auth_tokens
            where token = $1
		"#,
        api_token.auth_token()
    )
    .fetch_one(db_pool)
    .await
    .context("failed on fetch saved portfolio_state")?
    .id;

    Ok(auth_token_id)
}
