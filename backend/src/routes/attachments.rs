use axum::{
    routing::{get, post}, Router, extract::{State, Path}, Json, http::HeaderMap
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{state::AppState, errors::ApiError, auth::require_auth};

#[derive(Debug, Deserialize)]
pub struct PresignReq {
    pub content_type: String,
    pub size_bytes: i64,
}

#[derive(Debug, Serialize)]
pub struct PresignResp {
    pub attachment_id: Uuid,
    pub storage_key: String,
    pub upload_url: String,
}

#[derive(Debug, Serialize)]
pub struct DownloadReq {
    pub attachment_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct DownloadResp {
    pub download_url: String,
}

#[derive(Debug, Deserialize)]
pub struct CompleteReq {
    pub attachment_id: Uuid,
    pub storage_key: String,
    pub sha256_ciphertext_b64: String,
    pub size_bytes: i64,
    pub content_type: Option<String>,
    pub enc_alg: String,
    pub nonce_b64: String,
}

#[derive(Debug, Serialize)]
pub struct OkResp {
    pub ok: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/attachments/presign", post(presign))
        .route("/attachments/complete", post(complete_upload))
        .route("/attachments/url/:attachment_id", get(get_download_url))
}

pub async fn presign(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<PresignReq>,
) -> Result<Json<PresignResp>, ApiError> {
    let claims = require_auth(&headers, &state)?;
    
    let attachment_id = Uuid::new_v4();
    let storage_key = format!("{}/{}.bin", claims.sub, attachment_id);
    let expires_in = 600; // 10 minutes

    let upload_url = state.bucket.presign_put(&storage_key, expires_in as u32, Some(&req.content_type))
        .map_err(|e| { tracing::error!("s3 presign put: {}", e); ApiError::Internal })?;

    sqlx::query!(
        r#"
        INSERT INTO attachments (id, owner_user_id, storage_key, content_type, size_bytes)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        attachment_id,
        claims.sub,
        storage_key,
        req.content_type,
        req.size_bytes
    )
    .execute(&state.db)
    .await
    .map_err(|e| { tracing::error!("db attach: {}", e); ApiError::Internal })?;

    Ok(Json(PresignResp {
        attachment_id,
        storage_key,
        upload_url,
    }))
}

pub async fn get_download_url(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(attachment_id): Path<Uuid>,
) -> Result<Json<DownloadResp>, ApiError> {
    let _claims = require_auth(&headers, &state)?;

    let meta = sqlx::query!(
        r#"SELECT storage_key FROM attachments WHERE id = $1 AND deleted = false"#,
        attachment_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { tracing::error!("db attach get: {}", e); ApiError::Internal })?;

    let meta = meta.ok_or(ApiError::NotFound("Attachment not found".into()))?;

    let expires_in = 3600; // 1 hour
    let download_url = state.bucket.presign_get(&meta.storage_key, expires_in as u32, None)
        .map_err(|e| { tracing::error!("s3 presign get: {}", e); ApiError::Internal })?;

    Ok(Json(DownloadResp {
        download_url,
    }))
}

pub async fn complete_upload(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<CompleteReq>,
) -> Result<Json<OkResp>, ApiError> {
    let claims = require_auth(&headers, &state)?;

    // Verify attachment exists and belongs to user
    let existing = sqlx::query!(
        r#"SELECT owner_user_id FROM attachments WHERE id = $1"#,
        req.attachment_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { tracing::error!("db complete lookup: {}", e); ApiError::Internal })?;

    let record = existing.ok_or(ApiError::NotFound("Attachment not found".into()))?;
    
    if record.owner_user_id != claims.sub {
        return Err(ApiError::Unauthorized);
    }

    // Update with ciphertext metadata
    sqlx::query!(
        r#"
        UPDATE attachments 
        SET sha256_ciphertext_b64 = $1,
            enc_alg = $2,
            nonce_b64 = $3,
            size_bytes = $4,
            content_type = COALESCE($5, content_type),
            finalized = true
        WHERE id = $6
        "#,
        req.sha256_ciphertext_b64,
        req.enc_alg,
        req.nonce_b64,
        req.size_bytes,
        req.content_type,
        req.attachment_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| { tracing::error!("db complete update: {}", e); ApiError::Internal })?;

    Ok(Json(OkResp { ok: true }))
}
