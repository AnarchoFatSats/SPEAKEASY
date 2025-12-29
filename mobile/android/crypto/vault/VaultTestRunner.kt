package com.speakeasy.crypto.vault

import android.content.Context
import android.util.Log

// Test harness for Vault operations (Import -> Encrypt -> Store -> Load -> Decrypt)

class VaultTestRunner(private val context: Context) {
    
    private val TAG = "VaultTestRunner"
    
    fun run() {
        Log.d(TAG, "🚀 Starting Vault Test...")
        
        val store = com.speakeasy.crypto.KeystoreStore(context)
        val keyManager = VaultKeyManager(store)
        val fileStore = VaultFileStore(context)
        val indexDb = VaultIndexDb(context)
        
        try {
            // 0. Ensure identity exists (for DMK derivation)
            if (store.getIdentityKeyPair() == null) {
                Log.w(TAG, "⚠️ No identity key found. Generating test identity...")
                // Generate a fake identity for testing
                val fakeIdentity = ByteArray(32)
                java.security.SecureRandom().nextBytes(fakeIdentity)
                store.saveIdentity(fakeIdentity, 12345)
            }
            
            // 1. Create test plaintext
            val testPlaintext = "Hello Speakeasy Vault! 🔐".toByteArray(Charsets.UTF_8)
            Log.d(TAG, "✅ Plaintext: ${String(testPlaintext)}")
            
            // 2. Generate FileKey and encrypt
            val fileKey = keyManager.generateFileKey()
            val (ciphertext, nonce) = AttachmentCrypto.encrypt(testPlaintext, fileKey)
            Log.d(TAG, "✅ Encrypted: ${ciphertext.size} bytes")
            
            // 3. Wrap FileKey
            val wrappedKey = keyManager.wrapFileKey(fileKey)
            Log.d(TAG, "✅ FileKey wrapped: ${wrappedKey.size} bytes")
            
            // 4. Save to disk
            val itemId = java.util.UUID.randomUUID()
            fileStore.save(ciphertext, itemId)
            Log.d(TAG, "✅ Saved to disk: $itemId")
            
            // 5. Save to index
            val vaultItem = VaultItem(
                id = itemId,
                type = VaultItemType.NOTE,
                createdAt = java.util.Date(),
                updatedAt = java.util.Date(),
                wrappedFileKeyB64 = android.util.Base64.encodeToString(wrappedKey, android.util.Base64.NO_WRAP),
                nonceB64 = android.util.Base64.encodeToString(nonce, android.util.Base64.NO_WRAP),
                ciphertextPath = "${itemId}.enc",
                attachmentId = null,
                mimeType = "text/plain",
                ciphertextSizeBytes = ciphertext.size.toLong(),
                displayNameEnc = null
            )
            indexDb.insert(vaultItem)
            Log.d(TAG, "✅ Saved to index")
            
            // 6. Simulate "reboot" - load from disk
            val loadedItem = indexDb.get(itemId)!!
            val loadedCiphertext = fileStore.load(itemId)
            Log.d(TAG, "✅ Loaded from disk: ${loadedCiphertext.size} bytes")
            
            // 7. Unwrap FileKey
            val unwrappedKey = keyManager.unwrapFileKey(
                android.util.Base64.decode(loadedItem.wrappedFileKeyB64, android.util.Base64.NO_WRAP)
            )
            Log.d(TAG, "✅ FileKey unwrapped")
            
            // 8. Decrypt
            val loadedNonce = android.util.Base64.decode(loadedItem.nonceB64, android.util.Base64.NO_WRAP)
            val decrypted = AttachmentCrypto.decrypt(loadedCiphertext, loadedNonce, unwrappedKey)
            val decryptedString = String(decrypted, Charsets.UTF_8)
            Log.d(TAG, "✅ Decrypted: $decryptedString")
            
            // 9. Verify
            if (decryptedString == "Hello Speakeasy Vault! 🔐") {
                Log.d(TAG, "🏆 VAULT TEST SUCCESS!")
            } else {
                Log.e(TAG, "❌ MISMATCH")
            }
            
            // Cleanup
            fileStore.delete(itemId)
            indexDb.delete(itemId)
            Log.d(TAG, "🧹 Cleaned up test data")
            
        } catch (e: Exception) {
            Log.e(TAG, "❌ ERROR: ${e.message}", e)
        }
    }
}
