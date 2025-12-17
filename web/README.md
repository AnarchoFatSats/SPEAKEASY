# Speakeasy Web Client

Secure, privacy-focused web client for Speakeasy.

## Tech Stack
- **Framework:** Next.js (React)
- **Language:** TypeScript
- **Crypto:** `libsignal-protocol-javascript` (WASM preferred) + WebCrypto API
- **Storage:** IndexedDB (for encrypted keys and messages)

## Architecture
- **Zero-Knowledge:** The web client generates its own Identity Keys and Prekeys in the browser.
- **Storage:** Private keys are stored in IndexedDB, encrypted by a `Device Master Key (DMK)`.
- **Unlock:** The DMK is protected by a user passphrase (Argon2id derivation). The client must be "unlocked" to decrypt keys and session state.
- **Ephemeral:** Sensitive material is cleared from memory on tab close/inactivity.

## Build Setup
*(More details to follow in Phase 7)*
