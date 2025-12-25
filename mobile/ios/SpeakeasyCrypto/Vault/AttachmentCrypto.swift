import Foundation
import CryptoKit

// Streaming encryption/decryption for attachments using AES-GCM

class AttachmentCrypto {
    
    // Encrypt data with a provided key
    static func encrypt(plaintext: Data, key: SymmetricKey) throws -> (ciphertext: Data, nonce: Data) {
        let nonce = AES.GCM.Nonce()
        let box = try AES.GCM.seal(plaintext, using: key, nonce: nonce)
        
        return (ciphertext: box.ciphertext + box.tag, nonce: Data(nonce))
    }
    
    // Decrypt data with a provided key and nonce
    static func decrypt(ciphertext: Data, nonce: Data, key: SymmetricKey) throws -> Data {
        let gcmNonce = try AES.GCM.Nonce(data: nonce)
        
        // Separate ciphertext and tag (last 16 bytes)
        let tagLength = 16
        let encryptedData = ciphertext.prefix(ciphertext.count - tagLength)
        let tag = ciphertext.suffix(tagLength)
        
        let box = try AES.GCM.SealedBox(nonce: gcmNonce, ciphertext: encryptedData, tag: tag)
        return try AES.GCM.open(box, using: key)
    }
    
    // Compute SHA256 hash of data (for ciphertext verification)
    static func sha256(data: Data) -> String {
        let hash = SHA256.hash(data: data)
        return Data(hash).base64EncodedString()
    }
}
