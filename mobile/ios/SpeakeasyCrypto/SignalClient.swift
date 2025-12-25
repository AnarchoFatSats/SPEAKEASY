import Foundation
import SignalClient // Assumes module name from Package.swift

// The High-Level Client for Signal Operations

class SignalClient {
    private let store: SpeakeasyStore
    private let userId: String
    private let deviceId: String
    private let protocolStore: SignalProtocolStore // Hypothetical wrapper if needed, or we make SpeakeasyStore conform
    
    // Note: In a real implementation, SpeakeasyStore would conform to SignalProtocolStore.
    // For this scaffold, we'll assume we can pass our store context or that we are bridging manually.
    
    init(userId: String, deviceId: String, store: SpeakeasyStore) {
        self.userId = userId
        self.deviceId = deviceId
        self.store = store
        // self.protocolStore = store // If conforming
    }
    
    // 1. Setup / Registration
    func generateIdentity() throws {
        // Generate Keys
        let identityKeyPair = try IdentityKeyPair.generate()
        let registrationId = try RegistrationId.generate()
        
        // Save to Store
        store.saveIdentity(identityKeyPair.serialize(), registrationId: registrationId.id)
    }
    
    func generatePreKeys() throws -> [String: Any] {
        // 1. Generate One-Time PreKeys (e.g., 100 keys)
        let preKeys = try PreKeyRecord.generate(count: 100, offset: 0)
        
        // 2. Generate Signed PreKey
        let identityKP = try IdentityKeyPair(from: store.getIdentityKeyPair()!)
        let signedPreKey = try SignedPreKeyRecord.generate(
            id: 1, 
            identityKeyPair: identityKP, 
            timestamp: UInt64(Date().timeIntervalSince1970 * 1000)
        )
        
        // 3. Store Locally
        for key in preKeys {
            store.storePreKey(key.serialize(), id: key.id)
        }
        store.storeSignedPreKey(signedPreKey.serialize(), id: signedPreKey.id)
        
        // 4. Return Public Parts for API
        // In a real app, map these to the JSON structure expected by /v1/keys/upload
        return [
            "identity_key": identityKP.publicKey.serialize().base64EncodedString(),
            "signed_pre_key": signedPreKey.publicKey.serialize().base64EncodedString(),
            "signed_pre_key_sig": signedPreKey.signature.base64EncodedString(),
            "pre_keys": preKeys.map { $0.publicKey.serialize().base64EncodedString() }
        ]
    }
    
    // 2. Session Management
    func processPreKeyBundle(for remoteUserId: String, bundle: [String: Any]) throws {
        let address = ProtocolAddress(name: remoteUserId, deviceId: 1)
        
        // 1. Construct LibSignal PreKeyBundle
        // Note: Needs robust error handling for missing keys in production
        let preKeyBundle = try PreKeyBundle(
            registrationId:   UInt32(bundle["registration_id"] as! Int),
            deviceId:         UInt32(bundle["device_id"] as! Int),
            preKeyId:         UInt32(bundle["pre_key_id"] as! Int),
            preKeyPublic:     PublicKey(from: Data(base64Encoded: bundle["pre_key_public"] as! String)!),
            signedPreKeyId:   UInt32(bundle["signed_pre_key_id"] as! Int),
            signedPreKeyPublic: PublicKey(from: Data(base64Encoded: bundle["signed_pre_key_public"] as! String)!),
            signedPreKeySignature: Data(base64Encoded: bundle["signed_pre_key_signature"] as! String)!,
            identityKey:      IdentityKey(from: Data(base64Encoded: bundle["identity_key"] as! String)!)
        )
        
        // 2. Process
        let adapter = SignalStoreAdapter(store: self.store)
        let builder = try SessionBuilder(store: adapter, address: address)
        try builder.process(preKeyBundle: preKeyBundle)
    }
    
    // 3. Messaging
    func encrypt(to remoteAddress: String, plaintext: Data) throws -> String {
        let address = ProtocolAddress(name: remoteAddress, deviceId: 1)
        let adapter = SignalStoreAdapter(store: self.store)
        let cipher = try SessionCipher(store: adapter, address: address)
        
        let ciphertext = try cipher.encrypt(plaintext: plaintext)
        return ciphertext.serialize().base64EncodedString()
    }
    
    func decrypt(from remoteAddress: String, ciphertext: String) throws -> Data {
        let address = ProtocolAddress(name: remoteAddress, deviceId: 1)
        let adapter = SignalStoreAdapter(store: self.store)
        let cipher = try SessionCipher(store: adapter, address: address)
        
        let data = Data(base64Encoded: ciphertext)!
        return try cipher.decrypt(ciphertext: PreKeySignalMessage(from: data)) // Assuming PreKey msg for first message, or SignalMessage for subsequent
    }
}
