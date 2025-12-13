# DataRetention – Speakeasy

Version: 1.0  
Status: Draft  
Last updated: 2025‑12‑13

## 1. Principles

- Retain as little data as possible for as short a time as possible.
- Provide users a clear understanding of what is retained and for how long.
- Align with legal requirements (GDPR, CCPA, etc.) where applicable.

---

## 2. Data Categories & Retention

### 2.1 Messages (Encrypted Envelopes)

- Content: E2EE ciphertext only (no plaintext)
- Stored on backend until:
  - Delivered and acknowledged by recipient devices, OR
  - A maximum retention window (e.g. 30 days) if not delivered
- After that:
  - Message envelopes are deleted from server.
- Local device copies are governed by user settings (disappearing messages, manual deletion, etc.)

### 2.2 Attachments (Encrypted Blobs)

- Stored in object storage as encrypted blobs.
- Retained until:
  - All referencing messages have been deleted AND
  - A grace period (e.g. 30–90 days) has elapsed, OR
  - User explicitly deletes thread/vault content referencing those attachments and confirms deletion.
- Regular cleanup job must remove orphaned attachments.

### 2.3 Vault Data

- Vault content is stored **only on device** (or in user‑encrypted backups).
- Server does not store plaintext vault content.
- If backups are enabled:
  - Encrypted backup blobs stored until user deletes account or disables backup.

### 2.4 Backups (Encrypted DMK / identity bundles)

- Stored as encrypted blobs, keyed by user id.
- Retained while user account is active and backup is enabled.
- Deleted:
  - When user disables backup
  - When user deletes account

### 2.5 Account & Device Metadata

- User profile:
  - `user_id`, hashed discovery identifiers (phone/email), display name, created_at
  - Retained while account is active, plus legal retention period after deletion.
- Devices:
  - `device_id`, associated keys (public), created_at, last_seen
  - Device entries removed when device is revoked or after account deletion.

### 2.6 Logs & Metrics

- Application logs:
  - 30–90 days
- Security logs (e.g. login failures):
  - Up to 180 days
- Aggregated metrics (non‑personal):
  - May be retained longer for capacity planning and performance tuning.

---

## 3. User‑Initiated Deletion

When user deletes account:

1. Mark account as “pending deletion”.
2. Immediately:
   - Remove access tokens / sessions
   - Remove active device entries
3. Within a defined time window (e.g. 7–30 days):
   - Delete:
     - Prekeys, public keys
     - Encrypted backups
     - Any undelivered message envelopes
4. Attachments and logs:
   - Purged according to category rules above.

We must document that **we cannot retroactively delete content that was already delivered and stored on other users’ devices**, as those are outside our control.

---

## 4. Legal Holds

If required by law:

- Certain data (e.g. account logs) may be held longer under legal hold.
- Even under legal hold:
  - We never have the keys to decrypt E2EE content.
  - Only encrypted blobs and metadata can be produced.

---

## 5. Updates

Any changes to retention periods or categories must:

- Be recorded in this document with version bump.
- Be reflected in the user‑facing Privacy Policy.
