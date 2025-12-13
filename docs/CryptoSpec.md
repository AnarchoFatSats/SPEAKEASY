# CryptoSpec – Speakeasy

Version: 1.0  
Status: DRAFT → to be reviewed by security engineer before production  
Last updated: 2025‑12‑13

## 0. Goals & Non‑Goals

### 0.1 Goals

- Provide **end‑to‑end encryption (E2EE)** for:
  - 1:1 chats between Speakeasy users
  - Future group chats (not in v1)
- Provide **encrypted local storage** for:
  - Private messages cached on device
  - Vault items (photos, videos, docs, notes)
  - Private SMS inbox on Android (when enabled)
- Provide **zero‑knowledge cloud storage**:
  - Backend relays ciphertext only
  - Backend cannot decrypt messages or attachments
- Support **multi‑device** per user in the future (v1: 1–2 devices acceptable)
- Provide **optional recovery** via a recovery phrase, without server‑held decryption keys

### 0.2 Non‑Goals

- We do **not** protect against:
  - Compromised OS (rooted/jailbroken with full attacker control)
  - Hardware implants, baseband-level interception, or carrier call/SMS metadata
  - Key theft from user via social engineering/phishing
- We do **not** claim:
  - Anonymous, metadata‑free communication in the face of a global passive adversary
  - Perfect forward secrecy if the user leaks their recovery phrase and device secrets simultaneously

All claims in marketing and docs **must align** with this spec.

---

## 1. Identities, Identifiers & Key Types

### 1.1 User identifiers

- Primary application identifier: `user_id` (UUID v4), generated server‑side.
- Optional discovery identifiers (not required to use app):
  - Phone number (E.164 format)
  - Email address
- Discovery identifiers are:
  - Stored hashed (e.g. `H = HKDF-SHA256(salt, raw_identifier)` or `bcrypt/Argon2id`)
  - Never logged in plaintext
  - Only used for:
    - Contact discovery (who else uses Speakeasy)
    - Invites

### 1.2 Device identifiers

- Each device has:
  - `device_id` (UUID v4), generated client‑side on first run
- A user can have multiple devices (v1 may limit to a small number).

### 1.3 Key types per device

Each device maintains the following keys:

1. **Identity key pair (IK)**  
   - Type: Ed25519 (for signatures)  
   - Use: Long‑term device identity / authentication  
   - Stored only on device (private key), public part registered on server.

2. **X25519 static key pair (SPK)**  
   - Type: X25519 (for ECDH)  
   - Use: Static component in X3DH‑style handshake  
   - Public part included in prekey bundle.

3. **Signed prekey (SPK_s)**  
   - Type: X25519 key pair  
   - Public part signed by Ed25519 identity key  
   - Rotated periodically (e.g. every 30 days or on demand).

4. **One‑time prekeys (OPK[i])**  
   - Type: X25519 key pairs  
   - Used once per incoming session, then discarded.

5. **Double Ratchet session keys (per conversation)**  
   - Root key (RK)
   - Sending chain keys (CKs) and receiving chain keys (CKr)
   - Message keys (MK) derived per message.

---

## 2. Randomness

- All randomness MUST come from a **CSPRNG**:
  - Mobile:
    - iOS: `SecRandomCopyBytes`
    - Android: `SecureRandom` backed by OS or libsodium’s RNG
  - Rust backend (for non‑secret IDs only): `rand::rngs::OsRng` or equivalent
- No use of `Math.random`, `Random()` without cryptographic backing, etc.

---

## 3. Cryptographic Primitives

### 3.1 Asymmetric

- Key exchange: **X25519** (Curve25519)
- Signatures: **Ed25519**

### 3.2 Symmetric

- KDF: **HKDF‑SHA256**
- AEAD for messages and attachments:
  - Preferred: **XChaCha20‑Poly1305**
  - Fallback (if platform constraints): **AES‑256‑GCM** with 96‑bit nonce

### 3.3 Password / PIN‑based KDF

- Use **Argon2id** with parameters (to be fine‑tuned with perf testing):
  - `t` (iterations): 3–5
  - `m` (memory): 64–256 MB (platform‑dependent)
  - `p` (parallelism): 1–4
- If Argon2id is unavailable, fallback MAY be PBKDF2‑HMAC‑SHA256 with high iteration count (>= 600k), but **must be explicitly documented**.

---

## 4. E2EE Messaging Protocol

### 4.1 Overview

We use a **Signal‑style protocol**:

- X3DH‑like handshake for initial session establishment
- Double Ratchet (DH + symmetric ratchet) for message key evolution
- Implementation:
  - **Production**: official `libsignal` (where possible)
  - Experimental / prototypes: Dart/other ports are allowed but MUST BE AUDITED before production

### 4.2 Key registration

On device registration:

1. Generate identity key pair (Ed25519).
2. Generate static X25519 key pair (SPK).
3. Generate signed prekey (SPK_s):
   - Generate X25519 key pair
   - Sign the public part using Ed25519 IK.
