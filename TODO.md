# TODO

## Phase 1: Windows & Desktop Support
- [x] Create `src/main.rs` to provide a binary entry point for desktop.
- [x] Fix Tokio runtime panic on desktop by initializing a runtime in `main.rs`.
- [x] Verify that `cargo run` works on Windows.

## Phase 2: Pressure Display UI Refinement
- [x] Ensure Target and Current pressure boxes are equal size and responsive.
- [x] Update `PressureDisplay` to show 2 decimal points and move "bar" unit below numbers.
- [x] Implement dynamic font scaling in `PressureDisplay` to maximize space usage without wrapping.

## Phase 3: Settings Screen Overhaul
- [x] Narrow numeric input fields and align them consistently.
- [x] Implement strict validation ranges (Pressure 1-10 bar, Litres 10-999 l/ha).
- [x] Implement cross-field consistency validation (Min <= Nominal <= Max).
- [x] Add [+] and [-] buttons for 0.1 bar fine-tuning.
- [x] Add real-time visual warnings for invalid or inconsistent inputs.
- [x] Display color swatch for selected nozzle size.

## Phase 4: UX & Aesthetics
- [x] Integrate custom "Michroma" font for all numeric displays and inputs.
- [x] Add icons (📊, ⚙) to navigation buttons.
- [x] Implement "Unsaved Changes" logic:
    - [x] Track dirty state compared to last saved/reset state.
    - [x] Prevent/warn when leaving Settings view with unsaved changes.

## Phase 5: Licensing & Documentation
- [x] Add MIT `LICENSE` file.
- [x] Add `LICENSE-THIRD-PARTY.md` for font attribution.
- [x] Implement in-app "About & Legal" section in Settings screen.
- [x] Update `README.md` with current build and licensing info.

## Phase 6: Logo & Branding
- [x] Research and define logo concept (Salmiac diamond + Sprayer elements).
- [x] Create SVG logo asset (`assets/logo.svg`).
- [x] Generate PNG versions for app icon and documentation.
- [x] Configure Windows executable icon.
- [x] Integrate logo into the application's UI (top panel).
- [x] Update documentation to include the logo.
- [x] Refine logo with light blue outer line.

## Phase 7: Bug Fixes & Refinement
- [x] Fix `build.rs` compilation error (missing argument for `embed_resource::compile`).
- [x] Fix `build.rs` warning (unused `CompilationResult`).

## Phase 8: Build Issues
- [x] Verify if `bincode` is used or intended for use in the codebase.
- [x] Switch to `bincode-next` (v3.0.0-rc.13) to resolve the tombstone build error without downgrading the version range.

