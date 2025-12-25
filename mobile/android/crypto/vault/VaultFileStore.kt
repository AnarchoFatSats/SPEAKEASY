package com.speakeasy.crypto.vault

import android.content.Context
import java.io.File
import java.util.UUID

// Manages encrypted files on disk

class VaultFileStore(context: Context) {
    
    private val vaultDir: File = File(context.filesDir, "SpeakeasyVault").apply { mkdirs() }
    
    // Save encrypted data
    fun save(ciphertext: ByteArray, id: UUID): File {
        val file = File(vaultDir, "${id}.enc")
        file.writeBytes(ciphertext)
        return file
    }
    
    // Load encrypted data
    fun load(id: UUID): ByteArray {
        val file = File(vaultDir, "${id}.enc")
        return file.readBytes()
    }
    
    // Delete encrypted file
    fun delete(id: UUID) {
        val file = File(vaultDir, "${id}.enc")
        file.delete()
    }
    
    // Check existence
    fun exists(id: UUID): Boolean {
        val file = File(vaultDir, "${id}.enc")
        return file.exists()
    }
}
