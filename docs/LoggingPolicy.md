# LoggingPolicy (v1.0)

## Principles
- Logs are for **reliability**, not surveillance.
- **Never log plaintext** message content or attachment content.
- Minimize identifiers: avoid storing phone numbers or emails in logs; use hashed IDs.

## Backend logging
Allowed fields:
- request_id
- route name
- response status
- latency
- coarse error codes (e.g., PREKEY_NOT_FOUND)

Forbidden fields:
- ciphertext blobs (unless explicitly sampled for debugging behind a feature flag and immediately deleted)
- phone numbers, emails, usernames in plaintext
- push tokens in plaintext (store only last 6 chars if needed)

## Retention
- Application logs: 7–14 days (short)
- Security audit events (minimal): 90 days
- Message envelopes: delete after delivered + grace window (e.g., 30 days max), configurable

## Access
- Logs accessible only to on-call engineers with 2FA + least privilege
- Production logs are immutable append-only storage (e.g., CloudWatch with retention lock)

## Client logging
- Default off for sensitive modules.
- Provide user-controlled “diagnostic mode” that redacts identifiers.
