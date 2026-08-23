#!/usr/bin/env bash
set -euo pipefail

PROJECT="OXIDE Community Edition — CyberPunk2077 Interface GUI"
SRC_DIR="$(cd "$(dirname "$0")" && pwd)"

# ── CyberPunk2077 palette ───────────────────────────────────────────────
CP_YLW="255, 229, 100"
CP_YLW_B="255, 229, 100"
CP_RED="255, 65,  54"
CP_RED_B="255, 65,  54"
CP_BLU="0,   255, 255"
CP_BLU_B="0,   255, 255"
CP_GRN="0,   255, 128"
CP_GRN_B="0,   255, 128"
CP_PUR="200, 120, 255"
CP_PUR_B="200, 120, 255"
CP_ORG="255, 158, 50"
CP_ORG_B="255, 158, 50"
CP_GRY="120, 120, 140"
CP_GRY_B="120, 120, 140"
CP_BG="10,  10,  24"
CP_FG="200, 220, 255"

RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'; YELLOW='\033[1;33m'; BOLD='\033[1m'; NC='\033[0m'
info()  { echo -e "  ${CYAN}▸${NC} $1"; }
ok()    { echo -e "  ${GREEN}✔${NC} $1"; }
warn()  { echo -e "  ${YELLOW}⚠${NC} $1"; }
fail()  { echo -e "  ${RED}✘${NC} $1"; exit 1; }

verify_prereqs() {
    echo -e "\n${YELLOW}${BOLD}[Prerequisite Verification]${NC}"
    local all_ok=true
    command -v rustc &>/dev/null && ok "rustc: $(rustc --version | cut -d' ' -f2)" || { fail "rustc not found"; all_ok=false; }
    command -v cargo &>/dev/null && ok "cargo: $(cargo --version | cut -d' ' -f2)" || { fail "cargo not found"; all_ok=false; }
    command -v rustup &>/dev/null && ok "rustup: $(rustup --version | head -1 | cut -d' ' -f2)" || warn "rustup not found"
    for tool in cmake make pkg-config; do
        command -v "$tool" &>/dev/null && ok "$tool: $($tool --version 2>&1 | head -1 | grep -oP '[\d.]+' | head -1)" || { fail "$tool not found"; all_ok=false; }
    done
    command -v python3 &>/dev/null && ok "python3: $(python3 --version | cut -d' ' -f2)" || warn "python3 not found"
    if ! $all_ok; then fail "Prerequisite check failed"; fi
    echo ""
}

install_gui_deps() {
    echo -e "\n${YELLOW}${BOLD}[GUI System Libraries]${NC}"
    if dpkg -s libwebkit2gtk-4.1-dev &>/dev/null && dpkg -s libgtk-3-dev &>/dev/null; then
        ok "GUI system libraries already installed"
        return
    fi
    info "Installing GUI system libraries (requires sudo)..."
    sudo apt-get update -qq
    sudo apt-get install -y \
        libwebkit2gtk-4.1-dev \
        libgtk-3-dev \
        cmake \
        libsoup-3.0-dev \
        libjavascriptcoregtk-4.1-dev \
        libayatana-appindicator3-dev \
        pkg-config \
        libssl-dev
    ok "GUI system libraries installed"
}

usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Target:
  --release       Build in release mode (default: debug)
  --cross-win     Cross-compile for Windows (x86_64-pc-windows-gnu)
  --clean         Remove old build artifacts first
  --build-db      Rebuild encrypted SQLite database from CSVs
  --install       Copy built binary into the release folder
  --all           Full pipeline: build-db + release + install
  --help          Show this help

Examples:
  $0                          debug build
  $0 --release               release build
  $0 --release --cross-win   cross-compile Windows GUI
  $0 --release --install     build + copy to release dir
  $0 --all                   full pipeline
EOF
}

RELEASE=false
CROSS_WIN=false
CLEAN=false
BUILD_DB=false
INSTALL=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --release)    RELEASE=true ;;
        --cross-win)  CROSS_WIN=true ;;
        --clean)      CLEAN=true ;;
        --build-db)   BUILD_DB=true ;;
        --install)    INSTALL=true ;;
        --all)        BUILD_DB=true; RELEASE=true; INSTALL=true ;;
        --help)       usage; exit 0 ;;
        *) echo "Unknown: $1"; usage; exit 1 ;;
    esac
    shift
done

cd "$SRC_DIR"
verify_prereqs
install_gui_deps

if [[ "$BUILD_DB" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Database Build]${NC}"
    info "Rebuilding encrypted SQLite database from CSVs..."
    python3 tools/build_db.py
    ok "Database built"
fi

if [[ "$CLEAN" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Clean]${NC}"
    info "Cleaning GUI build artifacts..."
    cd "$SRC_DIR/gui"
    cargo clean 2>/dev/null || true
    rm -rf target 2>/dev/null || true
    ok "Cleaned"
    cd "$SRC_DIR"
fi

if [[ "$CROSS_WIN" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Cross-Compile Windows]${NC}"
    if ! rustup target list --installed 2>/dev/null | grep -q "x86_64-pc-windows-gnu"; then
        info "Installing Windows target..."
        rustup target add x86_64-pc-windows-gnu
    fi
    cd "$SRC_DIR/gui"
    if [[ "$RELEASE" == true ]]; then
        cargo build --release --target x86_64-pc-windows-gnu
        BINARY="$SRC_DIR/gui/target/x86_64-pc-windows-gnu/release/oxide-gui.exe"
    else
        cargo build --target x86_64-pc-windows-gnu
        BINARY="$SRC_DIR/gui/target/x86_64-pc-windows-gnu/debug/oxide-gui.exe"
    fi
    ok "Windows GUI: $BINARY"
    if [[ "$INSTALL" == true && -f "$BINARY" ]]; then
        INSTALL_DIR="$SRC_DIR/OxideCE-v8.5.0-community/Windows"
        mkdir -p "$INSTALL_DIR"
        cp "$BINARY" "$INSTALL_DIR/oxide-gui.exe"
        ok "Installed to $INSTALL_DIR/oxide-gui.exe"
    fi
    exit 0
fi

cd "$SRC_DIR/gui"
echo -e "\n${YELLOW}${BOLD}[Build]${NC}"
if [[ "$RELEASE" == true ]]; then
    info "Building in release mode..."
    cargo build --release
    BINARY="$SRC_DIR/gui/target/release/oxide-gui"
else
    info "Building in debug mode..."
    cargo build
    BINARY="$SRC_DIR/gui/target/debug/oxide-gui"
fi
ok "Binary: $BINARY"

if [[ "$INSTALL" == true ]]; then
    if [[ "$RELEASE" == true ]]; then
        INSTALL_DIR="$SRC_DIR/OxideCE-v8.5.0-community/Linux"
        mkdir -p "$INSTALL_DIR/bin"
        cp "$BINARY" "$INSTALL_DIR/bin/oxide-gui"
        ok "Installed to $INSTALL_DIR/bin/oxide-gui"
    else
        warn "--install only copies release builds; skipping"
    fi
fi
