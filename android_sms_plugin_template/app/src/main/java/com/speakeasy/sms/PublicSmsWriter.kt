package com.speakeasy.sms

import android.content.ContentValues
import android.content.Context
import android.net.Uri
import android.provider.Telephony

object PublicSmsWriter {
    fun writeToInbox(context: Context, from: String, body: String, ts: Long) {
        val values = ContentValues().apply {
            put(Telephony.Sms.ADDRESS, from)
            put(Telephony.Sms.BODY, body)
            put(Telephony.Sms.DATE, ts)
            put(Telephony.Sms.READ, 0)
        }
        context.contentResolver.insert(Uri.parse("content://sms/inbox"), values)
    }
}
