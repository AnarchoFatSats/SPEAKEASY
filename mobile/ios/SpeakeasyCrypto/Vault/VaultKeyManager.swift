import Foundation
import CryptoKit

// Derives Vault Key (VK) from Device Master Key (DMK) via HKDF

class VaultKeyManager {
    
    private let store: SpeakeasyStore
    private let hkdfInfo = Data("speakeasy-vault-v1".utf8)
    private let hkdfSalt = Data("speakeasy".utf8) // Stable salt
    
    init(store: SpeakeasyStore) {
        self.store = store
    }
    
    // DMK should be stored in Keychain as our identity key or derived from it.
    // For Phase 3, we'll use the identity key itself as the DMK (or derive from it).
    func getDeviceMasterKey() -> SymmetricKey? {
        guard let identityData = store.getIdentityKeyPair() else { return nil }
        // Use first 32 bytes of identity key as DMK (or SHA256 of it)
        let hash = SHA256.hash(data: identityData)
        return SymmetricKey(data: Data(hash))
    }
    
    // Derive Vault Key from DMK
    func getVaultKey() -> SymmetricKey? {
        guard let dmk = getDeviceMasterKey() else { return nil }
        
        // HKDF derivation
        let vkData = HKDF<SHA256>.deriveKey(
            inputKeyMaterial: dmk,
            salt: hkdfSalt,
            info: hkdfInfo,
            outputByteCount: 32
        )
        return vkData
    }
    
    // Generate a random FileKey for each file/attachment
    func generateFileKey() -> SymmetricKey {
        return SymmetricKey(size: .bits256)
    }
    
    // Wrap (encrypt) a FileKey with the VaultKey
    func wrapFileKey(_ fileKey: SymmetricKey) throws -> Data {
        guard let vk = getVaultKey() else {
            throw NSError(domain: "VaultKeyManager", code: 1, userInfo: [NSLocalizedDescriptionKey: "No Vault Key"])
        }
        
        let fileKeyData = fileKey.withUnsafeBytes { Data($0) }
        let box = try AES.GCM.seal(fileKeyData, using: vk)
        return box.combined! // nonce + ciphertext + tag
    }
    
    // Unwrap (decrypt) a FileKey
    func unwrapFileKey(wrapped: Data) throws -> SymmetricKey {
        guard let vk = getVaultKey() else {
            throw NSError(domain: "VaultKeyManager", code: 1, userInfo: [NSLocalizedDescriptionKey: "No Vault Key"])
        }
        
        let box = try AES.GCM.SealedBox(combined: wrapped)
        let fileKeyData = try AES.GCM.open(box, using: vk)
        return SymmetricKey(data: fileKeyData)
    }
}
