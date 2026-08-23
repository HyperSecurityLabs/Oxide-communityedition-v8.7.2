#!/usr/bin/env bash
# ----------------------------------------------------------------------------
#  build-android.sh — OXIDE Android cross-compile (fully automated)
#
#  • Auto-downloads NDK r27c to ~/Desktop if missing (never /tmp)
#  • Auto-installs rustup targets + cargo-ndk when absent
#  • Fixes the CARGO_TARGET_*_LINKER env var (dashes → underscores)
#  • Low-RAM guard: forces thin LTO under 8GB
#  • Verifies ELF output and deploys to ./oxide-android-builds/<abi>/
#
#  USAGE:  ./build-android.sh [aarch64|armv7|x86_64|i686|all]
# ----------------------------------------------------------------------------

set -euo pipefail

NDK_VER="r27c"
NDK_URL="https://dl.google.com/android/repository/android-ndk-${NDK_VER}-linux.zip"
DL_DIR="$HOME/Desktop"                       # downloads live on the Desktop
NDK_BASE="$HOME/Android/ndk"
NDK="$NDK_BASE/android-ndk-${NDK_VER}"
API="${ANDROID_API:-24}"

declare -A TRIPLES=( [aarch64]=aarch64-linux-android [armv7]=armv7-linux-androideabi [x86_64]=x86_64-linux-android [i686]=i686-linux-android )
declare -A ABIS=(    [aarch64]=arm64-v8a            [armv7]=armeabi-v7a             [x86_64]=x86_64              [i686]=x86 )

# --- 1. NDK -----------------------------------------------------------------
if [[ ! -d "$NDK" ]]; then
    echo "[*] NDK ${NDK_VER} not found — downloading to ~/Desktop ..."
    FREE_MB=$(df -Pm "$HOME" | awk 'NR==2{print $4}')
    if (( FREE_MB < 6000 )); then
        echo "[!] Only ${FREE_MB}MB free — NDK needs ~5GB (zip + extracted). Abort."; exit 1
    fi
    ZIP="$DL_DIR/android-ndk-${NDK_VER}-linux.zip"
    curl -L --progress-bar -o "$ZIP" "$NDK_URL"
    mkdir -p "$NDK_BASE"
    unzip -q -o "$ZIP" -d "$NDK_BASE"
    echo "[*] NDK unpacked at $NDK (zip kept on Desktop)"
fi
export ANDROID_NDK_HOME="$NDK"
export ANDROID_NDK_ROOT="$NDK"
export PATH="$NDK/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH"
echo "[*] NDK: $NDK"

# --- 2. rustup targets ------------------------------------------------------
echo "[*] Ensuring rustup targets ..."
for t in "${TRIPLES[@]}"; do
    rustup target list --installed | grep -q "^$t$" || rustup target add "$t"
done

# --- 3. cargo-ndk (optional; manual fallback exists) ------------------------
HAVE_CNDK=1
command -v cargo-ndk >/dev/null 2>&1 || {
    echo "[*] Installing cargo-ndk ..."
    cargo install cargo-ndk --locked || HAVE_CNDK=0
}

# --- 4. low-RAM safety ------------------------------------------------------
TOTAL_KB=$(grep MemTotal /proc/meminfo | awk '{print $2}')
if (( TOTAL_KB < 8000000 )); then
    echo "[*] RAM ${TOTAL_KB}KB < 8GB → thin LTO to avoid OOM"
    export CARGO_PROFILE_RELEASE_LTO=thin
    export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
fi

# --- 5. build + deploy -------------------------------------------------------
build_one() {
    local t="$1"
    local triple="${TRIPLES[$t]}"
    local abi="${ABIS[$t]}"

    #  BUGFIX: cargo env vars use UPPER_SNAKE of the triple, e.g.
    #  CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER
    local triple_us="${triple//-/_}"
    export "CARGO_TARGET_${triple_us^^}_LINKER=${triple}-${API}-clang"

    echo "[*] Building $triple ($abi) — API $API"
    if (( HAVE_CNDK )); then
        cargo ndk -t "$abi" --platform "$API" build --release
    else
        cargo build --release --target "$triple"
    fi

    local bin="target/$triple/release/oxide"
    if [[ -f "$bin" ]] && file "$bin" | grep -q ELF; then
        mkdir -p "oxide-android-builds/$abi"
        cp "$bin" "oxide-android-builds/$abi/oxide"
        echo "[✓] $abi → oxide-android-builds/$abi/oxide ($(du -h "$bin" | cut -f1))"
    else
        echo "[✗] $abi: ELF verification failed"; return 1
    fi
}

TARGET="${1:-aarch64}"
case "$TARGET" in
    aarch64|armv7|x86_64|i686) build_one "$TARGET" ;;
    all) for t in aarch64 armv7 x86_64 i686; do build_one "$t"; done ;;
    *) echo "[!] usage: $0 [aarch64|armv7|x86_64|i686|all]"; exit 1 ;;
esac

echo "[*] Deploy: adb push oxide-android-builds/<abi>/oxide /data/local/tmp/ && adb shell chmod +x /data/local/tmp/oxide"
