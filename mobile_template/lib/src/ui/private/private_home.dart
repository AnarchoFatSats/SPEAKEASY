import 'package:flutter/material.dart';
import '../../state/app_state.dart';

class PrivateHome extends StatelessWidget {
  final AppState state;
  const PrivateHome({super.key, required this.state});

  @override
  Widget build(BuildContext context) {
    if (!state.isUnlocked) {
      return const Scaffold(body: Center(child: Text('Locked')));
    }

    return Scaffold(
      appBar: AppBar(title: const Text('Private Room')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: const [
          ListTile(title: Text('Private Circle (contacts)'), subtitle: Text('TODO')),
          ListTile(title: Text('Secure Messenger'), subtitle: Text('TODO')),
          ListTile(title: Text('Vault'), subtitle: Text('TODO')),
          ListTile(title: Text('Privacy Modes'), subtitle: Text('TODO')),
        ],
      ),
    );
  }
}
