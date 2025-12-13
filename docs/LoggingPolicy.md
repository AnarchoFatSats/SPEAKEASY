# LoggingPolicy – Speakeasy

Version: 1.0  
Status: Draft  
Last updated: 2025‑12‑13

## 1. Goals

- Collect enough logs to operate, debug, and secure the service.
- Avoid logging any user‑provided sensitive content.
- Avoid logging enough metadata to rebuild full social graphs.

---

## 2. Must NOT Log

The backend and clients **MUST NOT** log:

- Message plaintext
- Attachment plaintext or decrypted bytes
- Vault content names or data
- Recovery phrases or any derivation (RRK, K_backup, DMK)
- User PINs/passwords
- Full phone numbers or emails in plaintext outside of strictly controlled components
- Cryptographic key material (identity keys, prekeys, DMK, VK, MK_local)
- Authentication tokens (full values)

If any of the above accidentally appears in logs (e.g. due to a bug), logs must be treated as a security incident.

---

## 3. May Log (Limited)

The backend MAY log:

- Request paths and HTTP method (e.g. `POST /v1/messages/send`)
- Result status (2xx, 4xx, 5xx)
- Anonymized user identifiers (e.g. truncated/hashed `user_id`)
- Error codes (e.g. “DB_TIMEOUT”, “INVALID_PAYLOAD”)
- Rate limit events (user/IP exceeded limit)
- Operational metrics (latency, counts, memory usage)

No logs should contain full user identifiers if not strictly necessary; where included, they should be truncated or hashed.

---

## 4. Log Retention

- Application logs:
  - Retained for **30–90 days** (exact value defined in `DataRetention.md`)
  - Older logs automatically deleted
- Access logs (e.g. reverse proxy):
  - May be retained longer for security (e.g. up to 180 days), but:
    - Must not include sensitive headers
    - Must not include query params with secrets

---

## 5. Client‑Side Logging

- Release builds of mobile apps:
  - Log minimal info, with log level “warning” or higher
  - No user content in logs
- Debug builds:
  - May log more details, but:
    - Must be opt‑in
    - Must never upload logs automatically without user consent

---

## 6. Centralized Logging

If using centralized logging (e.g. ELK, CloudWatch):

- Restricted access by role (only operations and security staff)
- TLS enforced for log ingestion
- Logs encrypted at rest

---

## 7. Incident Handling

If a misconfiguration or bug causes sensitive data to be logged:

1. Immediately stop log ingestion to prevent further leakage.
2. Rotate any affected secrets/tokens.
3. Delete or archive logs according to incident response plan.
4. Document incident and remediation.
