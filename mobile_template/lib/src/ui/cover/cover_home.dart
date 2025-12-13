import 'package:flutter/material.dart';
import '../../routes.dart';
import '../../state/app_state.dart';

/// CoverHome is the "public face" of the app.
/// Users can choose themes; this sample shows a simple Speakeasy-style landing.
class CoverHome extends StatelessWidget {
  final AppState state;
  const CoverHome({super.key, required this.state});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Speakeasy Bar')),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Text('Cover Mode (public)'),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: () => Navigator.pushNamed(context, Routes.unlock),
              child: const Text('Enter (Unlock)'),
            ),
          ],
        ),
      ),
    );
  }
}
