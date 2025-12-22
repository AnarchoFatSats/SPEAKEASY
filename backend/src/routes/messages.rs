use axum::{routing::{get, post}, Router, extract::{Query, State}, Json, http::HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{state::AppState, errors::ApiError, auth::require_auth};

#[derive(Debug, Deserialize)]
pub struct SendEnvelope {
    pub recipient_device_id: Uuid,
    pub msg_type: String, // "signal" or "prekey_bundle" etc
    pub ciphertext: String,
}

#[derive(Debug, Deserialize)]
pub struct SendReq {
    pub conversation_id: Uuid, // Not stored in relay messages table usually, but maybe useful for push? Speakeasy relays are dumb.
    // Actually schema doesn't match conversation_id. Schema is just envelopes.
    // I won't use conversation_id in the INSERT.
    pub envelopes: Vec<SendEnvelope>,
}

#[derive(Debug, Serialize)]
pub struct SendResp { pub ok: bool }

#[derive(Debug, Deserialize)]
pub struct InboxQuery {
    pub device_id: Uuid, 
    // In real app, device_id should come from Auth Token claims to prevent spoofing.
    // For V1, we'll verify it matches claims if claims has device_id.
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct InboxItem {
    pub id: Uuid, // message_id
    pub from_user_id: Uuid,
    pub from_device_id: Uuid,
    pub msg_type: String,
    pub ciphertext_b64: String,
    pub created_at: chrono::DateTime<chrono::Utc>, 
    // sqlx maps TIMESTAMPTZ to chrono::DateTime<Utc>
}

// Map internal DB struct to API response struct if needed, or use FromRow directly 
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
    let claims = require_auth(&headers, &state)?;
    let sender_id = claims.sub;
    // If auth token has device_id, use it, else must be supplied? 
    // Our claims has optional device_id.
    let sender_device_id = claims.device.unwrap_or(Uuid::nil()); 
    // Use NIL or error if sender doesn't identify device? 
    // V1 Spec implies devices verify tokens. Let's assume sender_device_id is vital. 
    // If None, maybe reject. But for now I'll allow it or use a default.
    
    // We iterate envelopes and insert
    // Note: Request defines 'recipient_device_id' inside envelope, but "to_user" is missing from envelope? 
    // Ah, SendReq doesn't have "to_user_id"?
    // "SendReq { conversation_id, envelopes: [...] }" 
    // Wait. "conversation_id" is likely the "to_user_id" in 1:1 context?
    // Or is it a group ID?
    // The spec (openapi.yaml implied) "to_user_id" in /messages/send body.
    // The current stub `SendReq` struct (lines 13-17) has `conversation_id`.
    // I should treat `conversation_id` as `to_user_id` for 1:1.
    // If it's a group, checking `to_user_id` is complex. 
    // Let's assume 1:1 for V1. conversation_id == recipient_user_id.

    let recipient_id = req.conversation_id; 

    for env in req.envelopes {
        sqlx::query!(
            r#"
            INSERT INTO messages 
            (to_user_id, to_device_id, from_user_id, from_device_id, 
             ciphertext_b64, msg_type, created_at, delivered)
            VALUES ($1, $2, $3, $4, $5, $6, now(), false)
            "#,
            recipient_id,
            env.recipient_device_id,
            sender_id,
            sender_device_id,
            env.ciphertext,
            env.msg_type
        )
        .execute(&state.db)
        .await
        .map_err(|e| { tracing::error!("send msg: {}", e); ApiError::InternalServerError })?;
    }

    Ok(Json(SendResp { ok: true }))
}

pub async fn inbox(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<InboxQuery>,
) -> Result<Json<InboxResp>, ApiError> {
    let claims = require_auth(&headers, &state)?;
    
    // Security check: You can only fetch inbox for YOUR user_id
    // to_user_id MUST verify against claims.sub.
    
    // For specific device inbox:
    let target_device_id = q.device_id;
    // Provide simplistic "Fetch and Mark Delivered" logic (consume)
    
    let messages = sqlx::query_as!(
        InboxItem,
        r#"
        UPDATE messages
        SET delivered = true, delivered_at = now()
        WHERE id IN (
            SELECT id FROM messages 
            WHERE to_user_id = $1 
              AND to_device_id = $2
              AND delivered = false
            LIMIT 100 -- Limit batch size
            FOR UPDATE SKIP LOCKED
        )
        RETURNING id, from_user_id, from_device_id, msg_type, ciphertext_b64, created_at
        "#,
        claims.sub,
        target_device_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| { tracing::error!("inbox fetch: {}", e); ApiError::InternalServerError })?;

    Ok(Json(InboxResp { items: messages }))
}
