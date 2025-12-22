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

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/attachments/presign", post(presign))
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
