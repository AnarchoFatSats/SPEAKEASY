# DataRetention (v1.0)

## Messaging
- Server stores encrypted message envelopes only.
- Retain undelivered messages for up to 30 days (configurable). After delivery, delete within 7 days.

## Attachments
- Store encrypted attachments in object storage.
- Retain until all recipients have downloaded OR until TTL (default 30 days) OR until sender deletes.

## Accounts
- Deleting account triggers:
  - revoke devices
  - delete public keys / bundles
  - delete message queues
  - delete attachment objects (best effort)

## Legal requests
- We can provide only:
  - account creation time
  - last seen time
  - device IDs
  - delivery metadata
- We cannot provide plaintext content because we do not possess keys.
