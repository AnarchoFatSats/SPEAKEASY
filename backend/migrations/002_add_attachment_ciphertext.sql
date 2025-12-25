-- Phase 3: Add ciphertext metadata columns to attachments table

ALTER TABLE attachments
    ADD COLUMN IF NOT EXISTS sha256_ciphertext_b64 TEXT,
    ADD COLUMN IF NOT EXISTS enc_alg VARCHAR(32),
    ADD COLUMN IF NOT EXISTS nonce_b64 TEXT,
    ADD COLUMN IF NOT EXISTS finalized BOOLEAN DEFAULT FALSE;

COMMENT ON COLUMN attachments.sha256_ciphertext_b64 IS 'SHA256 hash of the encrypted bytes on S3';
COMMENT ON COLUMN attachments.enc_alg IS 'Encryption algorithm (xchacha20poly1305 or aes256gcm)';
COMMENT ON COLUMN attachments.nonce_b64 IS 'Base64 encoded nonce/IV used for encryption';
COMMENT ON COLUMN attachments.finalized IS 'True after client calls /attachments/complete';
