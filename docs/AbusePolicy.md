# Abuse & Moderation Policy

Speakeasy is built on privacy, but we do not tolerate abuse. This document outlines how we balance End-to-End Encryption (E2EE) with safety.

## 1. The Challenge of E2EE
In traditional apps, the server scans messages for bad words or images. In Speakeasy, **the server sees only opaque random bytes.** We cannot scan your messages.
Therefore, moderation relies on **User Reports** and **Metadata Analysis**.

## 2. Reporting Mechanism
If a user receives an abusive message:
1. They select "Block & Report".
2. Their device **re-encrypts** the offending message (and slightly prior context) with a key accessible to the Speakeasy Safety Team.
3. This report is uploaded to our server.
4. Our Safety Team reviews the *decrypted report* only. We never see the rest of your conversation.

## 3. Blocking
- **User Block:** Users can block you. This prevents your messages from being delivered to them.
- **Server Block:** If you are blocked by many users or reported frequently, our server may refuse to relay messages from your `device_id`.

## 4. Metadata Analysis (Spam Detection)
We monitor traffic patterns to stop automated spam, such as:
- Sending messages to X distinct recipients in Y minutes (high fan-out).
- Creating many accounts from a single IP address.
- Excessive attachment upload volume.

**Action:** Accounts triggering these rate limits may be temporarily locked or permanently banned.

## 5. CSAM (Child Sexual Abuse Material)
We have zero tolerance for CSAM.
- If a user report contains CSAM, we will ban the offender and refer the report (including the provided content and user identity info) to the NCMEC or relevant authorities.
