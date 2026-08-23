# OXIDE CE v8.7.2 — Cross-Compilation Guide

## Rust Targets

```bash
# Linux
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu

# Android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# iOS
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim
```

---

## Kali / Parrot / Ubuntu (x86_64-linux-gnu)

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
cargo build --release --target x86_64-unknown-linux-gnu
```

---

## Linux ARM64 (aarch64-unknown-linux-gnu)

Raspberry Pi 4+, ARM servers, PinePhone, etc.

```bash
sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu pkg-config

# For cross-compiled OpenSSL
export OPENSSL_STATIC=1
export OPENSSL_LIB_DIR=/usr/lib/aarch64-linux-gnu
export OPENSSL_INCLUDE_DIR=/usr/include

cargo build --release --target aarch64-unknown-linux-gnu
```

Add to `.cargo/config.toml`:
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

---

## Android (requires Android NDK)

Download NDK r26+ from:
https://developer.android.com/ndk/downloads

### Environment Variables

```bash
# Windows
set ANDROID_NDK_HOME=C:\android-ndk-r26b
set PATH=%ANDROID_NDK_HOME%\toolchains\llvm\prebuilt\windows-x86_64\bin;%PATH%

# Linux / macOS
export ANDROID_NDK_HOME=/opt/android-ndk-r26b
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH
```

### Targets and Linkers

| Target | Linker | Use |
|--------|--------|-----|
| `aarch64-linux-android` | `aarch64-linux-android35-clang.cmd` | Modern phones (Pixel 6+, Samsung S21+) |
| `armv7-linux-androideabi` | `armv7a-linux-androideabi35-clang.cmd` | Older phones, budget devices |
| `x86_64-linux-android` | `x86_64-linux-android35-clang.cmd` | Emulators (x86_64) |
| `i686-linux-android` | `i686-linux-android35-clang.cmd` | Emulators (x86) |

### Build Commands

```bash
# ARM64 (most modern phones)
cargo build --release --target aarch64-linux-android

# ARM32 (older/budget phones)
cargo build --release --target armv7-linux-androideabi

# Emulator x86_64
cargo build --release --target x86_64-linux-android
```

### `.cargo/config.toml`

```toml
[target.aarch64-linux-android]
linker = "aarch64-linux-android35-clang.cmd"

[target.armv7-linux-androideabi]
linker = "armv7a-linux-androideabi35-clang.cmd"

[target.x86_64-linux-android]
linker = "x86_64-linux-android35-clang.cmd"

[target.i686-linux-android]
linker = "i686-linux-android35-clang.cmd"
```

---

## iOS (requires macOS + Xcode)

**Cannot cross-compile from Windows or Linux.** Must use macOS.

```bash
xcode-select --install
```

### Targets

| Target | Use |
|--------|-----|
| `aarch64-apple-ios` | iPhone / iPad (physical devices) |
| `x86_64-apple-ios` | iOS Simulator (Intel Mac) |
| `aarch64-apple-ios-sim` | iOS Simulator (Apple Silicon Mac) |

### Build Commands

```bash
# Physical device
cargo build --release --target aarch64-apple-ios

# Simulator (Apple Silicon)
cargo build --release --target aarch64-apple-ios-sim
```

---

## Embedded TLS Certificate

The `oxide.crt` Burp CA cert is embedded via `include_bytes!` in `lib.rs`. It compiles into every target automatically. No extra files needed at runtime for `--burp` mode.

---

## Notes

- `pnet` raw socket features are `#[cfg(target_os = "linux")]` — Android compiles them but sandboxing blocks raw sockets unless rooted. Falls back to HTTP-level fingerprinting.
- `rusqlite` uses `bundled` feature — SQLite is statically linked, no system sqlite3 dependency on any target.
- `reqwest` uses `rustls-tls` — no OpenSSL dependency required on any target.
- Binary size: ~8 MB (release, stripped). Use `strip` or `upx` to compress further for mobile.
