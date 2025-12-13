-- Speakeasy Relay DB Schema (v1)
-- NOTE: Store minimal metadata. Never store plaintext message content.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  created_at timestamptz NOT NULL DEFAULT now(),
  status text NOT NULL DEFAULT 'active',
  banned_at timestamptz,
  ban_reason text
);

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
  decrypted_content text, -- Optional: user can choose to upload decrypted proof
  reason text,
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS devices (
  id uuid PRIMARY KEY,
  user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  platform text NOT NULL CHECK (platform IN ('ios','android')),
  identity_key text NOT NULL, -- base64 public key
  push_token text,
  created_at timestamptz NOT NULL DEFAULT now(),
  last_seen_at timestamptz
);

CREATE TABLE IF NOT EXISTS signed_prekeys (
  device_id uuid PRIMARY KEY REFERENCES devices(id) ON DELETE CASCADE,
  key_id int NOT NULL,
  public_key text NOT NULL,
  signature text NOT NULL,
  updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS one_time_prekeys (
  device_id uuid NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
  key_id int NOT NULL,
  public_key text NOT NULL,
  used_at timestamptz,
  PRIMARY KEY (device_id, key_id)
);

CREATE TABLE IF NOT EXISTS conversations (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  type text NOT NULL CHECK (type IN ('direct','group')),
  created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS conversation_members (
  conversation_id uuid NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
  device_id uuid NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
  role text NOT NULL DEFAULT 'member',
  joined_at timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY (conversation_id, device_id)
);

CREATE TABLE IF NOT EXISTS messages (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  conversation_id uuid NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
  sender_device_id uuid NOT NULL REFERENCES devices(id) ON DELETE RESTRICT,
  msg_type text NOT NULL,
  ciphertext text NOT NULL, -- base64 blob; server cannot decrypt
  created_at timestamptz NOT NULL DEFAULT now(),
  expires_at timestamptz
);

CREATE TABLE IF NOT EXISTS message_recipients (
  message_id uuid NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
  recipient_device_id uuid NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
  delivered_at timestamptz,
  read_at timestamptz,
  PRIMARY KEY (message_id, recipient_device_id)
);

CREATE INDEX IF NOT EXISTS idx_message_recipients_device
  ON message_recipients (recipient_device_id, delivered_at);

CREATE TABLE IF NOT EXISTS attachments (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  message_id uuid REFERENCES messages(id) ON DELETE SET NULL,
  object_key text NOT NULL,
  size_bytes bigint NOT NULL,
  content_type text,
  sha256 text,
  created_at timestamptz NOT NULL DEFAULT now(),
  expires_at timestamptz
);
