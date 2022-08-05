use actix_web::{http::header::HeaderMap, Result};
use anyhow::{Context, Ok};
use uuid::Uuid;

use crate::authentication::API_TOKEN_HEADER;

pub mod portfolio_state;

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
