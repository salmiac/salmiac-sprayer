import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import '../models/sprayer_settings.dart';

class SprayerSettingsStorage {
  static const String _key = 'sprayer_settings';

  static Future<void> saveSettings(SprayerSettings settings) async {
    final prefs = await SharedPreferences.getInstance();
    final settingsJson = jsonEncode(settings.toMap());
    await prefs.setString(_key, settingsJson);
  }

  static Future<SprayerSettings?> loadSettings() async {
    final prefs = await SharedPreferences.getInstance();
    final settingsJson = prefs.getString(_key);
    if (settingsJson == null) return null;
    final settingsMap = jsonDecode(settingsJson);
    return SprayerSettings.fromMap(settingsMap);
  }
}
