import Foundation
import SignalClient

// Adapter that bridges our local SpeakeasyStore (Keychain) to LibSignal's Requirements

class SignalStoreAdapter: IdentityKeyStore, PreKeyStore, SessionStore, SignedPreKeyStore {
    
    private let store: SpeakeasyStore
    
    init(store: SpeakeasyStore) {
        self.store = store
    }
    
    // MARK: - IdentityKeyStore
    func getIdentityKeyPair() -> IdentityKeyPair {
        guard let data = store.getIdentityKeyPair() else {
            fatalError("Identity Key not generated yet!")
        }
        return try! IdentityKeyPair(from: data)
    }
    
    func getLocalRegistrationId() -> UInt32 {
        guard let id = store.getLocalRegistrationId() else {
            fatalError("Registration ID not generated yet!")
        }
        return id
    }
    
    func saveIdentity(_ identity: IdentityKeyPair?, registrationId: UInt32?) {
        // We usually generate initally, but this fulfills the protocol if needed.
        if let i = identity, let r = registrationId {
            store.saveIdentity(i.serialize(), registrationId: r)
        }
    }
    
    func isTrustedIdentity(_ identity: SignalClient.IdentityKey, isolationGroup: IsolationGroup?, direction: SignalClient.Direction, address: SignalClient.ProtocolAddress) -> Bool {
        // Trust On First Use (TOFU) or Always Trust for Phase 2
        return true
    }
    
    func getIdentity(address: SignalClient.ProtocolAddress) -> SignalClient.IdentityKey? {
        // We don't persist remote identities in Phase 2 yet (Trust all)
        return nil 
    }
    
    // MARK: - PreKeyStore
    func loadPreKey(id: UInt32) throws -> PreKeyRecord {
        guard let data = store.loadPreKey(id: id) else {
            throw SignalError.invalidKey
        }
        return try PreKeyRecord(from: data)
    }
    
    func storePreKey(_ record: PreKeyRecord, id: UInt32) throws {
        store.storePreKey(record.serialize(), id: id)
    }
    
    func containsPreKey(id: UInt32) -> Bool {
        return store.containsPreKey(id: id)
    }
    
    func removePreKey(id: UInt32) throws {
        store.removePreKey(id: id)
    }
    
    // MARK: - SignedPreKeyStore
    func loadSignedPreKey(id: UInt32) throws -> SignedPreKeyRecord {
        guard let data = store.loadSignedPreKey(id: id) else {
            throw SignalError.invalidKey
        }
        return try SignedPreKeyRecord(from: data)
    }
    
    func storeSignedPreKey(_ record: SignedPreKeyRecord, id: UInt32) throws {
        store.storeSignedPreKey(record.serialize(), id: id)
    }
    
    func containsSignedPreKey(id: UInt32) -> Bool {
        return store.containsSignedPreKey(id: id)
    }
    
    func removeSignedPreKey(id: UInt32) throws {
        store.removeSignedPreKey(id: id)
    }
    
    // MARK: - SessionStore
    func loadSession(address: SignalClient.ProtocolAddress) -> SessionRecord? {
        if let data = store.loadSession(address: address.name) {
             return try? SessionRecord(from: data)
        }
        return nil // Return nil or empty record depending on libsignal version
    }
    
    func storeSession(_ record: SessionRecord, address: SignalClient.ProtocolAddress) throws {
        store.storeSession(record.serialize(), address: address.name)
    }
    
    func containsSession(address: SignalClient.ProtocolAddress) -> Bool {
        return store.containsSession(address: address.name)
    }
    
    func deleteSession(address: SignalClient.ProtocolAddress) throws {
        store.deleteSession(address: address.name)
    }
}
