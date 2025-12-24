#[derive(Debug, Deserialize)]
pub struct SendReq {
    pub to_user_id: Uuid,
    pub to_device_id: Uuid,
    pub from_device_id: Uuid,
    pub ciphertext_b64: String,
    pub msg_type: String,
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
    pub to_user_id: Uuid,
    pub to_device_id: Uuid,
    pub from_user_id: Uuid, 
    pub from_device_id: Uuid,
    pub msg_type: String,
    pub ciphertext_b64: String,
    pub created_at: chrono::DateTime<chrono::Utc>, 
}

// Note: InboxItem is used directly as the response array item. 

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/messages/send", post(send))
        .route("/messages/inbox/:user_id", get(inbox))
}

pub async fn send(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<SendReq>,
) -> Result<Json<SendResp>, ApiError> {
    let claims = require_auth(&headers, &state)?;
    let sender_id = claims.sub;
    
    sqlx::query!(
        r#"
        INSERT INTO messages 
        (to_user_id, to_device_id, from_user_id, from_device_id, 
         ciphertext_b64, msg_type, created_at, delivered)
        VALUES ($1, $2, $3, $4, $5, $6, now(), false)
        "#,
        req.to_user_id,
        req.to_device_id,
        sender_id,
        req.from_device_id,
        req.ciphertext_b64,
        req.msg_type
    )
    .execute(&state.db)
    .await
    .map_err(|e| { tracing::error!("send msg: {}", e); ApiError::Internal })?;

    Ok(Json(SendResp { ok: true }))
}

pub async fn inbox(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
    Query(q): Query<InboxQuery>,
) -> Result<Json<Vec<InboxItem>>, ApiError> {
    let claims = require_auth(&headers, &state)?;
    
    // Security check: You can only fetch inbox for YOUR user_id
    if claims.sub != user_id {
        return Err(ApiError::Unauthorized);
    }
    
    // For specific device inbox:
    // Phase 2: Prefer device_id from token claims if present (secure binding)
    // Fallback to query param for Phase 1 legacy tokens.
    let target_device_id = claims.device.unwrap_or(q.device_id);
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
        RETURNING id, to_user_id, to_device_id, from_user_id, from_device_id, msg_type, ciphertext_b64, created_at
        "#,
        claims.sub,
        target_device_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| { tracing::error!("inbox fetch: {}", e); ApiError::InternalServerError })?;

    Ok(Json(messages))
}
