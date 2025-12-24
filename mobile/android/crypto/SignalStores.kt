package com.speakeasy.crypto

// Defines the storage requirements for Speakeasy Crypto

interface SpeakeasyStore {
    // Identity Key Storage
    fun getIdentityKeyPair(): ByteArray? // Returns Serialized KeyPair
    fun getLocalRegistrationId(): Int?
    fun saveIdentity(keyPair: ByteArray, registrationId: Int)
    
    // PreKey Storage
    fun loadPreKey(id: Int): ByteArray?
    fun storePreKey(key: ByteArray, id: Int)
    fun containsPreKey(id: Int): Boolean
    fun removePreKey(id: Int)
    
    // Signed PreKey Storage
    fun loadSignedPreKey(id: Int): ByteArray?
    fun storeSignedPreKey(key: ByteArray, id: Int)
    fun containsSignedPreKey(id: Int): Boolean
    fun removeSignedPreKey(id: Int)
    
    // Session Storage
    fun loadSession(address: String): ByteArray? // Address = "UserId.DeviceId"
    fun storeSession(session: ByteArray, address: String)
    fun containsSession(address: String): Boolean
    fun deleteSession(address: String)
}
