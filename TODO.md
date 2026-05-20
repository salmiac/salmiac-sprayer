# cargo-ndk Migration TODO

## Step 1: Clean Up
- [x] Remove `cargo-apk` specific metadata from `Cargo.toml`.
- [x] Remove temporary `fix_icon.ps1`, `AndroidManifest.xml`, `AndroidManifest.xml.template`, `patched.apk`, `test_manifest.apk`, and `salmiac_sprayer_fixed.apk`.
- [x] Remove `staging` directory.

## Step 2: Create Android Structure
- [x] Create `android/app/src/main/res` and `android/app/src/main/assets`.
- [x] Move/copy icons and other resources to `android/app/src/main/res`.

## Step 3: Gradle Configuration
- [x] Create `android/settings.gradle`.
- [x] Create `android/build.gradle`.
- [x] Create `android/app/build.gradle`.

## Step 4: Create AndroidManifest.xml
- [x] Create `android/app/src/main/AndroidManifest.xml`.

## Step 5: Verification
- [x] Build with Gradle: `cd android && ./gradlew assembleDebug`.
- [x] Verify APK badging for icon and label.
