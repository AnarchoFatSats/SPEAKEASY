package com.speakeasy.sms

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.provider.Telephony
import android.telephony.SmsMessage

/**
 * Fires ONLY when app is default SMS handler.
 * Route messages to:
 *  - public inbox (Telephony provider) or
 *  - private encrypted store (Room/SQLCipher)
 */
class SmsDeliverReceiver : BroadcastReceiver() {

    override fun onReceive(context: Context, intent: Intent) {
        if (Telephony.Sms.Intents.SMS_DELIVER_ACTION != intent.action) return

        val messages = Telephony.Sms.Intents.getMessagesFromIntent(intent)
        for (msg in messages) {
            val from = msg.displayOriginatingAddress ?: ""
            val body = msg.displayMessageBody ?: ""

            if (PrivatePolicyStore.isPrivateNumber(context, from)) {
                // Store in encrypted private DB only
                PrivateSmsStore.insertIncoming(context, from, body, msg.timestampMillis)
                // Optionally show generic notification or none
                Notifications.maybeNotifyPrivate(context)
            } else {
                // Write to system SMS provider so normal SMS apps can show it
                PublicSmsWriter.writeToInbox(context, from, body, msg.timestampMillis)
                Notifications.maybeNotifyPublic(context, from, body)
            }
        }

        // Abort broadcast so other receivers don't double-handle? (careful; default SMS typically handles)
        // abortBroadcast() // Only if ordered broadcast and allowed; verify behavior per Android version
    }
}
