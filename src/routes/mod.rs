use actix_web::http::header::HeaderMap;
use anyhow::{Context, Ok};
use uuid::Uuid;

pub mod portfolio_state;

pub const API_TOKEN_HEADER: &str = "X-Api-Token";
#[derive(Debug)]
pub struct ApiToken {
    auth_token_id: Uuid,
    auth_token: Uuid,
}

impl ApiToken {
    pub fn new(auth_token_id: Uuid, auth_token: Uuid) -> ApiToken {
        ApiToken {
            auth_token_id,
            auth_token,
        }
    }
    pub fn auth_token(&self) -> Uuid {
        self.auth_token
    }
    pub fn auth_token_id(&self) -> Uuid {
        self.auth_token_id
    }
}

pub fn get_api_token_header(headers: &HeaderMap) -> Result<Uuid, anyhow::Error> {
    let header_value = headers
        .get(API_TOKEN_HEADER)
        .context(format!("The '{}' header was missing", API_TOKEN_HEADER))?
        .to_str()
        .context(format!(
            "The '{}' header was not a valid UTF8 string.",
            API_TOKEN_HEADER
        ))?;

    let api_token = Uuid::parse_str(header_value).context(format!(
        "The '{}' header is not a valid uuidv4.",
        API_TOKEN_HEADER
    ))?;

    Ok(api_token)
}
