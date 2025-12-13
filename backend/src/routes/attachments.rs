use axum::{routing::post, Router, extract::State, Json, http::HeaderMap};
use serde::{Deserialize, Serialize};
use crate::{state::AppState, errors::ApiError, auth::require_auth};

#[derive(Debug, Deserialize)]
pub struct PresignReq {
    pub content_type: String,
    pub size_bytes: i64,
}

#[derive(Debug, Serialize)]
pub struct PresignResp {
    pub object_key: String,
    pub upload_url: String,
    pub expires_in_seconds: i64,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/attachments/presign", post(presign))
}

pub async fn presign(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<PresignReq>,
) -> Result<Json<PresignResp>, ApiError> {
    let _claims = require_auth(&headers, &state)?;
    // TODO: use S3/minio SDK to generate presigned PUT
    Ok(Json(PresignResp {
        object_key: format!("dev/{}.bin", uuid::Uuid::new_v4()),
        upload_url: "http://localhost:9000/speakeasy/dev-upload-url".to_string(),
        expires_in_seconds: 600,
    }))
}
