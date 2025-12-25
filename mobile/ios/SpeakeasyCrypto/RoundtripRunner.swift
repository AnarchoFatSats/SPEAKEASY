import Foundation

// A simple test harness to verify the full flow

class RoundtripRunner {
    
    static func run() {
        print("🚀 Starting Roundtrip Test...")
        
        let storeA = KeychainStore() // In real test, use ephemeral/mock store to avoid overwriting real keys
        let clientA = SignalClient(userId: "UserA", deviceId: "DeviceA", store: storeA)
        
        let storeB = KeychainStore()
        let clientB = SignalClient(userId: "UserB", deviceId: "DeviceB", store: storeB) // Note: Keychain might conflict on same device/sim if service name same
        
        do {
            // 1. Setup A
            try clientA.generateIdentity()
            let bundleA = try clientA.generatePreKeys()
            print("✅ User A Keys Generated")
            
            // 2. B "Fetches" A's Bundle (Mock Network)
            // Need to flatten/serialize logic here if strictly passing strings
            // For harness, we assume bundleA dictionary is passed directly
            
            // 3. B Builds Session
            try clientB.processPreKeyBundle(for: "UserA", bundle: bundleA)
            print("✅ User B Processed Bundle")
            
            // 4. B Encrypts
            let plaintext = "Hello Speakeasy!".data(using: .utf8)!
            let ciphertext = try clientB.encrypt(to: "UserA", plaintext: plaintext)
            print("✅ Encrypted: \(ciphertext.prefix(20))...")
            
            // 5. A Decrypts
            // A needs to build session from PreKeyMessage automatically on decrypt
            let decrypted = try clientA.decrypt(from: "UserB", ciphertext: ciphertext)
            let message = String(data: decrypted, encoding: .utf8)!
            
            print("✅ Decrypted: \(message)")
            
            if message == "Hello Speakeasy!" {
                print("🏆 ROUNDTRIP SUCCESS!")
            } else {
                print("❌ MISMATCH")
            }
            
        } catch {
            print("❌ ERROR: \(error)")
        }
    }
}
