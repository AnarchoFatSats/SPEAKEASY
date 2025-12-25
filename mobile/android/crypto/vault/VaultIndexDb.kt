package com.speakeasy.crypto.vault

import android.content.Context
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import java.io.File
import java.util.UUID

// Simple JSON-file based vault index
// For production, replace with SQLCipher

class VaultIndexDb(context: Context) {
    
    private val indexFile: File = File(context.filesDir, "SpeakeasyVault/index.json")
    private var items: MutableMap<UUID, VaultItem> = mutableMapOf()
    private val gson = Gson()
    
    init {
        load()
    }
    
    private fun load() {
        if (!indexFile.exists()) return
        try {
            val json = indexFile.readText()
            val type = object : TypeToken<MutableMap<UUID, VaultItem>>() {}.type
            items = gson.fromJson(json, type) ?: mutableMapOf()
        } catch (e: Exception) {
            items = mutableMapOf()
        }
    }
    
    private fun persist() {
        indexFile.parentFile?.mkdirs()
        indexFile.writeText(gson.toJson(items))
    }
    
    fun insert(item: VaultItem) {
        items[item.id] = item
        persist()
    }
    
    fun get(id: UUID): VaultItem? {
        return items[id]
    }
    
    fun getAll(): List<VaultItem> {
        return items.values.toList()
    }
    
    fun update(item: VaultItem) {
        items[item.id] = item
        persist()
    }
    
    fun delete(id: UUID) {
        items.remove(id)
        persist()
    }
}
