// lib/screens/monitor_screen.dart
import 'package:flutter/material.dart';
import 'dart:async';
import 'package:flutter/foundation.dart';
import '../widgets/pressure_display.dart';
import '../widgets/speed_display.dart';
import '../services/controller_service.dart';
import '../models/sprayer_data.dart' as data;
import '../models/sprayer_settings.dart' as settings;
import '../utils/constants.dart';

class MonitorScreen extends StatefulWidget {
  final ControllerService controllerService;
  final settings.SprayerSettings currentSettings; // Receive current settings

  const MonitorScreen({super.key, required this.controllerService, required this.currentSettings});

  @override
  _MonitorScreenState createState() => _MonitorScreenState();
}

class _MonitorScreenState extends State<MonitorScreen> {
  bool _controllerActivated = false;
  bool _constantPressureMode = false;
  data.SprayerData _sprayerData = data.SprayerData(currentPressure: 0.0, targetPressure: 0.0, speed: 0.0, boomLocked: false);
  StreamSubscription<data.SprayerData>? _dataSubscription;

  @override
  void initState() {
    super.initState();
    _dataSubscription = widget.controllerService.dataStream.listen((data) {
      setState(() {
        _sprayerData = data;
      });
    });
    widget.controllerService.startUDPListener(); // Start listening on screen init
  }

// ... existing code ...

  @override
  void didUpdateWidget(covariant MonitorScreen oldWidget) {
    super.didUpdateWidget(oldWidget);
    // Check if minSpeed or maxSpeed has changed
    final oldMin = oldWidget.currentSettings.minSpeed;
    final oldMax = oldWidget.currentSettings.maxSpeed;
    final newMin = widget.currentSettings.minSpeed;
    final newMax = widget.currentSettings.maxSpeed;

    if (newMin != oldMin || newMax != oldMax) {
      setState(() {}); // Trigger rebuild with updated values
    }
  }

// ... existing code ...

//   @override
//   void didUpdateWidget(covariant MonitorScreen oldWidget) {
//   super.didUpdateWidget(oldWidget);
//   print('Curent Settings updated: minSpeed=${widget.currentSettings.minSpeed}, maxSpeed=${widget.currentSettings.maxSpeed}');
//   if (widget.currentSettings != oldWidget.currentSettings) {
//     if (kDebugMode) {
//       print('Settings updated: minSpeed=${widget.currentSettings.minSpeed}, maxSpeed=${widget.currentSettings.maxSpeed}');
//     }
//     _sendButtonStates(); // Resend settings to controller if settings are updated
//   }
// }

  @override
  void dispose() {
    _dataSubscription?.cancel();
    widget.controllerService.stopUDPListener(); // Stop listening when screen is disposed
    super.dispose();
  }

// ...existing code...
  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    return Scaffold(
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceAround,
                children: [
                  ElevatedButton( // Controller ON/OFF Button
                    onPressed: () {
                      setState(() {
                        _controllerActivated = !_controllerActivated;
                        _sendButtonStates(); // Send updated settings to controller
                      });
                    },
                    style: ElevatedButton.styleFrom(
                      backgroundColor: _controllerActivated
                          ? AppConstants.successColor
                          : AppConstants.buttonColor,
                      padding: EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                      textStyle: TextStyle(fontSize: 20),
                    ),
                    child: Text(
                      _controllerActivated ? 'Controller ON' : 'Controller OFF', 
                      style: TextStyle(color: AppConstants.buttonTextColor)
                    ),
                  ),
                  ElevatedButton( // Constant/Variable Pressure Button
                    style: ElevatedButton.styleFrom(
                      backgroundColor: !_constantPressureMode
                          ? AppConstants.successColor
                          : AppConstants.buttonColor,
                      padding: EdgeInsets.symmetric(horizontal: 24, vertical: 16),
                      textStyle: TextStyle(fontSize: 20),
                    ),
                    onPressed: () {
                      setState(() {
                        _constantPressureMode = !_constantPressureMode;
                        _sendButtonStates(); // Send updated settings to controller
                      });
                    },
                    child: Text(
                      _constantPressureMode ? 'Constant' : 'Variable', 
                      style: TextStyle(color: AppConstants.buttonTextColor)
                    ),
                  ),
                ],
              ),
              SizedBox(height: 20),
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                children: [
                  // Target Pressure on the left
                  Expanded(
                    child: Column(
                      children: [
                        Text('Target Pressure', style: theme.textTheme.headlineSmall),
                        Card(
                          color: theme.cardColor,
                          child: Padding(
                            padding: const EdgeInsets.all(8.0),
                            child: PressureDisplay(pressureValue: _sprayerData.targetPressure),
                          ),
                        ),
                      ],
                    ),
                  ),
                  // Current Pressure on the right
                  Expanded(
                    child: Column(
                      children: [
                        Text('Current Pressure', style: theme.textTheme.headlineSmall),
                        Card(
                          color: theme.cardColor,
                          child: Padding(
                            padding: const EdgeInsets.all(8.0),
                            child: PressureDisplay(pressureValue: _sprayerData.currentPressure),
                          ),
                        ),
                      ],
                    ),
                  ),
                ],
              ),              
              SizedBox(height: 20),
              // Speed display in a Card
              Column(
                children: [
                  // Text('Speed',style: theme.textTheme.headlineSmall),
                  Card(
                    color: theme.cardColor,
                    child: Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: SpeedDisplay(
                        key: ValueKey('${widget.controllerService.currentSpeed}_${widget.currentSettings.minSpeed}_${widget.currentSettings.maxSpeed}'),
                        speedValue: widget.controllerService.currentSpeed,
                        minSpeed: widget.currentSettings.minSpeed, 
                        maxSpeed: widget.currentSettings.maxSpeed
                      ),
                    ),
                  ),
                ],
              ),
              SizedBox(height: 20),
              Text('Boom Locked: ${_sprayerData.boomLocked ? 'YES' : 'NO'}'),
              SizedBox(height: 20),
              _buildNozzleDisplay(), // Display Nozzle Information
            ],
          ),
        ),
      ),
    );
  }
// ...existing code...

  Widget _buildNozzleDisplay() {
    final currentNozzle = widget.currentSettings.nozzleSize;
    return Column(
      children: [
        SizedBox(height: 8),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
              width: 30,
              height: 30,
              decoration: BoxDecoration(
                color: currentNozzle.colorCode,
                shape: BoxShape.circle,
                border: Border.all(color: Colors.black38), // Optional border for better visibility of light colors
              ),
            ),
            SizedBox(width: 10),
            Text(
              '${currentNozzle.number} - ${currentNozzle.colorName}',
              style: TextStyle(fontSize: 18),
            ),
          ],
        ),
      ],
    );
  }

  void _sendButtonStates() {
    // Create a sender instance (adjust targetIp and port as needed)
    UDPButtonStateSender sender = UDPButtonStateSender(targetIp: '255.255.255.255', targetPort: 8888);
    sender.sendButtonState(_controllerActivated, _constantPressureMode);
    if (kDebugMode) {
      print('Button states sent: activated=$_controllerActivated, constantPressure=$_constantPressureMode');
    }
  }

}