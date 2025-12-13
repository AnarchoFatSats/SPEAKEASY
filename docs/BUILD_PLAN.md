# Build Plan (Cursor / Team Execution)

This is the **ordered, dependency-aware** plan that an IDE agent (Cursor) can execute.

## Phase 0 — Decisions (lock these first)
1. **E2EE library choice**:
   - Recommended: **official libsignal** (Rust) + platform bindings (Swift/Kotlin) + Flutter FFI.
2. Identifier model:
   - v1: phone number or email-based account (simplify onboarding)
   - v2: add anonymous IDs (QR/invite) for maximum privacy
3. Backups:
   - v1: optional local export + encrypted cloud backup (zero-knowledge)
   - Decide whether you allow “no recovery / lose device lose data”.

## Phase 1 — Backend (dumb relay)
- Implement OpenAPI from `/openapi/openapi.yaml`
- Implement DB schema from `/backend/migrations/001_init.sql`
- Implement:
  - Auth (OTP or email)
  - Device registration
  - Prekey bundle upload / fetch
  - Message relay (encrypted envelopes)
  - Attachment upload (pre-signed URL)
  - Push token registration
- Add:
  - Rate limits (Redis)
  - Minimal logging policy enforcement

## Phase 2 — Crypto Core (mobile)
- Integrate libsignal and implement:
  - Identity keypair per device
  - Signed prekeys & one-time prekeys
  - Session creation (X3DH) and Double Ratchet
  - Per-message encryption & decryption
- Add identity verification UI (QR / safety number)

## Phase 3 — Private Room UX (mobile)
- Cover Mode UI (Speakeasy, Notes, etc.)
- Hidden unlock -> Private Room
- Private Circle (contacts)
- Messenger UI
- Vault UI
- Privacy Modes (Car/Work/Public)

## Phase 4 — Android default SMS mode (optional, Android-only)
- Implement role request to become default SMS app
- SMS_RECEIVED handling (only when default)
- Private numbers stored in encrypted local DB
- Public messages written to Telephony provider
- Private messages stored only in encrypted DB and shown only in Private Room
- Notification suppression/generic notifications for private numbers

## Phase 5 — Security hardening
- ThreatModel review
- Logging + retention review
- Pen test
- Supply chain checks, SBOM, dependency pinning
- Bug bounty once launched

## Phase 6 — App Store / Play Store packaging
- Use store-safe positioning: privacy + safety + secure storage + work/personal separation
- Avoid “cheating” framing.
- Document required permissions (Android SMS role)
