import 'package:flutter/material.dart';
import 'state/app_state.dart';
import 'ui/cover/cover_home.dart';
import 'ui/unlock/unlock_screen.dart';
import 'ui/private/private_home.dart';

class Routes {
  static const cover = '/';
  static const unlock = '/unlock';
  static const privateHome = '/private';
}

class AppRouter {
  static Route<dynamic> onGenerateRoute(RouteSettings settings, AppState state) {
    switch (settings.name) {
      case Routes.cover:
        return MaterialPageRoute(builder: (_) => CoverHome(state: state));
      case Routes.unlock:
        return MaterialPageRoute(builder: (_) => UnlockScreen(state: state));
      case Routes.privateHome:
        return MaterialPageRoute(builder: (_) => PrivateHome(state: state));
      default:
        return MaterialPageRoute(builder: (_) => Scaffold(
          body: Center(child: Text('Unknown route: ${settings.name}')),
        ));
    }
  }
}
