use axum::{routing::post, Router, extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};
use jsonwebtoken::{EncodingKey, Header, Algorithm};
use crate::{state::AppState, errors::ApiError, auth::Claims};

#[derive(Debug, Deserialize)]
pub struct RequestCodeReq {
    pub identifier: String,
}
#[derive(Debug, Serialize)]
pub struct RequestCodeResp {
    pub ok: bool,
}

#[derive(Debug, Deserialize)]
pub struct VerifyCodeReq {
    pub identifier: String,
    pub code: String,
    pub device_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct VerifyCodeResp {
    pub token: String,
    pub user_id: Uuid,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/request_code", post(request_code))
        .route("/auth/verify_code", post(verify_code))
}

pub async fn request_code(
    State(_state): State<AppState>,
    Json(_req): Json<RequestCodeReq>,
) -> Result<Json<RequestCodeResp>, ApiError> {
    // TODO: integrate SMS/email provider. For now, dev stub.
    Ok(Json(RequestCodeResp { ok: true }))
}

pub async fn verify_code(
    State(state): State<AppState>,
    Json(_req): Json<VerifyCodeReq>,
) -> Result<Json<VerifyCodeResp>, ApiError> {
    // TODO: verify OTP, create user record if needed.
    let user_id = Uuid::new_v4();

    let exp = (Utc::now() + Duration::days(30)).timestamp() as usize;
    let claims = Claims { sub: user_id, device: None, exp };

    let token = jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ).map_err(|_| ApiError::Internal)?;

    Ok(Json(VerifyCodeResp { token, user_id }))
}
