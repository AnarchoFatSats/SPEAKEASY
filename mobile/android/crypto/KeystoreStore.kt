package com.speakeasy.crypto

import android.content.Context
import android.content.SharedPreferences
import android.util.Base64
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey

class KeystoreStore(context: Context) : SpeakeasyStore {
    
    // GPT REQUIREMENT: Use EncryptedSharedPreferences for production security.
    private val masterKey = MasterKey.Builder(context)
        .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
        .build()

    private val prefs: SharedPreferences = EncryptedSharedPreferences.create(
        context,
        "speakeasy_secure_store",
        masterKey,
        EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
        EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM
    )
    
    // Identity
    override fun getIdentityKeyPair(): ByteArray? {
        val b64 = prefs.getString("identity_key_pair", null) ?: return null
        return Base64.decode(b64, Base64.DEFAULT)
    }
    
    override fun getLocalRegistrationId(): Int? {
        if (!prefs.contains("registration_id")) return null
        return prefs.getInt("registration_id", 0)
    }
    
    override fun saveIdentity(keyPair: ByteArray, registrationId: Int) {
        prefs.edit()
            .putString("identity_key_pair", Base64.encodeToString(keyPair, Base64.DEFAULT))
            .putInt("registration_id", registrationId)
            .apply()
    }
    
    // PreKeys
    override fun loadPreKey(id: Int): ByteArray? {
        val b64 = prefs.getString("prekey_$id", null) ?: return null
        return Base64.decode(b64, Base64.DEFAULT)
    }
    
    override fun storePreKey(key: ByteArray, id: Int) {
        prefs.edit().putString("prekey_$id", Base64.encodeToString(key, Base64.DEFAULT)).apply()
    }
    
    override fun containsPreKey(id: Int): Boolean { return prefs.contains("prekey_$id") }
    override fun removePreKey(id: Int) { prefs.edit().remove("prekey_$id").apply() }
    
    // Signed PreKeys
    override fun loadSignedPreKey(id: Int): ByteArray? {
        val b64 = prefs.getString("signed_prekey_$id", null) ?: return null
        return Base64.decode(b64, Base64.DEFAULT)
    }
    
    override fun storeSignedPreKey(key: ByteArray, id: Int) {
        prefs.edit().putString("signed_prekey_$id", Base64.encodeToString(key, Base64.DEFAULT)).apply()
    }
    
    override fun containsSignedPreKey(id: Int): Boolean { return prefs.contains("signed_prekey_$id") }
    override fun removeSignedPreKey(id: Int) { prefs.edit().remove("signed_prekey_$id").apply() }
    
    // Sessions
    override fun loadSession(address: String): ByteArray? {
        val b64 = prefs.getString("session_$address", null) ?: return null
        return Base64.decode(b64, Base64.DEFAULT)
    }
    
    override fun storeSession(session: ByteArray, address: String) {
        prefs.edit().putString("session_$address", Base64.encodeToString(session, Base64.DEFAULT)).apply()
    }
    
    override fun containsSession(address: String): Boolean { return prefs.contains("session_$address") }
    override fun deleteSession(address: String) { prefs.edit().remove("session_$address").apply() }
}