4. Generate N one‑time prekeys (OPK[i]) (e.g. 50–100).
5. Upload **prekey bundle** to backend:

```jsonc
{
  "user_id": "...",
  "device_id": "...",
  "identity_key_ed25519_b64": "...",
  "static_x25519_b64": "...",
  "signed_prekey_x25519_b64": "...",
  "signed_prekey_signature_b64": "...",
  "one_time_prekeys_b64": ["...", "..."]
}
```
Backend stores this bundle and one‑time prekeys in Postgres.

### 4.3 Session establishment (A → B)
When Alice (A) starts a conversation with Bob (B) for the first time:

1. A fetches B’s prekey bundle from backend.
2. A performs X3DH‑style ECDH combinations (using X25519):
   - ECDH1: A_ephemeral ⨂ IK_B
   - ECDH2: A_ephemeral ⨂ SPK_B
   - ECDH3: IK_A ⨂ SPK_B
   - ECDH4: A_ephemeral ⨂ OPK_B (if available)
3. Concatenate or HKDF‑combine the ECDH outputs into a shared secret SK.
4. Derive initial root key RK and chain keys using HKDF‑SHA256.
5. A then sends an initial “prekey message” to B, containing:
   - A’s identity key
   - A’s ephemeral public key
   - Identifier of which OPK_B was used (if any)
   - Ciphertext of the first message, encrypted under the derived message key

Backend:
- Delivers this as normal message envelope to B
- Marks the one‑time prekey as “used”

B:
- Looks up the correct prekey bundle entry.
- Performs the same ECDH operations to derive RK.
- Derives matching chain keys and decrypts the first message.

### 4.4 Double Ratchet
For each message after session setup:
- Each side maintains:
  - Root key (RK)
  - Sending chain key (CKs)
  - Receiving chain key (CKr)

For sending:
1. Derive new message key: MK = HKDF( CKs )
2. Advance CKs.
3. Encrypt message with AEAD (XChaCha20‑Poly1305) using MK and:
   - Nonce: random 192‑bit value
   - AAD: header (sender device_id, session id, counters)

For receiving:
1. Possibly perform DH ratchet step (if header indicates new DH key)
2. Advance CKr until the correct MK is found (within a reasonable skip window)
3. Decrypt ciphertext with MK and verify MAC.

Message header (unencrypted) SHOULD be minimal and not include plaintext identifiers beyond what is necessary.

### 4.5 Message format
Logical message object:
```json
{
  "session_id": "...",       // opaque UUID or hash
  "message_index": 42,       // per-session counter
  "sender_device_id": "...",
  "ciphertext_b64": "...",   // AEAD payload (XChaCha20-Poly1305)
  "nonce_b64": "...",
  "created_at_ms": 1234567890
}
```
Entire payload (except header) is AEAD‑encrypted with MK.
Attachments are handled separately as described below.

---

## 5. Attachments (Media) Encryption

### 5.1 Per‑attachment keys
Each attachment:
1. Generate a random 32‑byte attachment key K_att.
2. Encrypt file bytes with XChaCha20‑Poly1305:
   - Nonce: random 192‑bit nonce
   - AAD: version string + media type + message id (if available)
3. Store:
   - nonce
   - ciphertext
   - MAC (part of AEAD output)
4. Upload the encrypted blob to object storage (S3/MinIO/etc.) via presigned URL.

### 5.2 Attachment reference in message
The message body (encrypted as part of message payload) includes:
```json
{
  "attachments": [
    {
      "attachment_id": "uuid",
      "key_b64": "...",      // 32-byte key for this attachment
      "nonce_b64": "...",    // XChaCha20 nonce
      "size_bytes": 12345,
      "content_type": "image/jpeg"
    }
  ]
}
```
Backend only sees:
- Random object key for storage
- Encrypted blob
- Metadata necessary for delivery (size, type)

---

## 6. Local Key Hierarchy & Vault

### 6.1 Overview
Each device has:
- A Device Master Key (DMK) – 32 bytes
- A Vault Key (VK) – derived from DMK
- A Messaging Cache Key (MK_local) – derived from DMK

DMK is never stored in plaintext; it is wrapped using OS keystore.

### 6.2 Device Master Key (DMK)
On first secure setup:
1. Generate random 32‑byte DMK: DMK = Random(32)
2. Wrap DMK using:
   - iOS: Keychain with kSecAttrAccessibleWhenUnlockedThisDeviceOnly + Secure Enclave access control if available
   - Android: Android Keystore (StrongBox/TEE‑backed AES or RSA wrapping key)
3. Optionally further “lock” DMK with user PIN:
   - Derive K_PIN = Argon2id(PIN, salt)
   - Store encrypted DMK: Enc_K_PIN(DMK) in local DB
   - Use both OS keystore and PIN to unwrap

