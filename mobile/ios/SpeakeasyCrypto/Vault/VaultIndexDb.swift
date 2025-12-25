import Foundation

// Simple in-memory + JSON file vault index
// For production, replace with SQLCipher

class VaultIndexDb {
    
    private var items: [UUID: VaultItem] = [:]
    private let indexURL: URL
    
    init() {
        let docs = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
        self.indexURL = docs.appendingPathComponent("SpeakeasyVault/index.json")
        load()
    }
    
    private func load() {
        guard FileManager.default.fileExists(atPath: indexURL.path),
              let data = try? Data(contentsOf: indexURL),
              let decoded = try? JSONDecoder().decode([UUID: VaultItem].self, from: data) else {
            return
        }
        items = decoded
    }
    
    private func persist() {
        if let data = try? JSONEncoder().encode(items) {
            try? data.write(to: indexURL)
        }
    }
    
    func insert(_ item: VaultItem) {
        items[item.id] = item
        persist()
    }
    
    func get(_ id: UUID) -> VaultItem? {
        return items[id]
    }
    
    func getAll() -> [VaultItem] {
        return Array(items.values)
    }
    
    func update(_ item: VaultItem) {
        items[item.id] = item
        persist()
    }
    
    func delete(_ id: UUID) {
        items.removeValue(forKey: id)
        persist()
    }
}
