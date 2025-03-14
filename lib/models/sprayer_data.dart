// lib/models/sprayer_data.dart
import 'dart:typed_data';

class SprayerData {
  final double currentPressure;
  final double targetPressure;
  final double speed;
  final bool boomLocked;

  SprayerData({
    required this.currentPressure,
    required this.targetPressure,
    required this.speed,
    required this.boomLocked,
  });

  factory SprayerData.fromBytes(List<int> bytes, double minSpeed, double maxSpeed) {
    // Assuming binary message format is:
    // pressure (float, 4 bytes), speed (float, 4 bytes), boomLocked (bool, 1 byte)
    // You'll need to adapt this based on the actual binary format.
    if (bytes.length != 9) {
      throw FormatException('Invalid byte array length for SprayerData');
    }

    double currentPressure = bytesToFloat(bytes.sublist(0, 4));
    double targetPressure = bytesToFloat(bytes.sublist(0, 4));
    double speed = bytesToFloat(bytes.sublist(4, 8));
    bool boomLocked = bytes[8] == 1; // 1 for locked, 0 for unlocked

    return SprayerData(
      currentPressure: currentPressure,
      targetPressure: targetPressure,
      speed: speed,
      boomLocked: boomLocked,
    );
  }

  static double bytesToFloat(List<int> bytes) {
    var buffer = ByteData.view(Uint8List.fromList(bytes).buffer);
    return buffer.getFloat32(0, Endian.little); // Assuming little-endian
  }
}