package com.speakeasy.crypto

// Import Signal Library (assumed available via Gradle)
import org.signal.libsignal.IdentityKeyPair
import org.signal.libsignal.IdentityKey
import org.signal.libsignal.state.PreKeyRecord
import org.signal.libsignal.state.SignedPreKeyRecord
import org.signal.libsignal.state.SignalProtocolStore
import org.signal.libsignal.SessionBuilder
import org.signal.libsignal.SessionCipher
import org.signal.libsignal.SignalProtocolAddress
import org.signal.libsignal.protocol.PreKeyBundle
import org.signal.libsignal.util.KeyHelper
import android.util.Base64

// The High-Level Client for Signal Operations

class SignalClient(
    private val userId: String,
    private val deviceId: String,
    private val store: SpeakeasyStore // Needs to adapt to SignalProtocolStore in real impl
) {
    
    // 1. Setup / Registration
    fun generateIdentity() {
        val identityKeyPair = KeyHelper.generateIdentityKeyPair()
        val registrationId = KeyHelper.generateRegistrationId(false)
        
        store.saveIdentity(identityKeyPair.serialize(), registrationId)
    }
    
    fun generatePreKeys(): Map<String, Any> {
        val identityBytes = store.getIdentityKeyPair() ?: throw IllegalStateException("No Identity")
        val identityKeyPair = IdentityKeyPair(identityBytes)
        
        // 1. One-Time PreKeys
        val preKeys = KeyHelper.generatePreKeys(0, 100)
        
        // 2. Signed PreKey
        val signedPreKey = KeyHelper.generateSignedPreKey(identityKeyPair, 1) // ID 1
        
        // 3. Store Locally
        for (key in preKeys) {
            store.storePreKey(key.serialize(), key.id)
        }
        store.storeSignedPreKey(signedPreKey.serialize(), signedPreKey.id)
        
        // 4. Return Public Parts (Map for easy JSON)
        return mapOf(
            "identity_key" to Base64.encodeToString(identityKeyPair.publicKey.serialize(), Base64.NO_WRAP),
            "signed_pre_key" to Base64.encodeToString(signedPreKey.keyPair.publicKey.serialize(), Base64.NO_WRAP),
            "signed_pre_key_sig" to Base64.encodeToString(signedPreKey.signature, Base64.NO_WRAP),
            "pre_keys" to preKeys.map { Base64.encodeToString(it.keyPair.publicKey.serialize(), Base64.NO_WRAP) }
        )
    }
    
    // 2. Session Management
    fun processPreKeyBundle(remoteUserId: String, bundle: Map<String, Any>) {
        val address = SignalProtocolAddress(remoteUserId, 1) // Default device 1
        
        // 1. Construct PreKeyBundle
        val preKeyBundle = PreKeyBundle(
            (bundle["registration_id"] as Number).toInt(),
            (bundle["device_id"] as Number).toInt(),
            (bundle["pre_key_id"] as Number).toInt(),
            KeyHelper.generatePublicKey(Base64.decode(bundle["pre_key_public"] as String, Base64.DEFAULT)),
            (bundle["signed_pre_key_id"] as Number).toInt(),
            KeyHelper.generatePublicKey(Base64.decode(bundle["signed_pre_key_public"] as String, Base64.DEFAULT)),
            Base64.decode(bundle["signed_pre_key_signature"] as String, Base64.DEFAULT),
            IdentityKey(Base64.decode(bundle["identity_key"] as String, Base64.DEFAULT))
        )
        
        // 2. Process
        val adapter = SignalStoreAdapter(store)
        val builder = SessionBuilder(adapter, address)
        builder.process(preKeyBundle)
    }
    
    // 3. Messaging
    fun encrypt(toRemoteAddress: String, plaintext: ByteArray): String {
        val address = SignalProtocolAddress(toRemoteAddress, 1)
        val adapter = SignalStoreAdapter(store)
        val cipher = SessionCipher(adapter, address)
        
        val ciphertext = cipher.encrypt(plaintext)
        return Base64.encodeToString(ciphertext.serialize(), Base64.NO_WRAP)
    }
    
    fun decrypt(fromRemoteAddress: String, ciphertext: String): ByteArray {
        val address = SignalProtocolAddress(fromRemoteAddress, 1)
        val adapter = SignalStoreAdapter(store)
        val cipher = SessionCipher(adapter, address)
        
        val bytes = Base64.decode(ciphertext, Base64.DEFAULT)
        // Try PreKeyMessage first (typical for 1st message), then SignalMessage
        // Note: Real impl needs try/catch or type check
        return cipher.decrypt(org.signal.libsignal.protocol.PreKeySignalMessage(bytes))
    }
}
