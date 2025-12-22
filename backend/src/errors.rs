use axum::{http::StatusCode, response::{IntoResponse, Response}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal error")]
    Internal,
    #[error("not found: {0}")]
    NotFound(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            ApiError::BadRequest(s) => (StatusCode::BAD_REQUEST, s),
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "internal".to_string()),
            ApiError::NotFound(s) => (StatusCode::NOT_FOUND, s),
        };
        (status, axum::Json(serde_json::json!({ "error": msg }))).into_response()
    }
}
