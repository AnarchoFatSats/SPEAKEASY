package com.speakeasy.crypto.vault

import java.util.UUID
import java.util.Date

// Models for Vault Items

data class VaultItem(
    val id: UUID,
    val type: VaultItemType,
    val createdAt: Date,
    val updatedAt: Date,
    val wrappedFileKeyB64: String?,
    val nonceB64: String,
    val ciphertextPath: String?,    // Local file path
    val attachmentId: UUID?,        // If uploaded to server
    val mimeType: String?,
    val ciphertextSizeBytes: Long?,
    val displayNameEnc: String?     // Encrypted display name
)

enum class VaultItemType {
    NOTE,
    FILE,
    PHOTO,
    VIDEO,
    ATTACHMENT_REF
}

data class VaultAttachmentRef(
    val attachmentId: UUID,
    val storageKey: String,
    val sha256CiphertextB64: String,
    val encAlg: String,
    val nonceB64: String,
    val sizeBytes: Long,
    val mimeType: String?
)
