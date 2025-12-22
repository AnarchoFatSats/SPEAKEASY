use axum::{routing::post, Router, extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc, Duration};
use jsonwebtoken::{EncodingKey, Header, Algorithm};
use sha2::{Sha256, Digest};
use crate::{state::AppState, errors::ApiError, auth::Claims};

#[derive(Debug, Deserialize)]
pub struct RegisterReq {
    pub display_name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthResp {
    pub user_id: Uuid,
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub code: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

fn hash_identifier(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterReq>,
) -> Result<Json<AuthResp>, ApiError> {
    let phone_hash = req.phone.as_ref().map(|s| hash_identifier(s));
    let email_hash = req.email.as_ref().map(|s| hash_identifier(s));

    let user_id = Uuid::new_v4();

    // Insert user
    sqlx::query!(
        r#"
        INSERT INTO users (id, display_name, phone_hash, email_hash)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        req.display_name,
        phone_hash,
        email_hash
    )
    .execute(&state.db)
    .await
    .map_err(|e| {
        // Handle uniqueness constraint violation?
        tracing::error!("register db: {}", e);
        ApiError::Internal 
    })?;

    // Issue Token
    let token = issue_token(&state.jwt_secret, user_id)?;

    Ok(Json(AuthResp { user_id, access_token: token }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<AuthResp>, ApiError> {
    // 1. Verify Code (DEV STUB: Accept ANY code for now, or "000000")
    if req.code != "000000" {
        // In real V1, we would check a redis OTP table.
        // allow bypass for testing.
        // return Err(ApiError::Unauthorized);
    }

    // 2. Lookup User
    let (phone_hash, email_hash) = (
        req.phone.as_ref().map(|s| hash_identifier(s)),
        req.email.as_ref().map(|s| hash_identifier(s))
    );

    let user = sqlx::query!(
        r#"
        SELECT id FROM users 
        WHERE (phone_hash IS NOT NULL AND phone_hash = $1)
           OR (email_hash IS NOT NULL AND email_hash = $2)
        LIMIT 1
        "#,
        phone_hash,
        email_hash
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| { tracing::error!("login user lookup db: {}", e); ApiError::Internal })?;

    let user_record = user.ok_or(ApiError::NotFound("User not found".into()))?;

    // 3. Issue Token
    let token = issue_token(&state.jwt_secret, user_record.id)?;

    Ok(Json(AuthResp { user_id: user_record.id, access_token: token }))
}

fn issue_token(secret: &str, user_id: Uuid) -> Result<String, ApiError> {
    let exp = (Utc::now() + Duration::days(30)).timestamp() as usize;
    let claims = Claims { sub: user_id, device: None, exp };

    jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).map_err(|_| ApiError::Internal)
}
