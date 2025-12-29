# 🤝 HANDOFF TO ALEX & GPT FRONTEND TEAM

**From:** Backend/Crypto Team (AntiGravity + User)  
**To:** Frontend/UI Team (Alex + GPT)  
**Date:** 2025-12-29  
**Status:** Backend & Crypto COMPLETE - Ready for UI Integration

---

## 📋 EXECUTIVE SUMMARY

We have completed:
- ✅ **Phase 1**: Backend Relay (Rust/Axum) - LIVE & TESTED
- ✅ **Phase 2**: Signal Protocol Crypto (iOS + Android) - CODE COMPLETE
- ✅ **Phase 3**: Encrypted Vault & Attachments - CODE COMPLETE

**Your first task**: Run the verification tests to confirm everything compiles and works.

---

## 🚀 GETTING STARTED

### Prerequisites
- **macOS** with Xcode 15+ (for iOS)
- **Android Studio** Hedgehog+ with JDK 17 (for Android)
- **Docker Desktop** (for backend)

### Clone & Setup
```bash
git clone https://github.com/AnarchoFatSats/SPEAKEASY.git
cd SPEAKEASY
```

---

## 🔧 BACKEND SETUP (5 minutes)

### 1. Start Docker Services
```bash
cd infra
docker-compose up -d
```

### 2. Verify Services Running
```bash
docker ps
# Should see: postgres, redis, minio
```

### 3. Run Migrations (if first time)
```bash
# Migration 001 (initial schema)
Get-Content backend/migrations/001_init.sql | docker exec -i infra-postgres-1 psql -U speakeasy -d speakeasy

# Migration 002 (vault columns)
Get-Content backend/migrations/002_add_attachment_ciphertext.sql | docker exec -i infra-postgres-1 psql -U speakeasy -d speakeasy
```

### 4. Start Backend (Optional - for API testing)
```bash
cd backend
cargo run
# Server starts on http://localhost:3000
```

---

## 📱 iOS VERIFICATION

### 1. Open Project
```bash
cd mobile/ios/SpeakeasyCrypto
open Package.swift  # Opens in Xcode
```

### 2. Run Signal Protocol Test
In Xcode, create a test or add to AppDelegate:
```swift
import SpeakeasyCrypto

// In your test or app launch:
RoundtripRunner.run()
```

**Expected Output:**
```
🚀 Starting Roundtrip Test...
✅ User A Keys Generated
✅ User B Processed Bundle
✅ Encrypted: ...
✅ Decrypted: Hello Speakeasy!
🏆 ROUNDTRIP SUCCESS!
```

### 3. Run Vault Test
```swift
VaultTestRunner.run()
```

**Expected Output:**
```
🚀 Starting Vault Test...
✅ Plaintext: Hello Speakeasy Vault! 🔐
✅ Encrypted: XX bytes
✅ FileKey wrapped: XX bytes
✅ Saved to disk
✅ Saved to index
✅ Loaded from disk
✅ FileKey unwrapped
✅ Decrypted: Hello Speakeasy Vault! 🔐
🏆 VAULT TEST SUCCESS!
🧹 Cleaned up test data
```

---

## 🤖 ANDROID VERIFICATION

### 1. Open Project
- Open Android Studio
- File → Open → Select `mobile/android`
- Wait for Gradle sync

### 2. Add Gson Dependency (if missing)
In `crypto/build.gradle`, add:
```gradle
implementation 'com.google.code.gson:gson:2.10.1'
```

### 3. Run Vault Test
Create a simple test activity or instrumented test:
```kotlin
import com.speakeasy.crypto.vault.VaultTestRunner

// In your test:
val runner = VaultTestRunner(applicationContext)
runner.run()
```

Check Logcat for output with tag `VaultTestRunner`.

---

## 🔑 KEY FILES REFERENCE

