package com.speakeasy.crypto

import android.app.Application
import android.util.Log
import com.speakeasy.crypto.vault.VaultTestRunner

// Simple test app to run verification
// This would normally be in mobile/android/app but we're keeping it simple

class TestApplication : Application() {
    
    override fun onCreate() {
        super.onCreate()
        
        Log.d("Speakeasy", "=== RUNNING VERIFICATION TESTS ===")
        
        // Run Vault Test
        val vaultRunner = VaultTestRunner(this)
        vaultRunner.run()
        
        Log.d("Speakeasy", "=== VERIFICATION COMPLETE ===")
    }
}
