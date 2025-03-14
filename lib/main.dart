// lib/main.dart
import 'package:flutter/material.dart';
import 'screens/monitor_screen.dart';
import 'package:flutter/foundation.dart';
import 'screens/settings_screen.dart';
import 'services/controller_service.dart';
import 'models/sprayer_settings.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Create ControllerService instance with your device IP and port.
  final controllerService =
      ControllerService(controllerIp: '255.255.255.255', controllerPort: 1111);

  // Start UDP receiver for status updates.
  await controllerService.startUDPListener();

  runApp(const SalmiacSprayer());
}

class SalmiacSprayer extends StatelessWidget {
  const SalmiacSprayer({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Smart Sprayer',
      theme: ThemeData.light(),
      darkTheme: ThemeData.dark(),
      themeMode: ThemeMode.system, // Uses system light/dark mode setting.
      home: const HomePage(),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  int _currentIndex = 1;
  late ControllerService _controllerService;
  SprayerSettings _currentSettings = SprayerSettings(); // Initialize with default settings

  @override
  void initState() {
    super.initState();
    // Replace with your controller IP and port
    _controllerService = ControllerService(controllerIp: '255.255.255.255', controllerPort: 1111);
  }

  @override
  void dispose() {
    _controllerService.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: _buildBody(),
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: _currentIndex,
        onTap: (index) {
          setState(() {
            _currentIndex = index;
          });
        },
        items: const [
          BottomNavigationBarItem(icon: Icon(Icons.monitor), label: 'Monitor'),
          BottomNavigationBarItem(icon: Icon(Icons.settings), label: 'Settings'),
        ],
      ),
    );
  }

  Widget _buildBody() {
    switch (_currentIndex) {
      case 0:
        return MonitorScreen(
          controllerService: _controllerService,
          currentSettings: _currentSettings,
        );
      case 1:
        return SettingsScreen(
          initialSettings: _currentSettings,
          onSettingsChanged: _updateSettings,
        );
      default:
        return Container(); // Should not happen
    }
  }

  void _updateSettings(SprayerSettings newSettings) {
    // setState(() {
      _currentSettings = newSettings;
      // Optionally, you can send these settings to the controller immediately upon saving from settings screen,
      // if that's the desired behavior. For now, settings are sent from MonitorScreen when control buttons are pressed.
    // });
    if (kDebugMode) {
      print('Settings updated in main app: Nozzle Size: ${_currentSettings.nozzleSize}, Litres/ha: ${_currentSettings.litresPerHa}');
    }
  }
}