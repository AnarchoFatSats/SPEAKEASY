# ThreatModel – Speakeasy

Version: 1.0  
Status: Draft – review required before production  
Last updated: 2025‑12‑13

## 0. Overview

Speakeasy is a privacy‑focused communication and vault app with:

- End‑to‑end encrypted messaging
- Encrypted local vault
- Optional private SMS inbox on Android
- Cover UI + hidden Private Room

This document defines:

- What we are defending against
- What we are not defending against
- Concrete threats and mitigations

---

## 1. Assets

Primary assets we want to protect:

1. **Message content**
   - Text, attachments, voice notes
2. **Vault content**
   - Photos, videos, docs, notes
3. **Local SMS (Android private inbox)**
4. **Cryptographic keys**
   - Identity keys
   - Session keys
   - Vault keys
   - DMK / backup keys
5. **User metadata**
   - Who talks to whom
   - Timestamps
   - Device identifiers
6. **User credentials**
   - PIN / password
   - Recovery phrase (if enabled)

---

## 2. Adversaries

### 2.1 Casual local attacker

- Knows or guesses device PIN, or uses device while unlocked
- Tries to:
  - Browse apps
  - Look at recent apps
  - Glance at notifications
  - Open cover app and see if anything looks “suspicious”

**Mitigations:**

- Cover Mode looks like a normal, useful app (game/notes/etc.)
- Private Room is hidden behind secret sequence + biometric/PIN
- App switcher blur on Private Room screens
- Generic notifications (no message text, no contact names by default)
- Panic gesture to jump to cover UI quickly

---

### 2.2 Thief or opportunistic physical attacker

- Steals or finds device
- Does NOT know device PIN / biometrics
- Tries to:
  - Extract backups
  - Connect device to PC
  - Inspect on‑device storage

**Mitigations:**

- Rely on OS disk encryption (iOS/Android)
- DMK stored only in OS keystore, not in plaintext
- Vault and message caches encrypted with keys derived from DMK
- No server‑side keys able to decrypt backups/content

---

### 2.3 Curious or compromised server operator (“honest‑but‑curious backend”)

- Can read all database tables
- Can read all object storage blobs
- Cannot compromise client devices

**Mitigations:**

- All message content encrypted end‑to‑end with Signal‑style protocol
- Attachments encrypted with per‑attachment keys before upload
- Vault items never leave device without encryption
- Backups encrypted with Recovery Root Key (RRK), which server never sees
- Minimal metadata stored (no plaintext message text, no vault content)

---

### 2.4 Network adversary (passive)

- Controls or observes parts of the network between device and backend
- Can see IP addresses, routing info, timing

**Mitigations:**

- All communication over TLS (HTTPS/WSS)
- Application‑layer crypto (Signal protocol) on top of TLS
- No sensitive content in URLs or headers
- Push notifications carry no plaintext content

---

### 2.5 Malicious contact / abusive user

- Someone the user is talking to, who:
  - Sends spam or harassment
  - Tries to flood the user with messages
  - Attempts to manipulate safety features

**Mitigations:**

- Local block/mute controls per contact
- Server‑side rate limiting per user/IP
- Ability to disable accounts or throttle abusive senders (at messaging relay level)
- Optional “report” flow (user consents to sending decrypted samples to trust team)

---

### 2.6 Advanced attacker with compromised OS

- Has root/jailbreak level access
- Can inspect process memory, hook system APIs, read keystores

**Mitigations:**

- **Out of scope for strong guarantees**
- We may implement best‑effort hardening (root/jailbreak detection, warnings), but we do not claim to be secure against a compromised OS.

---

## 3. Non‑Goals (Explicitly Out of Scope)

Speakeasy does **not** attempt to:

1. Provide anonymity against a global passive network adversary
2. Hide the existence of network connections to Speakeasy servers
3. Hide carrier‑level call/SMS metadata (who called whom, when)
4. Defend against:
   - Kernel‑level malware
   - Hardware implants
   - Nation‑state capabilities targeting the OS or baseband
5. Prevent users from leaking their own data by:
   - Screenshots
   - Screen recording with another device
   - Manually copying and pasting content into other apps

---

## 4. Threats & Mitigations (Table)

| Threat | Description | Mitigation |
|-------|-------------|-----------|
| Lock‑screen preview leak | Notifications show contact + message | Generic notifications, optional “no banner” modes |
| App list spying | Recent apps preview shows private content | Blur app switcher for Private Room |
| Server breach (DB dump) | Attacker gets all tables | E2EE messaging, encrypted attachments, no plaintext vault in DB |
| Server breach (object storage) | Encrypted attachments leaked | Per‑attachment keys, stored only with clients |
| Lost device | Physical thief with no PIN | OS disk encryption, DMK in keystore, no plaintext vault/messages |
| Android SMS provider read | Other SMS apps read messages | Private numbers never written to Telephony provider, only to encrypted store |
| Abuse/spam | Contact floods messages | Rate limits, block/mute, account throttling |
| Recovery phrase theft | Phrase exposed or phished | UX warnings, user education, optional no‑recovery mode |

---

## 5. Residual Risks

Even with mitigations:

- A determined attacker with physical access and OS‑level exploits may capture secrets.
- Users may choose weak PINs or reuse passwords.
- Users may be tricked into revealing recovery phrase.
- Legal/government orders may require us to provide metadata and encrypted blobs (we cannot decrypt them, but this may still concern users).

These residual risks must be clearly described in user‑facing security documentation.

---

## 6. Review & Audit

Before production launch:

1. This Threat Model must be reviewed by:
   - At least one experienced security engineer
   - Product owner (to ensure claims match UX)
2. External security audit must:
   - Validate crypto usage matches `CryptoSpec.md`
   - Validate that logging and data retention follow policies
   - Attempt to exploit messaging, vault, and backup flows

All findings must be documented and fixed or accepted with clear rationale.
