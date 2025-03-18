// lib/models/sprayer_settings.dart
import 'dart:typed_data';
import 'dart:convert';
import 'package:flutter/material.dart';

class Nozzle {
  final String colorName;
  final Color colorCode;
  final String number;
  final double sizeValue;

  Nozzle({
    required this.colorName,
    required this.colorCode,
    required this.number,
    required this.sizeValue,
  });

  static Nozzle? fromNumber(String number) {
    try {
      return nozzleTypes.firstWhere((nozzle) => nozzle.number == number);
    } catch (e) {
      return null; // Nozzle number not found
    }
  }
}

// Predefined Nozzle Types List
final List<Nozzle> nozzleTypes = [
  Nozzle(colorName: 'Orange', colorCode: const Color(0xFFFFA500), number: '01', sizeValue: 0.10),
  Nozzle(colorName: 'Green', colorCode: const Color(0xFF008000), number: '015', sizeValue: 0.15),
  Nozzle(colorName: 'Yellow', colorCode: const Color(0xFFFFFF00), number: '02', sizeValue: 0.20),
  Nozzle(colorName: 'Lilac', colorCode: const Color(0xFFFFC0CB), number: '025', sizeValue: 0.25), // Using Lilac as Pink is very similar and might be confused in UI
  Nozzle(colorName: 'Blue', colorCode: const Color(0xFF0000FF), number: '03', sizeValue: 0.30),
  Nozzle(colorName: 'Dark Red', colorCode: const Color(0xFF8B0000), number: '035', sizeValue: 0.35), // Using Dark Red instead of Red Brown as it's closer to the color code
  Nozzle(colorName: 'Red', colorCode: const Color(0xFFFF0000), number: '04', sizeValue: 0.40),
  Nozzle(colorName: 'Brown', colorCode: const Color(0xFFA52A2A), number: '05', sizeValue: 0.50),
  Nozzle(colorName: 'Gray', colorCode: const Color(0xFF808080), number: '06', sizeValue: 0.60),
  Nozzle(colorName: 'White', colorCode: Colors.white, number: '08', sizeValue: 0.80), // Using Colors.white for convenience
  Nozzle(colorName: 'Light Blue', colorCode: const Color(0xFFADD8E6), number: '1', sizeValue: 1.00), // Number '1' as string
  Nozzle(colorName: 'Light Green', colorCode: const Color(0xFF90EE90), number: '15', sizeValue: 1.50), // Number '15' as string
  Nozzle(colorName: 'Black', colorCode: Colors.black, number: '2', sizeValue: 2.00), // Number '2' as string, using Colors.black
];

// Model for settings
class SprayerSettings extends ChangeNotifier { // changed: added 'extends ChangeNotifier'
  Nozzle nozzleSize;
  double litresPerHa;
  double minPressure;
  double maxPressure;
  double nominalPressure;
  double nozzleSpacing;
  double minSpeed;
  double maxSpeed;

  SprayerSettings({
    Nozzle? nozzleSize,
    this.litresPerHa = 200,
    this.minPressure = 1.0,
    this.maxPressure = 6.0,
    this.nominalPressure = 3.0,
    this.nozzleSpacing = 0.5,
    this.minSpeed = 0.0,
    this.maxSpeed = 0.0,
  }) : nozzleSize = nozzleSize ?? nozzleTypes.firstWhere((nozzle) => nozzle.number == '025');

  SprayerSettings.from(SprayerSettings other)
      : nozzleSize = other.nozzleSize,
        litresPerHa = other.litresPerHa,
        minPressure = other.minPressure,
        maxPressure = other.maxPressure,
        nominalPressure = other.nominalPressure,
        nozzleSpacing = other.nozzleSpacing,
        minSpeed = other.minSpeed,
        maxSpeed = other.maxSpeed;

  // Call this method on Settings screen save to update values (e.g. for SpeedDisplay)
  void updateSettings(SprayerSettings newSettings) {
    nozzleSize = newSettings.nozzleSize;
    litresPerHa = newSettings.litresPerHa;
    minPressure = newSettings.minPressure;
    maxPressure = newSettings.maxPressure;
    nominalPressure = newSettings.nominalPressure;
    nozzleSpacing = newSettings.nozzleSpacing;
    minSpeed = newSettings.minSpeed;
    maxSpeed = newSettings.maxSpeed;
    notifyListeners(); // SpeedDisplay and other listeners update here
  }

  // You can add methods to convert settings to bytes for sending to controller
  List<int> toBytes({
    bool activated = false,
    bool constantPressure = false,
    double nozzleNumber = 0, // Use nozzle number to send to controller
  }) {
    // Define the binary message format for settings sent to the controller
    // Example: activated (bool), nominalPressure (float), constantPressure (bool), nozzleNumber (3 digit string)
    var buffer = BytesBuilder();
    buffer.addByte(activated ? 1 : 0); // Activated (1 byte)
    var byteData = ByteData(4)..setFloat32(0, nominalPressure, Endian.little);
    buffer.add(byteData.buffer.asUint8List()); // Nominal pressure (4 bytes)
    buffer.addByte(constantPressure ? 1 : 0); // Constant pressure (1 byte)

    // Ensure nozzle number is 3 digits - might need adjustment based on controller's expected input
    String nozzleNumberStr = nozzleNumber.toString().padLeft(3, '025'); // Pad with leading zeros if necessary
    buffer.add(byteData.buffer.asUint8List()); // Nominal pressure (4 bytes)
    buffer.add(utf8.encode(nozzleNumberStr)); // Nozzle number as 3 ASCII digits (3 bytes)

    return buffer.toBytes();
  }

  // Convert settings to a Map for serialization.
  Map<String, dynamic> toMap() {
    return {
      'nozzleNumber': nozzleSize.number, // store nozzle by number
      'litresPerHa': litresPerHa,
      'minPressure': minPressure,
      'maxPressure': maxPressure,
      'nominalPressure': nominalPressure,
      'nozzleSpacing': nozzleSpacing,
      'minSpeed': minSpeed,
      'maxSpeed': maxSpeed,
    };
  }

  // Create instance from a Map.
  factory SprayerSettings.fromMap(Map<String, dynamic> map) {
    return SprayerSettings(
      nozzleSize: Nozzle.fromNumber(map['nozzleNumber']),
      litresPerHa: (map['litresPerHa'] as num).toDouble(),
      minPressure: (map['minPressure'] as num).toDouble(),
      maxPressure: (map['maxPressure'] as num).toDouble(),
      nominalPressure: (map['nominalPressure'] as num).toDouble(),
      nozzleSpacing: (map['nozzleSpacing'] as num).toDouble(),
      minSpeed: (map['minSpeed'] as num).toDouble(),
      maxSpeed: (map['maxSpeed'] as num).toDouble(),
    );
  }
}

