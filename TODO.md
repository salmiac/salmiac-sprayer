# Pressure Notification Implementation

This plan covers the implementation of auditory and visual notifications for large discrepancies between target and current pressure.

## Phase 1: Preparation & Infrastructure
- [x] Add `rodio` dependency to `Cargo.toml` for audio support.
- [x] Define pressure alert threshold in `SprayerSettings` (default 0.5 bar).
- [x] Add configuration UI for pressure alert threshold in `SettingsScreen`.

## Phase 2: Visual Notification
- [x] Modify `PressureDisplay` widget to accept a `warning` state and display in red.
- [x] Implement discrepancy detection logic in `MonitorScreen`.
- [x] Add a warning banner/label to `MonitorScreen` UI when pressure is too low/high.

## Phase 3: Auditory Notification
- [x] Implement a simple `AudioService` or similar to handle beeps.
- [x] Integrate `AudioService` into `SalmiacSprayerApp`.
- [x] Trigger periodic beep when pressure discrepancy exceeds threshold and controller is active.

## Phase 4: Validation & Testing
- [x] Verify visual and auditory alerts with simulated low pressure.
- [x] Ensure no alerts occur when target pressure is 0 or controller is OFF.
- [x] Check performance and ensure no audio-related crashes on Android.
- [x] Fix Android crash by linking `c++_shared` and upgrading `rodio`.
- [x] Update `scripts/simulate_sprayer.py` with separate pressure settings for target and current.
