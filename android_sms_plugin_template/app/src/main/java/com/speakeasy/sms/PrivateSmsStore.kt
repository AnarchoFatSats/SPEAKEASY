package com.speakeasy.sms

import android.content.Context

object PrivateSmsStore {
    fun insertIncoming(context: Context, from: String, body: String, ts: Long) {
        // TODO: Room + SQLCipher insert
    }
}
