# Tasks

## Set Windows Corner Icon
- [x] Investigate why res/icon.rc is not being applied (Explorer icon set, but window corner requires explicit eframe call)
- [x] Add image dependency to Cargo.toml
- [x] Update src/lib.rs to explicitly set the window icon in desktop_main
- [x] Update build.rs to ensure it only runs on Windows
- [x] Verify build on Windows
