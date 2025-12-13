# Android Default SMS Plugin Template

This folder contains Kotlin templates to implement Android-only default SMS behavior.

You will integrate this as:
- a Flutter plugin (MethodChannel or FFI)
- or as native module if you go fully Kotlin UI for SMS.

Key requirements:
- Request RoleManager ROLE_SMS
- Implement BroadcastReceiver for SMS_DELIVER
- Write public SMS to Telephony provider
- Store private SMS in encrypted Room/SQLCipher DB
