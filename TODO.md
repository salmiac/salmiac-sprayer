# Current Tasks

## Documentation Update
- [x] Update `README.md` to include information about automated CI/CD builds via GitHub Actions.
- [x] Update `README.md` to mention the new network configuration features (Configurable Network Target IP).
- [x] Add a section in `README.md` explaining the automated formatting and linting checks.

## CI Fixes
- [x] Install `libasound2-dev` via `sudo apt-get install -y libasound2-dev` before running `cargo clippy` in `.github/workflows/ci.yml`.
- [x] Add `chmod +x ./gradlew` before running the Android build step in `.github/workflows/release.yml` to fix the permission denied error.
- [x] Fix Windows binary name in `.github/workflows/release.yml` from `salmiac-sprayer.exe` to `salmiac-sprayer-app.exe`.

## Windows Icon Fix
- [x] Update `build.rs` to compile `assets/icon.rc` instead of the non-existent `res/icon.rc`.
- [x] Update `assets/icon.rc` to correctly reference the icon path as `1 ICON "icon.ico"`.