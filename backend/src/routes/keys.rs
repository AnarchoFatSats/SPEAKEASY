use axum::{routing::{get, post}, Router, extract::{Path, State}, Json, http::HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{state::AppState, errors::ApiError, auth::require_auth};

#[derive(Debug, Deserialize)]
pub struct SignedPrekey {
    pub key_id: i32,
    pub public_key: String,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct OneTimePrekey {
    pub key_id: i32,
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadBundleReq {
    pub device_id: Uuid,
    pub signed_prekey: SignedPrekey,
    pub one_time_prekeys: Vec<OneTimePrekey>,
}

#[derive(Debug, Serialize)]
pub struct UploadBundleResp { pub ok: bool }

#[derive(Debug, Serialize)]
pub struct PrekeyBundleResp {
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub identity_key: String,
    pub signed_prekey: SignedPrekey,
    pub one_time_prekey: Option<OneTimePrekey>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/keys/upload_bundle", post(upload_bundle))
        .route("/keys/fetch_bundle/:user_id", get(fetch_bundle))
}

pub async fn upload_bundle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(_req): Json<UploadBundleReq>,
) -> Result<Json<UploadBundleResp>, ApiError> {
    let _claims = require_auth(&headers, &state)?;
    // TODO: store keys, mark OPKs available
    Ok(Json(UploadBundleResp { ok: true }))
}

pub async fn fetch_bundle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<PrekeyBundleResp>, ApiError> {
    let _claims = require_auth(&headers, &state)?;
    // TODO: fetch a device + pop one OPK
    // Dev stub:
    let device_id = Uuid::new_v4();
    Ok(Json(PrekeyBundleResp {
        user_id,
        device_id,
        identity_key: "BASE64_ID_KEY".to_string(),
        signed_prekey: SignedPrekey { key_id: 1, public_key: "BASE64_SPK".to_string(), signature: "BASE64_SIG".to_string() },
        one_time_prekey: Some(OneTimePrekey { key_id: 99, public_key: "BASE64_OPK".to_string() }),
    }))
}
