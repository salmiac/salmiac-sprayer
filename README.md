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
  rustup target add armv7-linux-androideabi # 32-bit ARM
  rustup target add x86_64-linux-android    # x86_64 Emulator
  ```

- **`cargo-apk`**: Install the cargo subcommand for building Android APKs.
  ```sh
  cargo install cargo-apk
  ```

- **Android SDK & NDK**: The easiest way is to install Android Studio and use its SDK Manager to install the "NDK (Side by side)" and "Android SDK Command-line Tools".

### 2. Configure Environment

`cargo-apk` needs to know where your SDK and NDK are. Set the following environment variables, pointing to the paths where you installed them.

**Linux/macOS (.bashrc, .zshrc):**
```sh
export ANDROID_SDK_ROOT="$HOME/Android/Sdk"
export ANDROID_NDK_ROOT="$ANDROID_SDK_ROOT/ndk/<your-ndk-version>"
```

**Windows (PowerShell):**
```powershell
$env:ANDROID_SDK_ROOT = "$env:LOCALAPPDATA\Android\Sdk"
$env:ANDROID_NDK_ROOT = "$env:ANDROID_SDK_ROOT\ndk\<your-ndk-version>"
```
> **Note**: Replace `<your-ndk-version>` with the actual version installed, e.g., `26.1.10909125`.

### 3. Configure Project Metadata

For `cargo-apk` to package the app correctly, ensure your `Cargo.toml` contains the following metadata.

```toml
[package.metadata.android]
label = "Salmiac Sprayer"
apk_name = "salmiac_sprayer"
min_sdk_version = 21
target_sdk_version = 33
permissions = ["android.permission.INTERNET"]
```

### 4. Build and Run

- **Connect a device** (with USB debugging enabled) or start an Android emulator.

- **Build and Run (Debug):**
  ```sh
  cargo apk run --lib
  ```
  > **Important**: The `--lib` flag is required because the project contains both a binary and a library target.

- **Build for Release:**
  Building a release APK requires a signed keystore.

  1. **Generate a keystore (if you don't have one):**
     ```sh
     keytool -genkey -v -keystore my-release-key.keystore -alias my-key-alias -keyalg RSA -keysize 2048 -validity 10000
     ```
  
  2. **Configure `Cargo.toml`:** Add the following to your `Cargo.toml`, replacing the values with your credentials. **Do not commit passwords or the keystore to a public repository.**
     ```toml
     [package.metadata.android.signing.release]
     path = "my-release-key.keystore"
     keystore_password = "your-keystore-password"
     key_alias = "my-key-alias"
     key_password = "your-key-password"
     ```

  3. **Build the APK:**
     ```sh
     cargo apk build --release --lib
     ```

### Implementation Notes for Android

This project uses **`NativeActivity`** via `android-native-activity` to maintain compatibility with `cargo-apk`. 

1.  **Feature Configuration**: The `eframe` dependency in `Cargo.toml` must have `default-features = false` to disable `accesskit`, which is incompatible with `NativeActivity`.
2.  **Initialization**: In `src/lib.rs`, `android_app` must be passed to `eframe::NativeOptions` to prevent runtime panics.
3.  **Layout**: A fallback top margin (30.0 points) is implemented in `src/app.rs` to avoid overlapping the Android status bar when `safe_area_insets` are not reported by the OS.


## Licensing

- **Salmiac Sprayer:** Licensed under the MIT License (see [LICENSE](LICENSE)).
- **Third-Party Components:** See [LICENSE-THIRD-PARTY](LICENSE-THIRD-PARTY.md) for details on fonts and other assets.

