package com.speakeasy.crypto.vault

import android.content.Context
import android.util.Base64
import java.security.MessageDigest
import javax.crypto.Cipher
import javax.crypto.Mac
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec
import javax.crypto.spec.SecretKeySpec
import com.speakeasy.crypto.SpeakeasyStore

// Derives Vault Key (VK) from Device Master Key (DMK) via HKDF

class VaultKeyManager(private val store: SpeakeasyStore) {
    
    private val hkdfInfo = "speakeasy-vault-v1".toByteArray()
    private val hkdfSalt = "speakeasy".toByteArray()
    
    // DMK from identity key
    fun getDeviceMasterKey(): ByteArray? {
        val identityData = store.getIdentityKeyPair() ?: return null
        val digest = MessageDigest.getInstance("SHA-256")
        return digest.digest(identityData)
    }
    
    // Derive Vault Key via HKDF
    fun getVaultKey(): SecretKey? {
        val dmk = getDeviceMasterKey() ?: return null
        val derived = hkdf(dmk, hkdfSalt, hkdfInfo, 32)
        return SecretKeySpec(derived, "AES")
    }
    
    // Generate random FileKey
    fun generateFileKey(): SecretKey {
        val keyBytes = ByteArray(32)
        java.security.SecureRandom().nextBytes(keyBytes)
        return SecretKeySpec(keyBytes, "AES")
    }
    
    // Wrap FileKey with VaultKey (AES-GCM)
    fun wrapFileKey(fileKey: SecretKey): ByteArray {
        val vk = getVaultKey() ?: throw IllegalStateException("No Vault Key")
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        cipher.init(Cipher.ENCRYPT_MODE, vk)
        val iv = cipher.iv
        val encrypted = cipher.doFinal(fileKey.encoded)
        return iv + encrypted // 12-byte IV + ciphertext+tag
    }
    
    // Unwrap FileKey
    fun unwrapFileKey(wrapped: ByteArray): SecretKey {
        val vk = getVaultKey() ?: throw IllegalStateException("No Vault Key")
        val iv = wrapped.sliceArray(0 until 12)
        val ciphertext = wrapped.sliceArray(12 until wrapped.size)
        
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        cipher.init(Cipher.DECRYPT_MODE, vk, GCMParameterSpec(128, iv))
        val keyBytes = cipher.doFinal(ciphertext)
        return SecretKeySpec(keyBytes, "AES")
    }
    
    // Simple HKDF implementation (extract + expand)
    private fun hkdf(ikm: ByteArray, salt: ByteArray, info: ByteArray, outputLen: Int): ByteArray {
        val mac = Mac.getInstance("HmacSHA256")
        mac.init(SecretKeySpec(salt, "HmacSHA256"))
        val prk = mac.doFinal(ikm)
        
        mac.init(SecretKeySpec(prk, "HmacSHA256"))
        val result = ByteArray(outputLen)
        var t = ByteArray(0)
        var offset = 0
        var i = 1
        while (offset < outputLen) {
            mac.update(t)
            mac.update(info)
            mac.update(i.toByte())
            t = mac.doFinal()
            val toCopy = minOf(t.size, outputLen - offset)
            System.arraycopy(t, 0, result, offset, toCopy)
            offset += toCopy
            i++
        }
        return result
    }
}
