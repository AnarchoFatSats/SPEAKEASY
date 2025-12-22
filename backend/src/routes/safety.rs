use axum::{routing::post, Router, extract::State, Json, http::HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{state::AppState, errors::ApiError, auth::require_auth};

#[derive(Debug, Deserialize)]
pub struct BlockReq {
    pub blocked_user_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ReportReq {
    pub reported_user_id: Uuid,
    pub reason: String,
    pub decrypted_content: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/block", post(block_user))
        .route("/reports/submit", post(submit_report))
}

pub async fn block_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<BlockReq>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let _claims = require_auth(&headers, &state)?;
    
    // DEV STUB: In Phase 2/3 we insert into 'blocks' table
    tracing::info!("User blocking: {:?}", req.blocked_user_id);
    
    Ok(Json(serde_json::json!({ "ok": true })))
}

pub async fn submit_report(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<ReportReq>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let _claims = require_auth(&headers, &state)?;
    
    // DEV STUB: In Phase 2/3 we insert into 'reports' table 
    tracing::info!("Abuse report submitted: {:?}", req.reported_user_id);
    
    Ok(Json(serde_json::json!({ "ok": true })))
}
