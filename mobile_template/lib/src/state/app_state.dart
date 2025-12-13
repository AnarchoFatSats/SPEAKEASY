import 'privacy_mode.dart';

class AppState {
  PrivacyMode mode = PrivacyMode.public;

  // In v1 keep these as in-memory; later persist securely.
  bool isUnlocked = false;

  void lock() => isUnlocked = false;
  void unlock() => isUnlocked = true;
}
