import 'package:flutter/material.dart';

class SpeedDisplay extends StatefulWidget {
  final double speedValue;
  final double minSpeed;
  final double maxSpeed;

  const SpeedDisplay({
    super.key,
    required this.speedValue,
    required this.minSpeed,
    required this.maxSpeed,
  });

  @override
  _SpeedDisplayState createState() => _SpeedDisplayState();
}

class _SpeedDisplayState extends State<SpeedDisplay> {
  late double _speedValue;
  late double _minSpeed;
  late double _maxSpeed;

  @override
  void initState() {
    super.initState();
    _speedValue = widget.speedValue;
    _minSpeed = widget.minSpeed;
    _maxSpeed = widget.maxSpeed;
  }

  // Update local state when widget properties change.
  @override
  void didUpdateWidget(covariant SpeedDisplay oldWidget) {
    super.didUpdateWidget(oldWidget);
    bool needsUpdate = false;
    if (widget.speedValue != oldWidget.speedValue) {
      _speedValue = widget.speedValue;
      needsUpdate = true;
    }
    if (widget.minSpeed != oldWidget.minSpeed) {
      _minSpeed = widget.minSpeed;
      needsUpdate = true;
    }
    if (widget.maxSpeed != oldWidget.maxSpeed) {
      _maxSpeed = widget.maxSpeed;
      needsUpdate = true;
    }
    if (needsUpdate) {
      setState(() {}); // trigger rebuild if any value changed
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final valueTextStyle = theme.textTheme.headlineLarge?.copyWith(
          fontSize: 96,
          fontWeight: FontWeight.bold,
        ) ??
        const TextStyle(fontSize: 96, fontWeight: FontWeight.bold);
    final minMaxValueTextStyle = theme.textTheme.headlineLarge?.copyWith(
          fontSize: 32,
          fontWeight: FontWeight.bold,
        ) ??
        const TextStyle(fontSize: 32, fontWeight: FontWeight.bold);
    final unitTextStyle = theme.textTheme.labelSmall?.copyWith(
          fontSize: 18,
        ) ??
        const TextStyle(fontSize: 18);
    final labelTextStyle = theme.textTheme.bodyMedium;

    return Column(
      children: [
        // Row with Min and Max speeds
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceEvenly,
          children: [
            // Min speed display
            Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Text('Min', style: labelTextStyle),
                RichText(
                  text: TextSpan(
                    children: <TextSpan>[
                      TextSpan(
                        text: _minSpeed.toStringAsFixed(1),
                        style: minMaxValueTextStyle,
                      ),
                      TextSpan(
                        text: ' km/h',
                        style: unitTextStyle,
                      ),
                    ],
                  ),
                ),
              ],
            ),
            // Max speed display
            Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                Text('Max', style: labelTextStyle),
                RichText(
                  text: TextSpan(
                    children: <TextSpan>[
                      TextSpan(
                        text: _maxSpeed.toStringAsFixed(1),
                        style: minMaxValueTextStyle,
                      ),
                      TextSpan(
                        text: ' km/h',
                        style: unitTextStyle,
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ],
        ),
        const SizedBox(height: 16),
        // Single Row for main Speed display
        Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Text('Speed', style: labelTextStyle),
            RichText(
              text: TextSpan(
                children: <TextSpan>[
                  TextSpan(
                    text: _speedValue.toStringAsFixed(1),
                    style: valueTextStyle,
                  ),
                  TextSpan(
                    text: ' km/h',
                    style: unitTextStyle,
                  ),
                ],
              ),
            ),
          ],
        ),
      ],
    );
  }
}