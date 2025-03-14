// lib/utils/constants.dart
import 'package:flutter/material.dart';

class AppConstants {
  static const defaultNominalPressure = 3.0;
  static const defaultNozzleSize = 0.25;
  static const defaultLitresPerHa = 200.0;
  static const defaultMinPressure = 1.0;
  static const defaultMaxPressure = 6.0;
  static const defaultNozzleSpacing = 0.5;

  static const pressureDecimalPlaces = 2;
  static const speedDecimalPlaces = 1;

  static const pressureRangeMin = 0.0; // Adjust as needed
  static const pressureRangeMax = 10.0; // Adjust as needed
  static const speedRangeMin = 0.0;
  static const speedRangeMax = 60.0;
  static const nozzleSizeRangeMin = 0.1;
  static const nozzleSizeRangeMax = 2.0;
  static const nozzleSpacingRangeMin = 0.1; // Adjust as needed
  static const nozzleSpacingRangeMax = 2.0; // Adjust as needed
  static const litresHaRangeMin = 10; // Adjust as needed
  static const litresHaRangeMax = 999; // Adjust as needed

  static const Color primaryColor = Colors.blue;
  static const Color accentColor = Colors.blueAccent;
  static const Color textColor = Colors.black87;
  static const Color backgroundColor = Colors.white;
  static const Color buttonColor = Colors.blue;
  static const Color buttonTextColor = Colors.white;
  static const Color errorColor = Colors.red;
  static const Color successColor = Colors.green;
}