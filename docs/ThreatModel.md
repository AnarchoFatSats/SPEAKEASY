# ThreatModel (v1.0) — Speakeasy Privacy Suite

## 1. Assets (what we protect)
- Message plaintext
- Attachment plaintext
- Vault contents (photos/videos/files/notes)
- Private contact list inside app
- Device identity keys and session keys
- Minimal metadata (who talks to whom, when)

## 2. Adversaries
A1. Casual snooper with temporary access to an unlocked phone (friend, coworker, shared environment)
A2. Thief with physical possession of phone (locked)
A3. Remote attacker with network visibility (WiFi MITM)
A4. Backend attacker who obtains DB + object storage dump
A5. Malicious push provider observer (can see push metadata)
A6. Malware on device / rooted or jailbroken device (high capability)
A7. Malicious insider with access to logs/admin tooling

## 3. Trust boundaries
- Device (trusted for crypto operations, but may be lost)
- Backend (untrusted for content; trusted for availability, routing, rate limits)
- Push providers (untrusted for content; must see routing metadata)
- OS-level apps (Messages/Phone/Photos) are out of scope for interception

## 4. Threats & mitigations

### T1: Snooper opens the app
Mitigations:
- Private Room requires secret unlock + optional biometric/PIN
- Auto-lock on background, short idle timeout
- App switcher privacy (blur/snapshot)
- Panic return to Cover Mode

### T2: Notifications expose sensitive content
Mitigations:
- Notification modes: none / generic / badge-only
- Never include plaintext in push payload
- iOS: default to “generic” notifications; encourage system setting “Show Previews: When Unlocked”
- Android: per-thread notification channels; private contacts can be silent

### T3: Backend breach reveals messages
Mitigations:
- E2EE: server stores only ciphertext
- Attachments encrypted client-side
- Key material never on server
- Rotate secrets; least privilege IAM; encrypted disks

### T4: MITM or TLS interception
Mitigations:
- TLS 1.2+ everywhere
- Certificate pinning (careful with rotation strategy)
- Strict transport security; disable cleartext traffic on Android

### T5: Device stolen
Mitigations:
- Hardware-backed key storage
- Vault master key bound to device + user auth
- Wipe-on-failed-attempts (optional)
- Remote “device revoke” (server blocks further delivery to lost device)

### T6: Spam/abuse in messenger
Mitigations:
- Rate limits by IP + account + device
- Block/mute lists stored locally + optionally server-side for routing
- Report flow: user can opt-in to forward offending ciphertext and decrypted copy to trust team
- Account disable & device ban policies

### T7: Metadata leakage (who talks to whom)
Mitigations:
- Minimize: store only device IDs and conversation IDs
- Optionally hash phone/email identifiers
- Short retention on message envelopes after delivery
- Consider sealed-sender style improvements in v2

### T8: Rooted device / spyware
Mitigations:
- Detect jailbreak/root and warn
- Optional “high risk mode” that disables screenshots, blocks clipboard copy, and reduces on-screen content
- Document this as out-of-scope for strong guarantees

## 5. Security testing plan
- SAST:
  - Rust: clippy, cargo-audit
  - Android: lint, detekt
  - Flutter: static analysis
- DAST:
  - API fuzzing on message endpoints
- Crypto:
  - test vectors
  - protocol-level tests via libsignal test suite
- Pen test:
  - before public launch
- Bug bounty:
  - after launch and initial hardening

## 6. What we explicitly do NOT claim (Out of Scope)
- **Compromised OS:** If the user's device is Rooted, Jailbroken, or infected with spyware (Pegasus, keyloggers), **Speakeasy cannot protect you.** The OS wins.
- **Shoulder Surfing:** If someone physically watches you type your PIN, game over.
- **Legal Coercion:** If a user is legally forced to unlock their device (biometrics/PIN), we cannot prevent access.
- **Deception:** We do NOT claim to hide the *existence* of the app from forensic analysis. A forensic scan of the phone will reveal "Speakeasy" is installed.
