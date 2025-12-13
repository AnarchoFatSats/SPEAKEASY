# Privacy Policy

**Effective Date:** 2025-12-12

Speakeasy ("we", "our", or "us") provides a private, encrypted communication service. This Privacy Policy describes how we handle—and more importantly, how we **cannot** handle—your data.

## 1. Our Core Principle: Zero Knowledge

We have designed our systems so that we **cannot** read your messages, view your photos, or access your private vault.
- **Messages:** End-to-End Encrypted (E2EE) using the Signal Protocol. Only you and the intended recipient have the keys to decrypt them.
- **Storage:** Your vault is encrypted with a key derived from your passphrase (which we never see).
- **Server Role:** Our server acts effectively as a "dumb relay," passing encrypted binary blobs between devices.

## 2. What Information We Collect

Because of our architecture, the data we possess is extremely limited:

### A. Account Information
- **User ID:** A random UUID assigned to your account.
- **Push Tokens:** Required to wake your device when a message arrives (e.g., APNS, FCM).
- **Public Identity Keys:** Cryptographic public keys needed for others to establish E2EE sessions with you.

### B. Transient Message Data
- **Encrypted Queues:** Undelivered messages sit in our queue as encrypted ciphertext. Once delivered to your device, they are deleted from our servers.
- **Metadata:** We store minimal routing logs (e.g., "Device A sent a message to Device B at Time T") to prevent abuse and ensure delivery.

### C. Usage Logs
- We log API access (IP address, endpoint, timestamp) for security and abuse prevention.
- **Retention:** These logs are retained for [30] days and then rotated/deleted.

## 3. What Information We Do NOT Collect
- **Message Content:** We never possess definitions of your messages.
- **Contact Lists:** Your contact graphs are stored encrypted on your device or via hashed matching (if enabled). We do not store a plaintext social graph.
- **Vault PINs/Passphrases:** If you lose your recovery phrase, we cannot restore your access.

## 4. How We Use Your Information
- To route messages to your device.
- To prevent spam and abuse (e.g., rate-limiting IPs).
- To comply with applicable laws (see Law Enforcement section).

## 5. Law Enforcement & Third Parties
We will comply with valid legal processes (e.g., subpoenas, court orders). However, because we do not hold the decryption keys:
> **We can only provide the encrypted ciphertext and metadata (who/when). We cannot produce message content.**

## 6. Your Rights
- **Deletion:** You may delete your account at any time. This wipes your identity keys and queued messages from our servers.
- **Access:** You can request a dump of your account data (which will mostly be meaningless keys and ciphertext).

## 7. Changes
We may update this policy. Significant changes will be notified via an in-app alert.
