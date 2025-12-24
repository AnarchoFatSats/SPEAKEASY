import Foundation
import Security

class KeychainStore: SpeakeasyStore {
    
    // Service name for Keychain
    private let service = "com.speakeasy.crypto"
    
    // Helper: Save Data
    private func save(_ data: Data, account: String) {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: account,
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly
        ]
        
        SecItemDelete(query as CFDictionary) // Delete existing
        SecItemAdd(query as CFDictionary, nil)
    }
    
    // Helper: Load Data
    private func load(account: String) -> Data? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: account,
            kSecReturnData as String: true,
            kSecMatchLimit as String: kSecMatchLimitOne
        ]
        
        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        
        return (status == errSecSuccess) ? (result as? Data) : nil
    }
    
    // Identity
    func getIdentityKeyPair() -> Data? { return load(account: "identity_key_pair") }
    func getLocalRegistrationId() -> UInt32? { 
        guard let data = load(account: "registration_id") else { return nil }
        return data.withUnsafeBytes { $0.load(as: UInt32.self) }
    }
    func saveIdentity(_ keyPair: Data, registrationId: UInt32) {
        save(keyPair, account: "identity_key_pair")
        var regId = registrationId
        let data = Data(bytes: &regId, count: MemoryLayout<UInt32>.size)
        save(data, account: "registration_id")
    }
    
    // PreKeys (Simple key-value mapping for now)
    func loadPreKey(id: UInt32) -> Data? { return load(account: "prekey_\(id)") }
    func storePreKey(_ key: Data, id: UInt32) { save(key, account: "prekey_\(id)") }
    func containsPreKey(id: UInt32) -> Bool { return loadPreKey(id: id) != nil }
    func removePreKey(id: UInt32) {
         let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: "prekey_\(id)"
        ]
        SecItemDelete(query as CFDictionary)
    }
    
    // Signed PreKeys
    func loadSignedPreKey(id: UInt32) -> Data? { return load(account: "signed_prekey_\(id)") }
    func storeSignedPreKey(_ key: Data, id: UInt32) { save(key, account: "signed_prekey_\(id)") }
    func containsSignedPreKey(id: UInt32) -> Bool { return loadSignedPreKey(id: id) != nil }
    func removeSignedPreKey(id: UInt32) {
         let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: "signed_prekey_\(id)"
        ]
        SecItemDelete(query as CFDictionary)
    }
    
    // Sessions
    func loadSession(address: String) -> Data? { return load(account: "session_\(address)") }
    func storeSession(_ session: Data, address: String) { save(session, account: "session_\(address)") }
    func containsSession(address: String) -> Bool { return loadSession(address: address) != nil }
    func deleteSession(address: String) {
         let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: "session_\(address)"
        ]
        SecItemDelete(query as CFDictionary)
    }
}
