import Foundation

// Test harness for Vault operations (Import -> Encrypt -> Store -> Load -> Decrypt)

class VaultTestRunner {
    
    static func run() {
        print("🚀 Starting Vault Test...")
        
        let store = KeychainStore()
        let keyManager = VaultKeyManager(store: store)
        let fileStore = VaultFileStore()
        let indexDb = VaultIndexDb()
        
        do {
            // 0. Ensure identity exists (for DMK derivation)
            if store.getIdentityKeyPair() == nil {
                print("⚠️ No identity key found. Run RoundtripRunner first to generate identity.")
                return
            }
            
            // 1. Create test plaintext
            let testPlaintext = "Hello Speakeasy Vault! 🔐".data(using: .utf8)!
            print("✅ Plaintext: \(String(data: testPlaintext, encoding: .utf8)!)")
            
            // 2. Generate FileKey and encrypt
            let fileKey = keyManager.generateFileKey()
            let (ciphertext, nonce) = try AttachmentCrypto.encrypt(plaintext: testPlaintext, key: fileKey)
            print("✅ Encrypted: \(ciphertext.count) bytes")
            
            // 3. Wrap FileKey
            let wrappedKey = try keyManager.wrapFileKey(fileKey)
            print("✅ FileKey wrapped: \(wrappedKey.count) bytes")
            
            // 4. Save to disk
            let itemId = UUID()
            let _ = try fileStore.save(ciphertext: ciphertext, id: itemId)
            print("✅ Saved to disk: \(itemId)")
            
            // 5. Save to index
            let vaultItem = VaultItem(
                id: itemId,
                type: .note,
                createdAt: Date(),
                updatedAt: Date(),
                wrappedFileKeyB64: wrappedKey.base64EncodedString(),
                nonceB64: nonce.base64EncodedString(),
                ciphertextPath: "\(itemId).enc",
                attachmentId: nil,
                mimeType: "text/plain",
                ciphertextSizeBytes: Int64(ciphertext.count),
                displayNameEnc: nil
            )
            indexDb.insert(vaultItem)
            print("✅ Saved to index")
            
            // 6. Simulate "reboot" - load from disk
            let loadedItem = indexDb.get(itemId)!
            let loadedCiphertext = try fileStore.load(id: itemId)
            print("✅ Loaded from disk: \(loadedCiphertext.count) bytes")
            
            // 7. Unwrap FileKey
            let unwrappedKey = try keyManager.unwrapFileKey(wrapped: Data(base64Encoded: loadedItem.wrappedFileKeyB64!)!)
            print("✅ FileKey unwrapped")
            
            // 8. Decrypt
            let loadedNonce = Data(base64Encoded: loadedItem.nonceB64)!
            let decrypted = try AttachmentCrypto.decrypt(ciphertext: loadedCiphertext, nonce: loadedNonce, key: unwrappedKey)
            let decryptedString = String(data: decrypted, encoding: .utf8)!
            print("✅ Decrypted: \(decryptedString)")
            
            // 9. Verify
            if decryptedString == "Hello Speakeasy Vault! 🔐" {
                print("🏆 VAULT TEST SUCCESS!")
            } else {
                print("❌ MISMATCH")
            }
            
            // Cleanup
            try fileStore.delete(id: itemId)
            indexDb.delete(itemId)
            print("🧹 Cleaned up test data")
            
        } catch {
            print("❌ ERROR: \(error)")
        }
    }
}