### 6.3 Vault Key (VK) and local message key (MK_local)
Derived from DMK:
```
VK       = HKDF-SHA256(DMK, info="speakeasy-vault-key", salt=nil)
MK_local = HKDF-SHA256(DMK, info="speakeasy-msg-cache-key", salt=nil)
```

### 6.4 Vault item encryption
Each vault item (file or note):
1. Generate random file key K_file (32 bytes).
2. Encrypt content with XChaCha20‑Poly1305 using K_file.
3. Encrypt K_file itself with VK using XChaCha20‑Poly1305 and a separate nonce.
4. Store on disk (or DB) as:
```json
{
  "version": 1,
  "file_nonce_b64": "...",
  "file_ciphertext_b64": "...",
  "file_mac_b64": "...",
  "wrapped_key_nonce_b64": "...",
  "wrapped_key_ciphertext_b64": "...",
  "wrapped_key_mac_b64": "..."
}
```

---

## 7. Backups & Recovery

### 7.1 Recovery mode options
We support two modes:

**No‑recovery mode (maximum local secrecy)**
- DMK exists only on device
- Loss of device = loss of local vault and cached messages

**Recovery‑phrase mode (optional)**
- User generates a Recovery Root Key (RRK) from a 12/16/24‑word phrase (BIP39‑style)
- RRK is used to encrypt a backup of DMK and/or identity keys

### 7.2 Recovery phrase and RRK
Use a BIP39‑like 12‑ or 24‑word mnemonic to derive RRK:
`RRK = HKDF-SHA256(Seed(mnemonic), info="speakeasy-rrk")`
**Never** send mnemonic or RRK to the server.

### 7.3 Backup format
If user enables recovery:
1. Encrypt DMK and optionally identity keys with RRK using XChaCha20‑Poly1305.
2. Upload encrypted backup blob to backend via authenticated API.

Example backup object:
```json
{
  "version": 1,
  "user_id": "...",
  "device_id": "...",
  "wrapped_dmk_nonce_b64": "...",
  "wrapped_dmk_ciphertext_b64": "...",
  "wrapped_dmk_mac_b64": "..."
}
```
Server cannot decrypt this without RRK.

### 7.4 Restore flow
1. User installs app on new device.
2. User enters recovery phrase.
3. App derives RRK and fetches backup from backend.
4. App decrypts DMK, re-establishes device identity and vault keys.
5. New prekey bundle is uploaded as part of “new device” registration.

---

## 8. Push Notifications

### 8.1 Payload constraints
No message content or sender names in push payload.
Minimal payload:
```json
{
  "type": "message",            // or "system"
  "user_id": "...",             // recipient
  "conversation_id": "hash",    // or opaque ID
  "event_id": "uuid"
}
```
On device, app will:
1. Fetch new messages via /messages/inbox
2. Decrypt locally
3. Show user notification based on privacy mode:
   - Generic title (e.g., “New activity”)
   - or no banner, just badge

### 8.2 Car / Public modes
In Car/Public mode:
- No previews in notification center
- Optionally: no notification banners at all
- All enforced locally based on user settings.

---

## 9. Key Rotation & Revocation

### 9.1 Identity key
Identity key is expected to be long‑lived.
If compromised:
- User must re‑register and re‑verify contacts.
- Old sessions are invalidated.

### 9.2 Signed prekey rotation
Signed prekey should be rotated at least:
- Every 30 days
- Or when user explicitly triggers “security refresh”
- Old signed prekeys kept for limited time to allow late messages; then deleted.

### 9.3 One‑time prekeys
One‑time prekeys are consumed on use.
When pool is low:
- Client generates new batch and uploads to server.

### 9.4 Device revocation
When a device is removed:
- Backend marks device as revoked
- Future prekey bundles exclude this device
- Client UI should show “device removed” in safety info

---

## 10. Implementation Requirements & Testing

### 10.1 Libraries
Wherever possible, use:
- Official libsignal for messaging protocol
- Well‑maintained crypto libraries (libsodium, BoringSSL wrappers, etc.)
- No “homegrown” implementations of low‑level primitives.

### 10.2 Side‑channel considerations
Avoid leaking secrets via:
- Logs
- Exceptions / stack traces
- UI debug overlays
Sensitive operations should avoid timing‑based leaks where feasible.

### 10.3 Testing
Required tests before production:
- **Unit tests** for:
  - KDFs
  - Local encryption/decryption of vault items
  - Backup/restore logic
  - Attachment encryption/decryption
- **Interop tests** between:
  - iOS app
  - Android app
  - Backend relay
- **Fuzz tests** for:
  - Message parsing
  - Attachment parsing
- **External security audit** of:
  - Use of libsignal (or alternative protocol implementation)
  - Local storage encryption
  - Backup/recovery cryptography

---

## 11. Documentation & Versioning

Any change to algorithms, parameters, or formats MUST bump:
- CryptoSpec version
- Data format version fields (e.g. vault item version)
Old data formats must be migrated or gracefully rejected.
