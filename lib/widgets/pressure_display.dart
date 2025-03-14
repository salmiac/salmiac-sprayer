// lib/widgets/pressure_display.dart
import 'package:flutter/material.dart';
import '../utils/constants.dart';

class PressureDisplay extends StatelessWidget {
  final double pressureValue;

  const PressureDisplay({super.key, required this.pressureValue});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    // Use the theme's headline style and then override size/weight if needed.
    final valueTextStyle = theme.textTheme.headlineLarge?.copyWith(
      fontSize: 72,
      fontWeight: FontWeight.bold,
    ) ?? const TextStyle(fontSize: 80, fontWeight: FontWeight.bold);

    final unitTextStyle = theme.textTheme.labelSmall?.copyWith(
      fontSize: 18,
    ) ?? const TextStyle(fontSize: 18);

    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        RichText(
          text: TextSpan(
            children: <TextSpan>[
              TextSpan(
                text: pressureValue.toStringAsFixed(AppConstants.pressureDecimalPlaces),
                style: valueTextStyle,
              ),
              TextSpan(
                text: ' bar',
                style: unitTextStyle,
              ),
            ],
          ),
        ),
      ],
    );
  }
}