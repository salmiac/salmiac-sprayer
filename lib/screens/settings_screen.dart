// lib/screens/settings_screen.dart
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import '../models/sprayer_settings.dart';
import '../utils/constants.dart';
import 'package:flutter/services.dart'; // For TextInputFormatters
import 'dart:math' as math; // For pow function if needed for more complex calculations
import '../services/controller_service.dart'; // For sending settings to controller

final nozzleConstant = 2.3095; // Reverse engineered constant.

class SettingsScreen extends StatefulWidget {
  final SprayerSettings initialSettings;
  final Function(SprayerSettings) onSettingsChanged;

  const SettingsScreen({super.key, required this.initialSettings, required this.onSettingsChanged});

  @override
  _SettingsScreenState createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  late SprayerSettings _currentSettings;
  final _formKey = GlobalKey<FormState>();
  double _nominalPressureSpeed = 0.0;

  @override
  void initState() {
    super.initState();
    _currentSettings = SprayerSettings.from(widget.initialSettings);
    _calculateSpeeds(); // Calculate initial speeds based on initial settings
  }

  @override
  void didUpdateWidget(covariant SettingsScreen oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (widget.initialSettings != oldWidget.initialSettings) {
      _currentSettings = SprayerSettings.from(widget.initialSettings);
      _calculateSpeeds(); // Recalculate speeds if initial settings change
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Form(
        key: _formKey,
        autovalidateMode: AutovalidateMode.onUserInteraction, // Enable real-time validation
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: ListView(
            children: [
              SizedBox(height: 16),
              _buildSettingInputField(
                labelText: 'Nozzle Spacing (meters)',
                initialValue: _currentSettings.nozzleSpacing.toString(),
                onChanged: (value) {
                  _currentSettings.nozzleSpacing = double.tryParse(value!) ?? AppConstants.defaultNozzleSpacing;
                  _update(); // Recalculate speeds when nozzle spacing changes
                },
                onSaved: (value) {},
                validator: _validateNozzleSpacing,
                keyboardType: TextInputType.numberWithOptions(decimal: true),
                inputFormatters: [FilteringTextInputFormatter.allow(RegExp(r'^\d*\.?\d{0,2}')),], // Allow digits and max 2 decimals
              ),
              _buildNozzleDropdownField( // Replaced input field with dropdown
                labelText: 'Nozzle Size',
                value: _currentSettings.nozzleSize.number, // Use nozzle number as value
                onChanged: (value) {
                  // Find the Nozzle object from nozzleTypes based on the selected number
                  _currentSettings.nozzleSize = nozzleTypes.firstWhere((nozzle) => nozzle.number == value);
                  _update(); // Recalculate speeds when nozzle size changes
                },
                onSaved: (value) {},
              ),
              _buildSettingInputField(
                labelText: 'Litres/ha (10-999)',
                initialValue: _currentSettings.litresPerHa.toString(),
                onChanged: (value) {
                  _currentSettings.litresPerHa = double.tryParse(value!) ?? AppConstants.defaultLitresPerHa;
                  _update(); // Recalculate speeds when nozzle spacing changes
                },
                onSaved: (value) {},
                validator: _validateLitresHa,
                keyboardType: TextInputType.number,
                inputFormatters: [FilteringTextInputFormatter.digitsOnly], // Only digits
              ),
              _buildPressureSettingRow(
                labelText: 'Min Pressure (1-10)',
                initialValue: _currentSettings.minPressure.toString(),
                onChanged: (value) {
                  _currentSettings.minPressure = double.tryParse(value!) ?? AppConstants.defaultMinPressure;
                  _update(); // Recalculate speeds when min pressure changes
                },
                onSaved: (value) {},
                validator: _validateMinPressure,
                calculatedSpeed: '${_currentSettings.minSpeed.toStringAsFixed(AppConstants.speedDecimalPlaces)} km/h',
              ),
              _buildPressureSettingRow(
                labelText: 'Max Pressure (1-10)',
                initialValue: _currentSettings.maxPressure.toString(),
                onChanged: (value) {
                  _currentSettings.maxPressure = double.tryParse(value!) ?? AppConstants.defaultMaxPressure;
                  _update(); // Recalculate speeds when max pressure changes
                },
                onSaved: (value) {},
                validator: _validateMaxPressure,
                calculatedSpeed: '${_currentSettings.maxSpeed.toStringAsFixed(AppConstants.speedDecimalPlaces)} km/h',
              ),
              _buildPressureSettingRow(
                labelText: 'Nominal Pressure 1-10)',
                initialValue: _currentSettings.nominalPressure.toString(),
                onChanged: (value) {
                  _currentSettings.nominalPressure = double.tryParse(value!) ?? AppConstants.defaultNominalPressure;
                  _update(); // Recalculate speeds when nominal pressure changes
                },
                onSaved: (value) {},
                validator: _validateNominalPressure,
                calculatedSpeed: '${_nominalPressureSpeed.toStringAsFixed(AppConstants.speedDecimalPlaces)} km/h',
              ),
              SizedBox(height: 24),
              ElevatedButton(
                onPressed: _saveSettings,
                child: Text('Save Settings'),
              ),
              SizedBox(height: 24),
              ElevatedButton(
                onPressed: _reset,
                child: Text('Reset changes'),
              ),
            ],
          ),
        ),
      ),
    );
  }


  Widget _buildNozzleDropdownField({
    required String labelText,
    required String value,
    required ValueChanged<String?>? onChanged,
    FormFieldSetter<String>? onSaved,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: DropdownButtonFormField<String>(
        decoration: InputDecoration(
          labelText: labelText,
          border: OutlineInputBorder(),
        ),
        value: value, // Current nozzle number
        items: nozzleTypes.map((Nozzle nozzle) {
          return DropdownMenuItem<String>(
            value: nozzle.number, // Store nozzle number as value
            child: Row(
              children: [
                Container(
                  width: 20,
                  height: 20,
                  color: nozzle.colorCode,
                  margin: EdgeInsets.only(right: 10),
                ),
                Text('${nozzle.colorName} - ${nozzle.number}'),
              ],
            ),
          );
        }).toList(),
        onChanged: onChanged,
        onSaved: onSaved,
        validator: (value) => value == null ? 'Please select a nozzle size' : null, // Basic validator if needed
      ),
    );
  }

  Widget _buildSettingInputField({
    required String labelText,
    required String initialValue,
    required ValueChanged<String?>? onChanged,
    FormFieldSetter<String>? onSaved,
    FormFieldValidator<String>? validator,
    TextInputType? keyboardType,
    List<TextInputFormatter>? inputFormatters,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: TextFormField(
        decoration: InputDecoration(
          labelText: labelText,
          border: OutlineInputBorder(),
        ),
        initialValue: initialValue,
        onSaved: onSaved,
        validator: validator,
        keyboardType: keyboardType,
        inputFormatters: inputFormatters,
        onChanged: onChanged,
      ),
    );
  }

    Widget _buildPressureSettingRow({
    required String labelText,
    required String initialValue,
    required ValueChanged<String?>? onChanged,
    FormFieldSetter<String>? onSaved,
    FormFieldValidator<String>? validator,
    required String calculatedSpeed,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 8.0),
      child: Row(
        children: [
          Expanded(
            child: TextFormField(
              decoration: InputDecoration(
                labelText: labelText,
                border: OutlineInputBorder(),
              ),
              initialValue: initialValue,
              onSaved: onSaved,
              validator: validator,
              keyboardType: TextInputType.numberWithOptions(decimal: true),
              inputFormatters: [FilteringTextInputFormatter.allow(RegExp(r'^\d*\.?\d{0,2}')),], // Allow digits and max 2 decimals
              onChanged: onChanged,
            ),
          ),
          SizedBox(width: 12),
          Text(calculatedSpeed, style: TextStyle(fontSize: 16, color: Colors.grey)),
        ],
      ),
    );
  }


  // String? _validateNozzleSize(String? value) {
    // if (value == null || value.isEmpty) {
    //   return 'Please enter nozzle size';
    // }
    // final size = double.tryParse(value);
    // if (size == null || size < AppConstants.nozzleSizeRangeMin || size > AppConstants.nozzleSizeRangeMax) {
    //   return 'Nozzle size must be between ${AppConstants.nozzleSizeRangeMin} and ${AppConstants.nozzleSizeRangeMax}';
    // }
  //   return null;
  // }

  String? _validateLitresHa(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please enter litres/ha';
    }
    final litres = double.tryParse(value);
    if (litres == null || litres < AppConstants.litresHaRangeMin || litres > AppConstants.litresHaRangeMax) {
      return 'Litres/ha must be between ${AppConstants.litresHaRangeMin} and ${AppConstants.litresHaRangeMax}';
    }
    return null;
  }

  String? _validateMinPressure(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please enter minimum pressure';
    }
    final pressure = double.tryParse(value);
    if (pressure == null || pressure < AppConstants.pressureRangeMin || pressure > _currentSettings.maxPressure) {
      return 'Min pressure must be between ${AppConstants.pressureRangeMin} and ${_currentSettings.maxPressure}';
    }
    return null;
  }
    String? _validateMaxPressure(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please enter maximum pressure';
    }
    final pressure = double.tryParse(value);
    if (pressure == null || pressure < _currentSettings.minPressure || pressure > AppConstants.pressureRangeMax) {
      return 'Max pressure must be between ${_currentSettings.minPressure} and ${AppConstants.pressureRangeMax}';
    }
    return null;
  }

  String? _validateNominalPressure(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please enter nominal pressure';
    }
    final pressure = double.tryParse(value);
    if (pressure == null || pressure < _currentSettings.minPressure || pressure > _currentSettings.maxPressure) {
      return 'Nominal pressure must be between ${_currentSettings.minPressure} and ${_currentSettings.maxPressure}';
    }
    return null;
  }

  String? _validateNozzleSpacing(String? value) {
    if (value == null || value.isEmpty) {
      return 'Please enter nozzle spacing';
    }
    final spacing = double.tryParse(value);
    if (spacing == null || spacing < AppConstants.nozzleSpacingRangeMin || spacing > AppConstants.nozzleSpacingRangeMax) {
      return 'Nozzle spacing must be between ${AppConstants.nozzleSpacingRangeMin} and ${AppConstants.nozzleSpacingRangeMax}';
    }
    return null;
  }

  void _saveSettings() {
    if (_formKey.currentState!.validate()) {
      _formKey.currentState!.save();
      widget.onSettingsChanged(_currentSettings); // Pass the updated settings back to the main app
      UDPSettingsSender sender = UDPSettingsSender(targetIp: '255.255.255.255', targetPort: 8888);
      sender.sendSettings(_currentSettings);
      if (kDebugMode) {
        print('Settings sent via UDP.');
      }
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Settings saved')),
      );
    }
  }

  void _reset() {
      _currentSettings = SprayerSettings.from(widget.initialSettings);
      _update(); // Recalculate speeds if initial settings change
  }

  void _update() {

    setState(() {
      
    _calculateSpeeds();
    });
  }

  // **Simplified Speed Calculation - Placeholder. Replace with actual formula.**
  void _calculateSpeeds() {
    _currentSettings.minSpeed = _calculateSpeedForPressure(_currentSettings.minPressure);
    _currentSettings.maxSpeed = _calculateSpeedForPressure(_currentSettings.maxPressure);
    _nominalPressureSpeed = _calculateSpeedForPressure(_currentSettings.nominalPressure);
    widget.onSettingsChanged(_currentSettings); // Pass the updated settings back to the main app
  }

  double _calculateSpeedForPressure(double pressure) {
    /*
    litres_per_min = nozzleConstant * nozzleSize * sqrt(pressure)
    speed = (litres_per_min * 600) / (litres_per_ha * nozzleSpacing)
    speed = (nozzleConstant * nozzleSize * sqrt(pressure) * 600) / (litres_per_ha * nozzleSpacing)
    */
    double speedPerSqrtPressure = nozzleConstant * _currentSettings.nozzleSize.sizeValue * 600 / (_currentSettings.litresPerHa * _currentSettings.nozzleSpacing);
    return math.sqrt(pressure) * speedPerSqrtPressure;
  }
}

// Extension to create a copy of SprayerSettings (already defined in sprayer_settings.dart previously)