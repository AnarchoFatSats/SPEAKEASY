use axum::{routing::{get, post}, Router, extract::{Query, State}, Json, http::HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{state::AppState, errors::ApiError, auth::require_auth};

#[derive(Debug, Deserialize)]
pub struct SendEnvelope {
    pub recipient_device_id: Uuid,
    pub msg_type: String,
    pub ciphertext: String,
}

#[derive(Debug, Deserialize)]
pub struct SendReq {
    pub conversation_id: Uuid,
    pub envelopes: Vec<SendEnvelope>,
}

#[derive(Debug, Serialize)]
pub struct SendResp { pub ok: bool }

#[derive(Debug, Deserialize)]
pub struct InboxQuery {
    pub device_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct InboxItem {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_device_id: Uuid,
    pub msg_type: String,
    pub ciphertext: String,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct InboxResp {
    pub items: Vec<InboxItem>
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/messages/send", post(send))
        .route("/messages/inbox", get(inbox))
}

pub async fn send(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SendReq>,
) -> Result<Json<SendResp>, ApiError> {
    let _claims = require_auth(&headers, &state)?;
    // TODO: insert into messages + recipients tables, enqueue push
    Ok(Json(SendResp { ok: true }))
}

pub async fn inbox(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<InboxQuery>,
) -> Result<Json<InboxResp>, ApiError> {
    let _claims = require_auth(&headers, &state)?;
    // TODO: fetch undelivered for device_id
    Ok(Json(InboxResp { items: vec![] }))
}
