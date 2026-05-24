# <img src="assets/logo.svg" width="48" height="48" align="center"> Salmiac Sprayer

Controller UI app to be used with pi-steer along with AgOpenGPS. This application is built in Rust using the `egui` and `eframe` libraries.

## Development Setup

This guide will help you set up your development environment to build and run the application.

### Prerequisites

- **Rust**: Ensure you have a recent version of the Rust toolchain installed. You can get it from rustup.rs.

### Running on Desktop (Windows/macOS/Linux)

1.  **Clone the repository:**
    ```sh
    git clone <repository-url>
    cd salmiac-sprayer
    ```

2.  **Build and run:**
    To run the application in debug mode:
    ```sh
    cargo run
    ```

    For a more performant release build:
    ```sh
    cargo run --release
    ```

## Building for Android

### 1. Install Android Build Tools

- **Rust Targets**: Add the Android targets to your Rust toolchain.
  ```sh
  rustup target add aarch64-linux-android # 64-bit ARM
  rustup target add x86_64-linux-android    # x86_64 Emulator
  ```

- **`cargo-ndk`**: Install the tool for cross-compiling Rust to Android.
  ```sh
  cargo install cargo-ndk
  ```

- **Android SDK & NDK**: Install Android Studio and use its SDK Manager to install the "NDK (Side by side)" and "Android SDK Command-line Tools".

- **Java**: Ensure you have **Java 17 or 21** installed and your `JAVA_HOME` environment variable is set.

### 2. Build the Application

The project uses a Gradle wrapper located in the `android/` directory.

1.  **Enter the android directory:**
    ```sh
    cd android
    ```

2.  **Build the Debug APK:**
    ```sh
    ./gradlew assembleDebug
    ```
    The build automatically invokes `cargo-ndk` to compile the Rust source code and packages it into the APK.

3.  **Find the APK:**
    The finished APK is located at: `android/app/build/outputs/apk/debug/app-debug.apk`.

### 3. Run on Device / Emulator

- **Connect a device** (with USB debugging enabled) or start an Android emulator.
- **Install and run (Debug):**
  ```sh
  ./gradlew installDebug
  ```

### 4. Build for Release

To create a signed release APK for distribution:

1.  **Generate a Keystore (if missing):**
    ```sh
    keytool -genkey -v -keystore release.keystore -alias sprayer-key -keyalg RSA -keysize 2048 -validity 10000
    ```

2.  **Configure Automatic Signing:**
    Create a file at `android/keystore.properties` (ignored by git):
    ```properties
    storeFile=../release.keystore
    storePassword=your_password
    keyAlias=sprayer-key
    keyPassword=your_password
    ```

3.  **Build and Install Release:**
    ```sh
    cd android
    ./gradlew installRelease
    ```
    *Note: `installRelease` will only be available after `keystore.properties` is created.*

4.  **Manual Signing (Alternative):**
    If you prefer manual signing:
    ```sh
    ./gradlew assembleRelease
    apksigner sign --ks release.keystore --out SalmiacSprayer.apk android/app/build/outputs/apk/release/app-release-unsigned.apk
    ```

## Implementation Notes for Android

This project uses **`NativeActivity`** via `android-native-activity`. The build process is orchestrated by Gradle, which calls `cargo ndk` to produce shared libraries for multiple architectures (`arm64-v8a`, `x86_64`).

1.  **Feature Configuration**: The `eframe` dependency in `Cargo.toml` must have `default-features = false` to disable `accesskit`, which is incompatible with `NativeActivity`.
2.  **Initialization**: In `src/lib.rs`, `android_app` must be passed to `eframe::NativeOptions` to prevent runtime panics.
3.  **Layout**: A fallback top margin (30.0 points) is implemented in `src/app.rs` to avoid overlapping the Android status bar when `safe_area_insets` are not reported by the OS.


## Licensing

- **Salmiac Sprayer:** Licensed under the MIT License (see [LICENSE](LICENSE)).
- **Third-Party Components:** See [LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY.md) for details on fonts and other assets.
