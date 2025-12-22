use axum::{routing::{get, post}, Router, extract::{Path, State}, Json, http::HeaderMap};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{state::AppState, errors::ApiError, auth::require_auth};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct SignedPrekey {
    pub key_id: i32,
    pub public_key: String,
    pub signature: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimePrekey {
    pub key_id: i32,
    pub public_key: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadBundleReq {
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub identity_key_ed25519_b64: String,
    pub static_x25519_b64: String,
    pub signed_prekey_x25519_b64: String,
    pub signed_prekey_signature_b64: String,
    pub one_time_prekeys_b64: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct UploadBundleResp { pub ok: bool }

#[derive(Debug, Serialize)]
pub struct PrekeyBundleResp {
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub identity_key_ed25519_b64: String,
    pub static_x25519_b64: String,
    pub signed_prekey_x25519_b64: String,
    pub signed_prekey_signature_b64: String,
    pub one_time_prekey_b64: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/keys/upload", post(upload_bundle))
        .route("/keys/bundle/:user_id", get(fetch_bundle))
}

pub async fn upload_bundle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UploadBundleReq>,
) -> Result<Json<UploadBundleResp>, ApiError> {
    let claims = require_auth(&headers, &state)?;
    // Ensure user requesting upload matches token
    if claims.sub != req.user_id { return Err(ApiError::Unauthorized); }

    // Upsert Identity/Signed Prekey Bundle
    sqlx::query!(
        r#"
        INSERT INTO prekey_bundles 
        (user_id, device_id, identity_key_ed25519_b64, 
         signed_prekey_x25519_b64, signed_prekey_signature_b64, static_x25519_b64)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (user_id, device_id) 
        DO UPDATE SET 
            identity_key_ed25519_b64 = EXCLUDED.identity_key_ed25519_b64,
            signed_prekey_x25519_b64 = EXCLUDED.signed_prekey_x25519_b64,
            signed_prekey_signature_b64 = EXCLUDED.signed_prekey_signature_b64,
            static_x25519_b64 = EXCLUDED.static_x25519_b64,
            created_at = now()
        "#,
        claims.sub,
        req.device_id,
        req.identity_key_ed25519_b64,
        req.signed_prekey_x25519_b64,
        req.signed_prekey_signature_b64,
        req.static_x25519_b64
    )
    .execute(&state.db)
    .await.map_err(|e| { tracing::error!("db: {}", e); ApiError::Internal })?;

    // Insert OTKs (Bulk insert or loop)
    for key_b64 in req.one_time_prekeys_b64 {
        sqlx::query!(
            r#"
            INSERT INTO one_time_prekeys (user_id, device_id, prekey_b64)
            VALUES ($1, $2, $3)
            "#,
            claims.sub,
            req.device_id,
            key_b64
        )
        .execute(&state.db)
        .await.ok(); // ignore dupes
    }

    Ok(Json(UploadBundleResp { ok: true }))
}

pub async fn fetch_bundle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<PrekeyBundleResp>>, ApiError> {
    let _claims = require_auth(&headers, &state)?;

    // Fetch bundles for ALL devices of the user
    let bundles = sqlx::query!(
        r#"
        SELECT user_id, device_id, identity_key_ed25519_b64, 
               static_x25519_b64, signed_prekey_x25519_b64, signed_prekey_signature_b64 
        FROM prekey_bundles 
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(&state.db)
    .await.map_err(|e| { tracing::error!("db bundles fetch: {}", e); ApiError::Internal })?;

    let mut results = Vec::new();

    for b in bundles {
        // Pop one OTK per device
        let otk = sqlx::query!(
            r#"
            WITH popped AS (
                SELECT id, prekey_b64 
                FROM one_time_prekeys 
                WHERE user_id = $1 AND device_id = $2 AND consumed_at IS NULL
                LIMIT 1
                FOR UPDATE SKIP LOCKED
            )
            UPDATE one_time_prekeys 
            SET consumed_at = now() 
            FROM popped 
            WHERE one_time_prekeys.id = popped.id
            RETURNING popped.prekey_b64
            "#,
            user_id,
            b.device_id
        )
        .fetch_optional(&state.db)
        .await.map_err(|e| { tracing::error!("db otk pop: {}", e); ApiError::Internal })?;

        results.push(PrekeyBundleResp {
            user_id,
            device_id: b.device_id,
            identity_key_ed25519_b64: b.identity_key_ed25519_b64,
            static_x25519_b64: b.static_x25519_b64,
            signed_prekey_x25519_b64: b.signed_prekey_x25519_b64,
            signed_prekey_signature_b64: b.signed_prekey_signature_b64,
            one_time_prekey_b64: otk.map(|o| o.prekey_b64),
        });
    }

    Ok(Json(results))
}
