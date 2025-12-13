package com.speakeasy.sms

import android.content.Context

object PrivatePolicyStore {
    // TODO: store in encrypted prefs/db
    fun isPrivateNumber(context: Context, e164: String): Boolean {
        // Implement exact match normalization (E.164) here.
        return false
    }
}
