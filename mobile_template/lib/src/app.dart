import 'package:flutter/material.dart';
import 'routes.dart';
import 'state/app_state.dart';

class SpeakeasyApp extends StatefulWidget {
  const SpeakeasyApp({super.key});

  @override
  State<SpeakeasyApp> createState() => _SpeakeasyAppState();
}

class _SpeakeasyAppState extends State<SpeakeasyApp> {
  final AppState state = AppState();

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Speakeasy',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      onGenerateRoute: (settings) => AppRouter.onGenerateRoute(settings, state),
      initialRoute: Routes.cover,
    );
  }
}
