package com.speakeasy.crypto.vault

import android.util.Base64
import java.security.MessageDigest
import javax.crypto.Cipher
import javax.crypto.SecretKey
import javax.crypto.spec.GCMParameterSpec
import javax.crypto.spec.SecretKeySpec
import java.security.SecureRandom

// Encryption/decryption for attachments using AES-GCM

object AttachmentCrypto {
    
    // Encrypt data with key
    fun encrypt(plaintext: ByteArray, key: SecretKey): Pair<ByteArray, ByteArray> {
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        cipher.init(Cipher.ENCRYPT_MODE, key)
        val nonce = cipher.iv
        val ciphertext = cipher.doFinal(plaintext)
        return Pair(ciphertext, nonce)
    }
    
    // Decrypt data with key and nonce
    fun decrypt(ciphertext: ByteArray, nonce: ByteArray, key: SecretKey): ByteArray {
        val cipher = Cipher.getInstance("AES/GCM/NoPadding")
        cipher.init(Cipher.DECRYPT_MODE, key, GCMParameterSpec(128, nonce))
        return cipher.doFinal(ciphertext)
    }
    
    // Compute SHA256 hash (Base64)
    fun sha256Base64(data: ByteArray): String {
        val digest = MessageDigest.getInstance("SHA-256")
        return Base64.encodeToString(digest.digest(data), Base64.NO_WRAP)
    }
}
