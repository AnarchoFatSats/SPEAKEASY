# CryptoSpec (v1.0) — Speakeasy Privacy Suite

This document is a **concrete cryptographic specification**. It is written so engineering can implement it without “wiggle room”.

## Goals
- **End-to-end encrypted** messaging and attachments between app users.
- Server is a **dumb relay**: cannot decrypt content.
- Compromise of backend storage does **not** reveal message content.
- Local device storage is encrypted; keys are protected by hardware-backed keystore where possible.

## Non-Goals / Explicit Limits
- Does not protect against a fully compromised OS (root/jailbreak + malware).
- Does not hide or intercept iOS iMessage/SMS.
- Does not provide “perfect anonymity” unless user opts into anonymous ID mode.

---

## Cryptographic Primitives

### Messaging (E2EE)
- Protocol: **Signal Protocol** (MANDATORY)
  - Must use official **libsignal** client libraries (Rust/Java/Swift wrappers) on mobile. 
  - *Risk Acceptance:* Pure Dart ports are permitted ONLY if they have undergone a third-party cryptographic audit.
  - X3DH for initial key agreement.
  - Double Ratchet for forward secrecy.
- Curve: **Curve25519** (X25519).
- Signatures: **Ed25519**.
- KDF: **HKDF-SHA256**.

### Vault / local storage encryption
- AEAD: **XChaCha20-Poly1305** (preferred over AES-GCM for mobile perf/safety).
- KDF for user passphrase: **Argon2id** (m=64MB, t=3, p=1).
- Derivation: `master_key = HKDF(ikm = hardware_key || passphrase_key, info="vault-master", salt=random_32)`

### Attachments
- Encrypt on sender device, decrypt on receiver device.
- Use a random per-attachment key:
  - `K_att = random(32)`
  - `nonce = random(24)` for XChaCha20-Poly1305
- Server stores only ciphertext + metadata required for transport (size, content-type if you must, but ideally keep minimal).

---

## Identity, Devices, and Keys

### Account identity
Each **device** has an **Identity Key Pair**:
- `IK = (ik_priv, ik_pub)` (Curve25519 / Ed25519 as required by library)
- Stored **only on device** in hardware-backed secure storage.

### Prekeys
Each device uploads:
- One **Signed Prekey** `SPK` with signature `Sig(IK_sign_priv, spk_pub)`
- A batch of **One-Time Prekeys** `OPK[i]`

Server stores:
- `(device_id, spk_id, spk_pub, spk_sig, opk_id, opk_pub...)`

### Session creation
When Alice wants to message Bob:
1. Alice fetches Bob’s Prekey Bundle (IKB, SPKB, OPKB) from server.
2. Alice runs X3DH to derive a shared secret.
3. Alice stores session state locally and sends an initial message (prekey message).
4. Bob consumes OPK (marks as used) and establishes session.

### Multi-device
- Each user may have multiple devices.
- Sender encrypts the same plaintext separately for each recipient device (or uses Sender Keys for groups).
- Server relays envelopes per device.

---

## Message Format (Transport Envelope)

Server-visible envelope (JSON/Protobuf):
- `message_id` (uuid)
- `conversation_id` (uuid)
- `sender_device_id` (uuid)
- `recipient_device_id` (uuid)
- `created_at` (unix ms)
- `ciphertext` (bytes, base64)
- `msg_type` (enum: PREKEY, STANDARD, GROUP, RECEIPT)
- `expires_at` (optional, for disappearing messages)

**No plaintext**, no sender name, no contact labels.

---

## Attachments Flow

1) Sender encrypts file locally with XChaCha20-Poly1305 using `K_att`.
2) Sender uploads ciphertext to object storage using server-issued presigned URL.
3) Sender sends an E2EE message that includes:
   - `attachment_pointer` (opaque object key)
   - `K_att` (encrypted inside message)
   - `sha256(ciphertext)` (optional for integrity)
4) Receiver downloads ciphertext from object store and decrypts with `K_att`.

---

## Local Vault Key Hierarchy

### Keys
- `K_hw`: hardware key (device-bound; from Keychain/Keystore)
- `K_user`: derived from user passcode/passphrase with Argon2id (optional but recommended)
- `K_master`: vault master key, derived via HKDF from `K_hw || K_user`
- Per-item key:
  - `K_item = HKDF(K_master, info="vault-item:"+item_id, salt=item_salt)`

### Storage format for encrypted blob
Binary:
- `version (1 byte) = 0x01`
- `nonce (24 bytes)`
- `ad_len (2 bytes)`
- `ad (ad_len bytes)` — e.g., item_id, mime type, created_at (not secret but integrity-protected)
- `ciphertext (N bytes)` — includes Poly1305 tag

---

## Backups / Account Recovery (Zero-Knowledge)

### Policy: Mandatory Mode B (Recovery Phrase)
We standardize on **BIP39 Recovery Phrases** (24 words).
- **Setup:** User generates 24 words. Application derives `K_recovery` from this seed.
- **Backup Payload:** 
  - `E(K_recovery, { IdentityKey_Priv, Vault_Master_Key })`
  - Uploaded to Speakeasy Server in binary blob format.
- **Restoration:**
  - User enters 24 words -> `K_recovery` -> Decrypts blob -> Restoration complete.
- **Loss Scenario:**
  - If user loses phone AND 24 words: **Game Over**. Data is mathematically irretrievable.
  - *We do not support "forgot password" email resets.*

---

## Identity Verification UX
- Each contact pairing shows a “Safety Number”:
  - e.g., hash of both identity keys
- Provide QR code scan to verify.
- Warn user on identity key changes (“possible re-install/new device”).

---

## Push Notifications
- Push payload must contain:
  - `event_type: "new_message"`
  - `message_id`
  - no sender name, no plaintext.
- Client fetches ciphertext via authenticated call and decrypts locally.

---

## Mandatory Security Reviews
- If you do NOT use official libsignal, you need:
  - external cryptography audit prior to claiming Signal-level security.
- Regardless:
  - run static analysis
  - dependency pinning & SBOM
  - penetration testing before launch