### Backend (Rust)
| File | Purpose |
|------|---------|
| `backend/src/routes/auth_routes.rs` | Register/Login, JWT |
| `backend/src/routes/keys.rs` | Signal prekey bundles |
| `backend/src/routes/messages.rs` | Send/Inbox |
| `backend/src/routes/attachments.rs` | Presign/Complete |
| `openapi/openapi.yaml` | Full API spec |

### iOS Crypto
| File | Purpose |
|------|---------|
| `SignalClient.swift` | Main Signal facade |
| `SignalStoreAdapter.swift` | LibSignal store bridge |
| `KeychainStore.swift` | Secure key storage |
| `Vault/VaultKeyManager.swift` | DMK → VK derivation |
| `Vault/AttachmentCrypto.swift` | AES-GCM encryption |
| `Vault/VaultTestRunner.swift` | Test harness |

### Android Crypto
| File | Purpose |
|------|---------|
| `SignalClient.kt` | Main Signal facade |
| `SignalStoreAdapter.kt` | LibSignal store bridge |
| `KeystoreStore.kt` | Encrypted preferences |
| `vault/VaultKeyManager.kt` | DMK → VK derivation |
| `vault/AttachmentCrypto.kt` | AES-GCM encryption |
| `vault/VaultTestRunner.kt` | Test harness |

---

## 🎨 UI WORK FOR YOU

Once tests pass, you build:

### Phase 4: Core UI
1. **Login/Register Screen** - Call `auth_routes` endpoints
2. **Chat List Screen** - Display conversations
3. **Chat View Screen** - Messages with encryption via `SignalClient`
4. **Profile/Settings** - User preferences

### Phase 5: Vault UI
1. **Vault List** - Show encrypted items from `VaultIndexDb`
2. **Import File** - Use `AttachmentCrypto` + presign upload
3. **View File** - Download + decrypt

### Phase 6: Cover Mode
1. **Fake "Bar" Interface** - Decoy UI
2. **Quick Switch** - Panic button
3. **App Switcher Blur** - Privacy on background

---

## 🛡️ SECURITY NOTES

### What the Backend NEVER Sees
- Plaintext messages
- File contents
- Encryption keys
- User passwords (we use hashed identifiers)

### Key Hierarchy
```
DMK (Device Master Key) - Keychain/Keystore
    ↓ HKDF
VK (Vault Key) - Derived, never stored
    ↓ wrap/unwrap
FileKey (per file) - Random, wrapped with VK
```

### Signal Protocol Flow
```
1. Generate identity + prekeys → Upload to backend
2. Fetch recipient's bundle → Build session
3. Encrypt with SessionCipher → Send ciphertext
4. Recipient decrypts with their SessionCipher
```

---

## 📞 API QUICK REFERENCE

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/v1/auth/register` | POST | Create user |
| `/v1/auth/login` | POST | Get JWT |
| `/v1/keys/upload` | POST | Upload prekeys |
| `/v1/keys/bundle/{user_id}` | GET | Get user's bundle |
| `/v1/messages/send` | POST | Send message |
| `/v1/messages/inbox/{user_id}` | GET | Fetch messages |
| `/v1/attachments/presign` | POST | Get upload URL |
| `/v1/attachments/complete` | POST | Finalize upload |
| `/v1/attachments/url/{id}` | GET | Get download URL |

Full spec: `openapi/openapi.yaml`

---

## ✅ ACCEPTANCE CHECKLIST

Before starting UI work, confirm:

- [ ] Docker services running
- [ ] iOS `RoundtripRunner` passes
- [ ] iOS `VaultTestRunner` passes
- [ ] Android `VaultTestRunner` passes (Logcat)
- [ ] Backend `/health` returns 200

Once all pass, Phase 2 & 3 are **OFFICIALLY CLOSED**.

---

## 🏁 GIT STATE

- **Branch:** `main`
- **Latest Commit:** Check with `git log -1`
- **Repo:** `github.com/AnarchoFatSats/SPEAKEASY`

---

**Questions?** Refer to `docs/` folder or the OpenAPI spec.

**TOGETHER WE ARE INVINCIBLE!** 🛡️🚀
