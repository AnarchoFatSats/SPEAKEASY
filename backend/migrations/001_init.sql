-- 001_init.sql – initial schema for Speakeasy ('V1 merged')

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users
CREATE TABLE users (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    display_name    TEXT NOT NULL,
    phone_hash      TEXT,
    email_hash      TEXT,
    status          TEXT NOT NULL DEFAULT 'active', -- added back for safety
    banned_at       TIMESTAMPTZ,                    -- added back for safe
    ban_reason      TEXT,                           -- added back for safety
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE UNIQUE INDEX users_phone_hash_key ON users (phone_hash) WHERE phone_hash IS NOT NULL;
CREATE UNIQUE INDEX users_email_hash_key ON users (email_hash) WHERE email_hash IS NOT NULL;

-- Devices
CREATE TABLE devices (
    id                  UUID PRIMARY KEY,
    user_id             UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_seen_at        TIMESTAMPTZ,
    revoked_at          TIMESTAMPTZ
);

CREATE INDEX devices_user_id_idx ON devices (user_id);

-- Prekey bundles
CREATE TABLE prekey_bundles (
    user_id                         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id                       UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    identity_key_ed25519_b64        TEXT NOT NULL,
    static_x25519_b64               TEXT NOT NULL,
    signed_prekey_x25519_b64        TEXT NOT NULL,
    signed_prekey_signature_b64     TEXT NOT NULL,
    created_at                      TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, device_id)
);

-- One-time prekeys
CREATE TABLE one_time_prekeys (
    id                  BIGSERIAL PRIMARY KEY,
    user_id             UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id           UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    prekey_x25519_b64   TEXT NOT NULL,
    consumed_at         TIMESTAMPTZ
);

CREATE INDEX one_time_prekeys_user_device_idx ON one_time_prekeys (user_id, device_id) WHERE consumed_at IS NULL;

-- Messages (store-and-forward)
CREATE TABLE messages (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    to_user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    to_device_id    UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    from_user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    from_device_id  UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    ciphertext_b64  TEXT NOT NULL,
    msg_type        TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX messages_to_user_device_idx ON messages (to_user_id, to_device_id, created_at);

-- Attachments
CREATE TABLE attachments (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    owner_user_id   UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    storage_key     TEXT NOT NULL,
    content_type    TEXT NOT NULL,
    size_bytes      BIGINT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX attachments_owner_idx ON attachments (owner_user_id);

-- Backups (encrypted DMK/identity bundles)
CREATE TABLE backups (
    user_id         UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    blob_b64        TEXT NOT NULL,
    version         INTEGER NOT NULL,
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Push tokens
CREATE TABLE push_tokens (
    id              BIGSERIAL PRIMARY KEY,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_id       UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    platform        TEXT NOT NULL, -- "ios" or "android"
    token           TEXT NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_used_at    TIMESTAMPTZ
);

CREATE INDEX push_tokens_user_device_idx ON push_tokens (user_id, device_id);

-- Basic rate-limit table (for abuse control)
CREATE TABLE rate_limits (
    id              BIGSERIAL PRIMARY KEY,
    key             TEXT NOT NULL,
    window_start    TIMESTAMPTZ NOT NULL,
    count           INTEGER NOT NULL
);

CREATE INDEX rate_limits_key_window_idx ON rate_limits (key, window_start);

-- == SAFETY ADDITIONS (Preserved) ==
CREATE TABLE IF NOT EXISTS blocked_users (
  blocker_user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  blocked_user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (blocker_user_id, blocked_user_id)
);

CREATE TABLE IF NOT EXISTS reports (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  reporter_user_id uuid REFERENCES users(id) ON DELETE SET NULL,
  reported_user_id uuid REFERENCES users(id) ON DELETE CASCADE,
  decrypted_content text, 
  reason text,
  created_at timestamptz NOT NULL DEFAULT now()
);
