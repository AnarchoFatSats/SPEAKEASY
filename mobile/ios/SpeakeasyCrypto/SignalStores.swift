import Foundation
// Placeholder for LibSignal imports if available via SPM
// import SignalProtocol

// Defines the storage requirements for Speakeasy Crypto

protocol SpeakeasyStore {
    // Identity Key Storage
    func getIdentityKeyPair() -> Data? // Returns Serialized KeyPair
    func getLocalRegistrationId() -> UInt32?
    func saveIdentity(_ keyPair: Data, registrationId: UInt32)
    
    // PreKey Storage
    func loadPreKey(id: UInt32) -> Data?
    func storePreKey(_ key: Data, id: UInt32)
    func containsPreKey(id: UInt32) -> Bool
    func removePreKey(id: UInt32)
    
    // Signed PreKey Storage
    func loadSignedPreKey(id: UInt32) -> Data?
    func storeSignedPreKey(_ key: Data, id: UInt32)
    func containsSignedPreKey(id: UInt32) -> Bool
    func removeSignedPreKey(id: UInt32)
    
    // Session Storage
    func loadSession(address: String) -> Data? // Address = "UserId.DeviceId"
    func storeSession(_ session: Data, address: String)
    func containsSession(address: String) -> Bool
    func deleteSession(address: String)
}
