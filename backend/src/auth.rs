use crate::errors::ApiError;
use crate::state::AppState;
use axum::{extract::State, http::HeaderMap};
use jsonwebtoken::{DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,          // user_id
    pub device: Option<Uuid>,
    pub exp: usize,
}

pub fn require_auth(headers: &HeaderMap, state: &AppState) -> Result<Claims, ApiError> {
    let auth = headers.get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;

    let token = auth.strip_prefix("Bearer ").ok_or(ApiError::Unauthorized)?;

    let claims = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    ).map_err(|_| ApiError::Unauthorized)?.claims;

    Ok(claims)
}
