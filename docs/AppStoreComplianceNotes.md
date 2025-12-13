# App Store / Play Store Compliance Notes

## iOS
- iOS does not permit:
  - replacing Messages as default SMS app
  - intercepting iMessage or SMS content
  - altering CarPlay Messages output
- Our iOS app:
  - provides in-app encrypted messaging + vault
  - uses generic/silent notifications
  - provides Shortcuts/App Intents for user-configured automation (Car Mode)

## Android
- Default SMS features require:
  - becoming the default SMS app (RoleManager)
  - proper Play Console permission declarations
  - user-visible justification
- We do NOT read messages from other apps (WhatsApp/Instagram/etc.).

## Marketing language
Do:
- “privacy-first”, “secure vault”, “protect sensitive notifications”, “separate work/personal”
Avoid:
- “cheating”, “hide from spouse”, “undetectable”, “secret affair”
