import 'dart:io';
import 'dart:async';
import '../models/sprayer_data.dart' as sprayer_data;
import '../models/sprayer_settings.dart' as sprayer_settings;
import 'package:flutter/foundation.dart';

class UDPButtonStateSender {
  final String targetIp;
  final int targetPort;

  UDPButtonStateSender({required this.targetIp, this.targetPort = 8888});

  /// Sends a UDP message encoding the controller and variable pressure (constant pressure)
  /// button states as bits in a single byte.
  Future<void> sendButtonState(bool activated, bool constantPressure) async {
    // Define a unique header for the button state message.
    List<int> data = [0xAA, 0xAB, 0x70, 0x71, 0x01];

    // Encode button states as bits in a single byte.
    // Bit0: Controller activated state.
    // Bit1: Constant pressure state.
    int buttonStates = 0;
    if (activated) {
      buttonStates |= 0x01; // Set bit 0.
    }
    if (constantPressure) {
      buttonStates |= 0x02; // Set bit 1.
    }
    data.add(buttonStates);

    // Calculate the checksum (CRC) from bytes starting at index 2 up to this point.
    int crc = data.sublist(2).fold(0, (int acc, int val) => (acc + val) & 0xFF);
    data.add(crc);

    if (kDebugMode) {
      print('Sending button state message: $data');
    }

    // Send the data via UDP.
    RawDatagramSocket socket =
        await RawDatagramSocket.bind(InternetAddress.anyIPv4, 0);
    socket.send(data, InternetAddress(targetIp), targetPort);
    socket.close();
  }
}

class UDPSettingsSender {
  final String targetIp;
  final int targetPort;

  UDPSettingsSender({required this.targetIp, this.targetPort = 8888});

  Future<void> sendSettings(sprayer_settings.SprayerSettings settings) async {
    // Define a unique header for settings packet: [0x90, 0x91, 0x70, 0x70, 0x06]
    List<int> data = [0x90, 0x91, 0x70, 0x70, 0x06];

    // Convert each double value to an integer after scaling by 100.
    // This preserves two decimals.
    int nozzleSizeInt = (settings.nozzleSize.sizeValue * 100).round();
    int litresPerHaInt = (settings.litresPerHa * 100).round();
    int minPressureInt = (settings.minPressure * 100).round();
    int maxPressureInt = (settings.maxPressure * 100).round();
    int nominalPressureInt = (settings.nominalPressure * 100).round();
    int nozzleSpacingInt = (settings.nozzleSpacing * 100).round();

    // Append each value as 2 bytes in little-endian.
    data.addAll(_uInt16ToBytes(nozzleSizeInt));
    data.addAll(_uInt16ToBytes(litresPerHaInt));
    data.addAll(_uInt16ToBytes(minPressureInt));
    data.addAll(_uInt16ToBytes(maxPressureInt));
    data.addAll(_uInt16ToBytes(nominalPressureInt));
    data.addAll(_uInt16ToBytes(nozzleSpacingInt));

    // Calculate the checksum (CRC) from bytes starting at index 2 up to this point.
    int crc = data.sublist(2).fold(0, (int acc, int val) => (acc + val) & 0xFF);
    data.add(crc);

    // Send the data via UDP.
    RawDatagramSocket socket =
        await RawDatagramSocket.bind(InternetAddress.anyIPv4, 0);
    socket.send(data, InternetAddress(targetIp), targetPort);
    socket.close();
  }

  // Helper: convert a 16-bit integer to little-endian bytes.
  List<int> _uInt16ToBytes(int value) {
    return [value & 0xFF, (value >> 8) & 0xFF];
  }
}

class UDPReceiver {
  RawDatagramSocket? _socket;
  final int port;
  // Callback to deliver parsed sprayer data.
  final void Function(sprayer_data.SprayerData) onDataReceived;

  UDPReceiver({this.port = 1111, required this.onDataReceived});

  Future<void> startListening() async {
    _socket = await RawDatagramSocket.bind(InternetAddress.anyIPv4, port,
        reuseAddress: true, reusePort: true);
    if (kDebugMode) {
      print('UDP Receiver listening on ${_socket!.address.address}:$port');
    }

    _socket!.listen((RawSocketEvent event) {
      if (event == RawSocketEvent.read) {
        final datagram = _socket!.receive();
        if (datagram != null) {
          _processData(datagram.data);
        }
      }
    });
  }

