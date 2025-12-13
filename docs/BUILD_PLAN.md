# BUILD_PLAN – Speakeasy

Version: 1.0  
Status: Draft  
Last updated: 2025‑12‑13

This plan defines the recommended build order so that core security is implemented *before* UI polish and advanced features.

---

## Phase 0 – Read & Align (Docs)

**Goal:** Team + tools (Cursor/AntiGravity) fully understand architecture.

1. Read:
   - `docs/CryptoSpec.md`
   - `docs/ThreatModel.md`
   - `docs/LoggingPolicy.md`
   - `docs/DataRetention.md`
2. Confirm product decisions:
   - Identity model (phone/email/handle)
   - Recovery mode (no‑recovery vs recovery phrase)
   - Whether Android Default SMS ships in v1 or v2

---

## Phase 1 – Backend Relay Skeleton (Rust)

**Goal:** Have a running Rust Axum backend with DB and minimal endpoints.

Tasks:

1. Implement DB migrations in `backend/migrations/001_init.sql`.
2. Wire Postgres + SQLx in `backend/src/main.rs`.
3. Implement:
   - `GET /health`
   - `POST /v1/keys/upload`
   - `GET /v1/keys/bundle/:user_id`
   - `POST /v1/messages/send`
   - `GET /v1/messages/inbox/:user_id`
4. Add minimal authentication (e.g. bearer token stub) to secure endpoints.
5. Set up Docker Compose for local dev (Postgres, Redis, MinIO).

---

## Phase 2 – Mobile Crypto Core (libsignal + Vault)

**Goal:** Device can establish a secure session and encrypt/decrypt vault items locally.

Tasks:

1. Create mobile crypto module:
   - Key generation (Ed25519, X25519)
   - DMK + VK + MK_local per `CryptoSpec.md`
2. Integrate libsignal on at least one platform (Android or iOS):
   - Prekey bundle creation
   - Session establishment
   - Message encrypt/decrypt
3. Implement basic vault encryption:
   - Encrypt/decrypt a file or note with VK
   - Store in local sandbox
4. Implement minimal UI to:
   - Create account
   - Register keys via backend
   - Start 1:1 chat
   - Send & receive a single encrypted message

---

## Phase 3 – Attachments & Vault UX

**Goal:** Users can store media/files securely.

Tasks:

1. Implement attachment encryption per `CryptoSpec`:
   - Per‑attachment key
   - XChaCha20‑Poly1305
2. Implement backend:
   - Presigned upload URL
   - Attachment metadata table
3. Implement vault UI:
   - Grid/list of items
   - Detail view
   - Import from share sheet
4. Ensure app switcher blur and screen‑recording protection on vault screens.

---

## Phase 4 – Cover Mode & Private Room

**Goal:** Public‑facing cover UI + hidden private space.

Tasks:

1. Implement Cover Mode UI (Speakeasy bar / alternative themes).
2. Implement unlock sequence:
   - Bouncer + difficulty flow
   - Then biometric/PIN gate
3. Implement Private Room dashboard:
   - Private contacts
   - Chats
   - Vault
   - Modes (Car/Work/Public, initially simple toggles)

---

## Phase 5 – Push, Notifications & Modes

**Goal:** Background message delivery that respects privacy modes.

Tasks:

1. Implement backend push gateway:
   - FCM + APNs integration
   - Minimal payloads (no content preview)
2. Implement client push handling:
   - On push → fetch `/messages/inbox` → decrypt → update UI
3. Implement notification behavior:
   - Generic notifications by default
   - Stricter behavior in Car/Work/Public modes
4. Implement Car Mode automation:
   - Android: BT connection triggers
   - iOS: Shortcuts actions that user can wire to CarPlay events

---

## Phase 6 – Android Default SMS (Optional / Advanced)

**Goal:** Private SMS inbox on Android.

Tasks:

1. Implement SMS role request & receiver (in `android_sms_plugin_template`).
2. Implement private/public routing:
   - For private numbers → store ciphertext in local encrypted DB only
   - For normal numbers → insert into Telephony provider as usual
3. Integrate with vault key hierarchy for private SMS storage.
4. Build UI for Private Inbox (SMS) inside Private Room.
5. Ensure Play Store compliance for SMS permissions.

---

## Phase 7 – Monetization & Harden

**Goal:** Make app production‑ready.

Tasks:

1. Implement subscription tiers (Free / Standard / Pro).
2. Add:
   - Block/mute contacts
   - Basic abuse reporting
   - Rate limiting on backend
3. Add analytics (privacy‑respecting, aggregated only).
4. Run:
   - Security code review
   - External audit on crypto and backend
5. Prepare:
   - Privacy Policy
   - Terms of Service
   - Store listing text consistent with Threat Model.
