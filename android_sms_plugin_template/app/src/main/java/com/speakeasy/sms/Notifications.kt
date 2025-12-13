package com.speakeasy.sms

import android.content.Context

object Notifications {
    fun maybeNotifyPrivate(context: Context) {
        // TODO: show generic notification (no sender/body) or none
    }
    fun maybeNotifyPublic(context: Context, from: String, body: String) {
        // TODO: normal notification (respect user settings)
    }
}
