import Foundation

// Manages encrypted files on disk
// For Phase 3, this is a simple file-based store. SQLCipher integration can follow.

class VaultFileStore {
    
    private let vaultDirectory: URL
    
    init() {
        let docs = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
        self.vaultDirectory = docs.appendingPathComponent("SpeakeasyVault", isDirectory: true)
        
        try? FileManager.default.createDirectory(at: vaultDirectory, withIntermediateDirectories: true)
    }
    
    // Save encrypted data to file
    func save(ciphertext: Data, id: UUID) throws -> URL {
        let fileURL = vaultDirectory.appendingPathComponent("\(id.uuidString).enc")
        try ciphertext.write(to: fileURL)
        return fileURL
    }
    
    // Load encrypted data from file
    func load(id: UUID) throws -> Data {
        let fileURL = vaultDirectory.appendingPathComponent("\(id.uuidString).enc")
        return try Data(contentsOf: fileURL)
    }
    
    // Delete encrypted file
    func delete(id: UUID) throws {
        let fileURL = vaultDirectory.appendingPathComponent("\(id.uuidString).enc")
        try FileManager.default.removeItem(at: fileURL)
    }
    
    // Check if file exists
    func exists(id: UUID) -> Bool {
        let fileURL = vaultDirectory.appendingPathComponent("\(id.uuidString).enc")
        return FileManager.default.fileExists(atPath: fileURL.path)
    }
}
