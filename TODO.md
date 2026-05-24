# Current Tasks

## Network Configuration Feature
- [x] Add `target_ip` field (String, default "255.255.255.255") to `SprayerSettings` struct in `src/models/sprayer_settings.rs`.
- [x] Update `SettingsScreen` UI in `src/screens/settings.rs` to display and edit the `target_ip` field.
- [x] Add IP format validation logic in `SettingsScreen` (must be a valid IPv4 address).
- [x] Update `app.rs` to use `self.sprayer_settings.target_ip` instead of hardcoded `"255.255.255.255"` when making controller service calls.
- [x] Add unit tests for network configuration settings.

## UI Polish & Bug Fixes
- [x] Increase the width of the Target IP edit box in `src/screens/settings.rs` to ensure a full IPv4 address (up to 15 characters) fits without horizontal scrolling or clipping when using the Monospace 24pt font.
- [x] Verify the UI changes by compiling the application.

## Target IP Input Size Tuning
- [x] Decrease the font size of the Target IP edit box in `src/screens/settings.rs` from `24.0` to `16.0` (using `FontFamily::Monospace`) so that it fits comfortably on all screen sizes.
- [x] Verify the compilation and test suite passes.

## Nozzle Size Selection UI Enhancements
- [x] Increase the selected text font size of the Nozzle Size `ComboBox` selector in `src/screens/settings.rs` from standard size to `24.0` (using proportional font for readability).
- [x] Add `+` (next) and `-` (previous) buttons on either side of the Nozzle Size selector in `src/screens/settings.rs` to allow quick cycling through available nozzle types.
- [x] Verify the UI changes by compiling the application and running the unit tests.

## Nozzle Selection Font Size Tuning
- [x] Decrease the always-visible selected text font size in `src/screens/settings.rs` from `24.0` to `18.0` (Proportional).
- [x] Increase the dropdown list item font size of the Nozzle Size ComboBox options in `src/screens/settings.rs` to `18.0` (Proportional) for improved readability and tap targets.
- [x] Verify the changes by compiling the application and running the test suite.

## Monitor UI Layout Improvements
- [x] Center-align the Speed panel (including Min/Max and Main Speed rows) in `src/widgets/speed_display.rs` and `src/screens/monitor.rs`.
- [x] Change the Target and Current pressure displays in `src/screens/monitor.rs` from side-by-side columns to a vertical, center-aligned layout.
- [x] Verify the layout and UI changes by compiling and running the application.

## Speed Display Alignment and Pressure Unit Placement Fixes
- [x] Restore original internal layout in `src/widgets/speed_display.rs` and center-align the whole speed panel frame in `src/screens/monitor.rs`.
- [x] Move the "bar" unit text to the same line as the pressure digits in `src/widgets/pressure_display.rs` and ensure horizontal center alignment.
- [x] Verify the layout and compilation of all screens and widgets.

## Fix Pressure Box Size Bug
- [x] Tune font size calculation in `src/widgets/pressure_display.rs` by dividing available width by `7.6` and clamping between `32.0` and `64.0`.
- [x] Verify compilation and run cargo tests.

## Fix Pressure Box Height Expansion
- [x] Change egui Layout cross_align to Align::Min in src/widgets/pressure_display.rs to prevent vertical expansion.
- [x] Verify layout and compilation.

## Speed and Pressure Component Alignment
- [x] Vertically center 'bar' unit text in src/widgets/pressure_display.rs and horizontally center the component content.
- [x] Center align the current speed display in src/widgets/speed_display.rs and vertically center its 'km/h' units relative to digits.
- [x] Verify layout via cargo check.

## Minor Code Cleanup
- [x] Fix unused mut warnings in src/widgets/speed_display.rs.

## Revert Speed and Pressure Component Alignment
- [x] Restore speed_display.rs Min/Max rows to original layout to fix Max speed disappearance.
- [x] Restore pressure_display.rs to use standard baseline alignment for 'bar' unit text.
- [x] Verify layout via cargo check.
- [x] Verify layout via cargo check.

 # #   H o r i z o n t a l   C e n t e r i n g   F i x 
 -   [ x ]   F i x   h o r i z o n t a l   c e n t e r i n g   i n   s r c / w i d g e t s / p r e s s u r e _ d i s p l a y . r s   u s i n g   a l l o c a t e _ u i _ w i t h _ l a y o u t   t o   e n s u r e   c e n t e r i n g   w o r k s   w i t h o u t   e x p a n d i n g   v e r t i c a l l y . 
 -   [ x ]   F i x   h o r i z o n t a l   c e n t e r i n g   i n   s r c / w i d g e t s / s p e e d _ d i s p l a y . r s   u s i n g   a l l o c a t e _ u i _ w i t h _ l a y o u t   f o r   t h e   m a i n   s p e e d   r o w .  
 
 # #   H o r i z o n t a l   C e n t e r i n g   F i n a l   F i x 
 -   [ x ]   R e f a c t o r e d   p r e s s u r e _ d i s p l a y . r s   a n d   s p e e d _ d i s p l a y . r s   t o   u s e   e g u i : : t e x t : : L a y o u t J o b   t o   c o m b i n e   d i g i t s   a n d   u n i t s   i n t o   a   s i n g l e   w i d g e t ,   a l l o w i n g   f l a w l e s s   h o r i z o n t a l   c e n t e r i n g   v i a   u i . v e r t i c a l _ c e n t e r e d   w i t h o u t   l a y o u t   e x p a n s i o n   s i d e - e f f e c t s .  

## Logo Adjustments
- [x] Make light blue salmiac shape outline thicker (stroke-width 24) in assets/logo.svg for better visibility on small icons.

## Logo Assets Generation
- [x] Write a script to convert `assets/logo.svg` to PNGs of sizes 48, 64, 72, 96, 128, 144, 192, 256, and 512.
- [x] Run the generation script and verify PNGs are updated in `assets/`.
- [x] Generate `icon.ico` from the exported PNGs and update `assets/icon.ico`.
- [x] Copy the generated PNGs and `icon.ico` to their respective Android resource paths (`android/app/src/main/res/` and `android/app/src/main/assets/`).

## GitHub Actions Release Workflow
- [x] Create `.github/workflows/release.yml`.
- [x] Configure workflow to trigger on push to `master` branch.
- [x] Add a Windows build job (`cargo build --release`).
- [x] Add an Android build job (`cd android && ./gradlew assembleRelease`) with required dependencies.
- [x] Add a job/step to create a GitHub Release and upload the Windows `.exe` and Android `.apk` files.