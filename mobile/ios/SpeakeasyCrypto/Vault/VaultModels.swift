import Foundation

// Models for Vault Items

struct VaultItem: Codable {
    var id: UUID
    var type: VaultItemType
    var createdAt: Date
    var updatedAt: Date
    var wrappedFileKeyB64: String?
    var nonceB64: String
    var ciphertextPath: String?     // Local file path
    var attachmentId: UUID?         // If uploaded to server
    var mimeType: String?
    var ciphertextSizeBytes: Int64?
    var displayNameEnc: String?     // Encrypted display name
}

enum VaultItemType: String, Codable {
    case note
    case file
    case photo
    case video
    case attachmentRef
}

struct VaultAttachmentRef: Codable {
    var attachmentId: UUID
    var storageKey: String
    var sha256CiphertextB64: String
    var encAlg: String
    var nonceB64: String
    var sizeBytes: Int64
    var mimeType: String?
}