  void _processData(Uint8List data) {
    // Expected total length is 11 bytes:
    // [0x80, 0x81, 0x70, 0x70, 0x05] + 2 bytes target pressure +
    // 2 bytes current pressure + 1 byte boom locked + 1 byte CRC
    if (data.length != 13) {
      if (kDebugMode) {
        print('Invalid data length: ${data.length}');
      }
      return;
    }

    // Validate header bytes.
    if (!(data[0] == 0x80 &&
        data[1] == 0x81 &&
        data[2] == 0x70 &&
        data[3] == 0x70 &&
        data[4] == 0x07)) {
      if (kDebugMode) {
        print('Invalid header.');
      }
      return;
    }

    // Parse target pressure (little endian conversion).
    int targetPressureInt = data[5] | (data[6] << 8);
    double targetPressure = targetPressureInt / 100.0;

    // Parse current pressure.
    int currentPressureInt = data[7] | (data[8] << 8);
    double currentPressure = currentPressureInt / 100.0;

    // Parse current speed.
    int speedInt = data[9] | (data[10] << 8);
    double speed = speedInt / 100.0;

    // Parse boom locked status.
    bool boomLocked = data[11] == 1;

    // Calculate the checksum (CRC) from bytes at index 2 to before last byte.
    int calculatedCrc = data.sublist(2, data.length - 1).fold(
        0, (int acc, int element) => (acc + element) & 0xFF);
    int receivedCrc = data[12];
    if (calculatedCrc != receivedCrc) {
      if (kDebugMode) {
        print(
            'CRC mismatch. Expected: $calculatedCrc, Received: $receivedCrc');
      }
      return;
    }

    // Create a sprayer data object.
    // Here we assume your model constructor accepts these named parameters.
    final sprayerData = sprayer_data.SprayerData(
      targetPressure: targetPressure,
      currentPressure: currentPressure,
      boomLocked: boomLocked,
      speed: speed,

    );

    // Pass the data to the provided callback.
    onDataReceived(sprayerData);
  }

  void stopListening() {
    _socket?.close();
  }
}

class ControllerService {
  final String controllerIp;
  final int controllerPort;
  Socket? _tcpSocket;
  UDPReceiver? _udpReceiver;
  double _currentSpeed = 0.0;
  List<double> _speeds = []; // new: store all speeds
  final StreamController<sprayer_data.SprayerData> _dataStreamController =
      StreamController<sprayer_data.SprayerData>.broadcast();
  Stream<sprayer_data.SprayerData> get dataStream => _dataStreamController.stream;

  ControllerService({required this.controllerIp, required this.controllerPort});

  get currentSpeed => _currentSpeed;
  // get minSpeed => _speeds.isNotEmpty ? _speeds.reduce((a, b) => a < b ? a : b) : 0.0; // compute min
  // get maxSpeed => _speeds.isNotEmpty ? _speeds.reduce((a, b) => a > b ? a : b) : 0.0; // compute max

  Future<void> startUDPListener() async {
    if (_udpReceiver != null) return; // Already listening
    _udpReceiver = UDPReceiver(
      port: 1111,
      onDataReceived: (sprayer_data.SprayerData data) {
        _currentSpeed = data.speed;
        _speeds.add(data.speed); // add new speed to list
        _dataStreamController.add(data);
      },
    );
    await _udpReceiver!.startListening();
    if (kDebugMode) {
      print('UDP listener started.');
    }
  }

  Future<void> stopUDPListener() async {
    _udpReceiver?.stopListening();
    _udpReceiver = null;
    if (kDebugMode) {
      print('UDP listener stopped');
    }
  }

  Future<bool> connectTCP() async {
    try {
      _tcpSocket = await Socket.connect(controllerIp, controllerPort,
          timeout: Duration(seconds: 5));
      if (kDebugMode) {
        print('TCP connected to $controllerIp:$controllerPort');
      }
      return true;
    } catch (e) {
      if (kDebugMode) {
        print('Failed to connect to TCP $controllerIp:$controllerPort: $e');
      }
      _tcpSocket?.destroy();
      _tcpSocket = null;
      return false;
    }
  }

  Future<void> disconnectTCP() async {
    await _tcpSocket?.close();
    _tcpSocket = null;
    if (kDebugMode) {
      print('TCP disconnected');
    }
  }

  void dispose() {
    stopUDPListener();
    disconnectTCP();
    _dataStreamController.close();
  }
}