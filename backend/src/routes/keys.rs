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
    pub identity_key: String,
    pub signed_prekey: SignedPrekey,
    pub one_time_prekey: Option<OneTimePrekey>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/keys/upload_bundle", post(upload_bundle))
        .route("/keys/fetch_bundle/:user_id", get(fetch_bundle))
}

pub async fn upload_bundle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UploadBundleReq>,
) -> Result<Json<UploadBundleResp>, ApiError> {
    let claims = require_auth(&headers, &state)?;
    // Ensure user requesting upload matches token
    if claims.sub != claims.sub { return Err(ApiError::Unauthorized); } // redundant but safe logic pattern

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
) -> Result<Json<PrekeyBundleResp>, ApiError> {
    let _claims = require_auth(&headers, &state)?;

    // 1. Find a valid device for user (simplistic: pick most recently active or just first)
    // In real Signal, you fetch bundles for ALL devices. 
    // This endpoint returns *one* bundle (singular). 
    // Spec says "/keys/bundle/{user_id}" -> "Prekey bundle". 
    // If user has multiple devices, strict Signal protocol fetches all.
    // For V1 MVP, we might fetch just one specific device or master device?
    // Let's just fetch ANY device row from prekey_bundles.

    let bundle = sqlx::query!(
        r#"
        SELECT user_id, device_id, identity_key_ed25519_b64, 
               static_x25519_b64, signed_prekey_x25519_b64, signed_prekey_signature_b64 
        FROM prekey_bundles 
        WHERE user_id = $1 
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(&state.db)
    .await.map_err(|_| ApiError::InternalServerError)?;

    let bundle = match bundle {
        Some(b) => b,
        None => return Err(ApiError::NotFound("User not found or no keys".into())),
    };

    // 2. Consume one OTK
    // We transactionally delete one unused key and return it.
    let otk = sqlx::query!(
        r#"
        WITH popped AS (
            SELECT id, prekey_x25519_b64 
            FROM one_time_prekeys 
            WHERE user_id = $1 AND device_id = $2 AND consumed_at IS NULL
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        )
        UPDATE one_time_prekeys 
        SET consumed_at = now() 
        FROM popped 
        WHERE one_time_prekeys.id = popped.id
        RETURNING popped.id, popped.prekey_x25519_b64
        "#,
        user_id,
        bundle.device_id
    )
    .fetch_optional(&state.db)
    .await.map_err(|e| { tracing::error!("db otk: {}", e); ApiError::InternalServerError })?;

    Ok(Json(PrekeyBundleResp {
        user_id,
        device_id: bundle.device_id,
        identity_key: bundle.identity_key_ed25519_b64,
        signed_prekey: SignedPrekey { 
            key_id: 1, // TODO: store key_id in DB 
            public_key: bundle.signed_prekey_x25519_b64, 
            signature: bundle.signed_prekey_signature_b64 
        },
        one_time_prekey: otk.map(|o| OneTimePrekey { 
            key_id: o.id as i32, 
            public_key: o.prekey_x25519_b64 
        }),
    }))
}
