# Current Tasks

## Theme and Localization Support
- [x] Add `ThemeMode` (System, Light, Dark) and `AppLanguage` enums to `src/models/sprayer_settings.rs` and update `SprayerSettings` default values.
- [x] Create a localization system (`src/i18n.rs` or `rust-i18n`) with translations for English, Finnish, Swedish, Spanish, German, French, Portuguese, and Italian.
- [x] Update `src/app.rs` to apply the selected egui theme and initialize the language setting.
- [x] Add Theme and Language selector UI components in `src/screens/settings.rs`.
- [x] Replace hardcoded UI text with localized strings across `src/screens` and `src/widgets`.