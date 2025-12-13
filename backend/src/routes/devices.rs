use axum::{routing::post, Router, extract::{State}, Json, http::HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{state::AppState, errors::ApiError, auth::require_auth};

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceReq {
    pub device_id: Uuid,
    pub platform: String,
    pub identity_key: String, // base64 public key
    pub push_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RegisterDeviceResp { pub ok: bool }

pub fn router() -> Router<AppState> {
    Router::new().route("/devices/register", post(register_device))
}

pub async fn register_device(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<RegisterDeviceReq>,
) -> Result<Json<RegisterDeviceResp>, ApiError> {
    let _claims = require_auth(&headers, &state)?;
    // TODO: insert into DB.
    Ok(Json(RegisterDeviceResp { ok: true }))
}
