package com.speakeasy.crypto

import org.signal.libsignal.IdentityKey
import org.signal.libsignal.IdentityKeyPair
import org.signal.libsignal.SignalProtocolAddress
import org.signal.libsignal.state.PreKeyRecord
import org.signal.libsignal.state.SessionRecord
import org.signal.libsignal.state.SignalProtocolStore
import org.signal.libsignal.state.SignedPreKeyRecord
import java.io.IOException

class SignalStoreAdapter(private val store: SpeakeasyStore) : SignalProtocolStore {

    // IdentityKeyStore
    override fun getIdentityKeyPair(): IdentityKeyPair {
        val data = store.getIdentityKeyPair() ?: throw RuntimeException("No Identity Key")
        return IdentityKeyPair(data)
    }

    override fun getLocalRegistrationId(): Int {
        return store.getLocalRegistrationId() ?: throw RuntimeException("No Registration ID")
    }

    override fun saveIdentity(address: SignalProtocolAddress, identityKey: IdentityKey): Boolean {
        // TOFU: Always trust for now
        return true
    }

    override fun isTrustedIdentity(address: SignalProtocolAddress, identityKey: IdentityKey, direction: IdentityKeyStore.Direction): Boolean {
        return true
    }

    override fun getIdentity(address: SignalProtocolAddress): IdentityKey? {
        // We don't persist remote identities yet
        return null
    }

    // PreKeyStore
    override fun loadPreKey(preKeyId: Int): PreKeyRecord {
        val data = store.loadPreKey(preKeyId) ?: throw org.signal.libsignal.InvalidKeyIdException("PreKey not found")
        return PreKeyRecord(data)
    }

    override fun storePreKey(preKeyId: Int, record: PreKeyRecord) {
        store.storePreKey(record.serialize(), preKeyId)
    }

    override fun containsPreKey(preKeyId: Int): Boolean {
        return store.containsPreKey(preKeyId)
    }

    override fun removePreKey(preKeyId: Int) {
        store.removePreKey(preKeyId)
    }

    // SignedPreKeyStore
    override fun loadSignedPreKey(signedPreKeyId: Int): SignedPreKeyRecord {
        val data = store.loadSignedPreKey(signedPreKeyId) ?: throw org.signal.libsignal.InvalidKeyIdException("SignedPreKey not found")
        return SignedPreKeyRecord(data)
    }

    override fun loadSignedPreKeys(): List<SignedPreKeyRecord> {
        return emptyList() // Not used in basic flow
    }

    override fun storeSignedPreKey(signedPreKeyId: Int, record: SignedPreKeyRecord) {
        store.storeSignedPreKey(record.serialize(), signedPreKeyId)
    }

    override fun containsSignedPreKey(signedPreKeyId: Int): Boolean {
        return store.containsSignedPreKey(signedPreKeyId)
    }

    override fun removeSignedPreKey(signedPreKeyId: Int) {
        store.removeSignedPreKey(signedPreKeyId)
    }

    // SessionStore
    override fun loadSession(address: SignalProtocolAddress): SessionRecord {
        val data = store.loadSession(address.toString())
        return if (data != null) SessionRecord(data) else SessionRecord()
    }

    override fun getSubDeviceSessions(name: String): List<Int> {
        return emptyList() 
    }

    override fun storeSession(address: SignalProtocolAddress, record: SessionRecord) {
        store.storeSession(record.serialize(), address.toString())
    }

    override fun containsSession(address: SignalProtocolAddress): Boolean {
        return store.containsSession(address.toString())
    }

    override fun deleteSession(address: SignalProtocolAddress) {
        store.deleteSession(address.toString())
    }

    override fun deleteAllSessions(name: String) {
        // No-op
    }
}
