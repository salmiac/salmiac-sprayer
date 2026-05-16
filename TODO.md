# TODO

## Fix Windows Build and Run
- [x] Create `src/main.rs` to provide a binary entry point for desktop.
- [x] Fix Tokio runtime panic on desktop by initializing a runtime in `main.rs`.
- [x] Verify that `cargo run` works on Windows.

## Refactor Pressure Display UI
- [x] Update `PressureDisplay` to show 2 decimal points, use fixed spacing, and move "bar" below.
- [x] Update `MonitorScreen` to ensure pressure boxes are equal size and responsive.
- [x] Verify pressure display changes.
