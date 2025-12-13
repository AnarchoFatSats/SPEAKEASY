import 'package:flutter/material.dart';
import '../../routes.dart';
import '../../state/app_state.dart';

/// UnlockScreen is intentionally *not* a password screen by default.
/// It can be:
/// - a gesture unlock
/// - a character/sequence unlock
/// - and optionally a biometric check after success
class UnlockScreen extends StatefulWidget {
  final AppState state;
  const UnlockScreen({super.key, required this.state});

  @override
  State<UnlockScreen> createState() => _UnlockScreenState();
}

class _UnlockScreenState extends State<UnlockScreen> {
  // Example: secret sequence "BOUNCER" typed into a field for demo.
  final controller = TextEditingController();

  @override
  void dispose() {
    controller.dispose();
    super.dispose();
  }

  void tryUnlock() {
    if (controller.text.trim().toUpperCase() == "BOUNCER") {
      widget.state.unlock();
      Navigator.pushReplacementNamed(context, Routes.privateHome);
    } else {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Nope.')),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Difficulty Select')),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          children: [
            const Text('Demo unlock: type BOUNCER'),
            TextField(controller: controller),
            const SizedBox(height: 12),
            ElevatedButton(onPressed: tryUnlock, child: const Text('Start')),
          ],
        ),
      ),
    );
  }
}
