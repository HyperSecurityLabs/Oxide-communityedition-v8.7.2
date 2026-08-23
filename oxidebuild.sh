#!/usr/bin/env bash
set -euo pipefail

PROJECT="OXIDE Community Edition v8.7.2"
SRC_DIR="$(cd "$(dirname "$0")" && pwd)"
DISPLAY_FILE="$SRC_DIR/src/cli/display.rs"

# ── Tokyo Night palette ────────────────────────────────────────────────
TN_BG="40,  40,  40"
TN_FG="192, 202, 245"
TN_FG0="192, 202, 245"
TN_RED="247, 118, 142"
TN_RED_B="247, 118, 142"
TN_GRN="158, 206, 106"
TN_GRN_B="158, 206, 106"
TN_YLW="224, 175, 104"
TN_YLW_B="224, 175, 104"
TN_BLU="122, 162, 247"
TN_BLU_B="122, 162, 247"
TN_PUR="187, 154, 247"
TN_PUR_B="187, 154, 247"
TN_AQU="125, 207, 255"
TN_AQU_B="125, 207, 255"
TN_ORG="255, 158, 100"
TN_ORG_B="255, 158, 100"
TN_GRY="86,  95,  137"
TN_GRY_B="86,  95,  137"

# ── Gruvbox Dark palette ──────────────────────────────────────────────
GB_BG="40,  40,  40"
GB_FG="235, 219, 178"
GB_FG0="251, 241, 199"
GB_RED="204, 36,  29"
GB_RED_B="251, 73,  52"
GB_GRN="152, 151, 26"
GB_GRN_B="184, 187, 38"
GB_YLW="215, 153, 33"
GB_YLW_B="250, 189, 47"
GB_BLU="69,  133, 136"
GB_BLU_B="131, 165, 152"
GB_PUR="177, 98,  134"
GB_PUR_B="211, 134, 155"
GB_AQU="104, 157, 106"
GB_AQU_B="142, 192, 124"
GB_ORG="214, 93,  14"
GB_ORG_B="254, 128, 25"
GB_GRY="146, 131, 116"
GB_GRY_B="168, 153, 132"

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

usage() {
    cat <<EOF
Usage: $0 [OPTIONS]

Themes:
  --tokyo-night    Apply Tokyo Night theme to display colors
  --revert-theme   Revert to original Gruvbox Dark

Database:
  --build-db       Rebuild encrypted SQLite DB from CSVs

Build:
  --release        Build in release mode (default: debug)
  --cross-win      Cross-compile for Windows (x86_64-pc-windows-gnu)
  --gui            Build CLI + GUI together
  --clean          Remove old build artifacts first
  --install        Copy release binaries into the release folder
  --all            Full pipeline: build-db + release + gui + install
  --help           Show this help

Examples:
  $0                                debug build, gruvbox
  $0 --release                      release build
  $0 --tokyo-night --release        release build, tokyo night
  $0 --release --gui                release build with GUI
  $0 --release --gui --install      build + copy to release dir
  $0 --build-db                     rebuild encrypted DB only
  $0 --release --cross-win          cross-compile Windows binary
EOF
}

TOKYO=false
REVERT=false
RELEASE=false
BUILD_DB=false
BUILD_GUI=false
CROSS_WIN=false
CLEAN=false
INSTALL=false

while [[ $# -gt 0 ]]; do
    case "$1" in
        --tokyo-night)  TOKYO=true ;;
        --revert-theme) REVERT=true ;;
        --build-db)     BUILD_DB=true ;;
        --release)      RELEASE=true ;;
        --gui)          BUILD_GUI=true ;;
        --cross-win)    CROSS_WIN=true ;;
        --clean)        CLEAN=true ;;
        --install)      INSTALL=true ;;
        --all)          BUILD_DB=true; RELEASE=true; BUILD_GUI=true; INSTALL=true ;;
        --help)         usage; exit 0 ;;
        *) echo "Unknown: $1"; usage; exit 1 ;;
    esac
    shift
done

cd "$SRC_DIR"
verify_prereqs

# ── Theme ──
apply_theme() {
    local prefix="$1"
    shift
    while [[ $# -gt 0 ]]; do
        local name="$1" val="$2"
        local pad
        pad=$(printf '%*s' $((13 - ${#name})) '')
        sed -i "s/^pub const ${prefix}${name}:.*/pub const ${prefix}${name}:${pad} (u8, u8, u8) = (${val});/" "$DISPLAY_FILE"
        shift 2
    done
}

if [[ "$TOKYO" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Theme: Tokyo Night]${NC}"
    apply_theme "GB_" \
        BG    "$TN_BG"    FG    "$TN_FG"    FG0   "$TN_FG0" \
        RED   "$TN_RED"   RED_B "$TN_RED_B" \
        GRN   "$TN_GRN"   GRN_B "$TN_GRN_B" \
        YLW   "$TN_YLW"   YLW_B "$TN_YLW_B" \
        BLU   "$TN_BLU"   BLU_B "$TN_BLU_B" \
        PUR   "$TN_PUR"   PUR_B "$TN_PUR_B" \
        AQU   "$TN_AQU"   AQU_B "$TN_AQU_B" \
        ORG   "$TN_ORG"   ORG_B "$TN_ORG_B" \
        GRY   "$TN_GRY"   GRY_B "$TN_GRY_B"
    ok "Tokyo Night applied"
fi

if [[ "$REVERT" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Theme: Gruvbox Dark]${NC}"
    apply_theme "GB_" \
        BG    "$GB_BG"    FG    "$GB_FG"    FG0   "$GB_FG0" \
        RED   "$GB_RED"   RED_B "$GB_RED_B" \
        GRN   "$GB_GRN"   GRN_B "$GB_GRN_B" \
        YLW   "$GB_YLW"   YLW_B "$GB_YLW_B" \
        BLU   "$GB_BLU"   BLU_B "$GB_BLU_B" \
        PUR   "$GB_PUR"   PUR_B "$GB_PUR_B" \
        AQU   "$GB_AQU"   AQU_B "$GB_AQU_B" \
        ORG   "$GB_ORG"   ORG_B "$GB_ORG_B" \
        GRY   "$GB_GRY"   GRY_B "$GB_GRY_B"
    ok "Original theme restored"
fi

# ── Clean ──
if [[ "$CLEAN" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Clean]${NC}"
    info "Cleaning build artifacts..."
    cargo clean 2>/dev/null || true
    rm -rf target 2>/dev/null || true
    ok "Cleaned"
fi

# ── Database ──
if [[ "$BUILD_DB" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Database Build]${NC}"
    info "Rebuilding encrypted SQLite database from CSVs..."
    python3 tools/build_db.py
    ok "Database built"
fi

# ── Cross-compile Windows ──
if [[ "$CROSS_WIN" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Cross-Compile Windows]${NC}"
    if ! rustup target list --installed 2>/dev/null | grep -q "x86_64-pc-windows-gnu"; then
        info "Installing Windows target..."
        rustup target add x86_64-pc-windows-gnu
    fi
    TARGET="x86_64-pc-windows-gnu"
    BIN_NAME=$(grep -m1 '^name *= *"' "$SRC_DIR/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
    BUILD_FLAGS="--target $TARGET"
    [[ "$RELEASE" == true ]] && BUILD_FLAGS="$BUILD_FLAGS --release"
    info "Building $BIN_NAME for Windows..."
    cargo build $BUILD_FLAGS -j"$(nproc)"
    TARGET_DIR="$SRC_DIR/target/$TARGET/$([[ "$RELEASE" == true ]] && echo release || echo debug)"
    OK "Windows binary: $TARGET_DIR/$BIN_NAME.exe"
    if [[ "$INSTALL" == true && "$RELEASE" == true ]]; then
        INSTALL_DIR="$SRC_DIR/OxideCE-v8.5.0-community/Windows"
        mkdir -p "$INSTALL_DIR"
        cp "$TARGET_DIR/$BIN_NAME.exe" "$INSTALL_DIR/oxide-ce.exe"
        ok "Installed to $INSTALL_DIR/oxide-ce.exe"
    fi
    exit 0
fi

# ── Main build ──
BIN_NAME=$(grep -m1 '^name *= *"' "$SRC_DIR/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
TARGET="${CARGO_BUILD_TARGET:-$(rustc -vV | grep host | awk '{print $2}')}"
TARGET_DIR="$SRC_DIR/target/$TARGET/$([[ "$RELEASE" == true ]] && echo release || echo debug)"

BUILD_FLAGS="--target $TARGET"
[[ "$RELEASE" == true ]] && BUILD_FLAGS="$BUILD_FLAGS --release"

echo -e "\n${YELLOW}${BOLD}[Build]${NC}"
info "Building $BIN_NAME (target: $TARGET, mode: $([[ "$RELEASE" == true ]] && echo release || echo debug))..."
cargo build $BUILD_FLAGS -j"$(nproc)"
ok "Binary: $TARGET_DIR/$BIN_NAME"

if [[ "$BUILD_GUI" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Build GUI]${NC}"
    info "Building oxide-gui..."
    cargo build $BUILD_FLAGS --manifest-path "$SRC_DIR/gui/Cargo.toml" -j"$(nproc)"
    GUI_BIN=$(grep -m1 '^name *= *"' "$SRC_DIR/gui/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/')
    GUI_TARGET_DIR="$SRC_DIR/gui/target/$TARGET/$([[ "$RELEASE" == true ]] && echo release || echo debug)"
    if [[ -f "$GUI_TARGET_DIR/$GUI_BIN" ]]; then
        ok "GUI binary: $GUI_TARGET_DIR/$GUI_BIN"
    else
        warn "GUI binary not found (check gui/target)"
    fi
fi

# ── Install ──
if [[ "$INSTALL" == true && "$RELEASE" == true ]]; then
    echo -e "\n${YELLOW}${BOLD}[Install]${NC}"
    INSTALL_DIR="$SRC_DIR/OxideCE-v8.5.0-community/Linux"
    mkdir -p "$INSTALL_DIR/bin"
    cp "$TARGET_DIR/$BIN_NAME" "$INSTALL_DIR/bin/oxide-ce"
    ok "Installed CLI to $INSTALL_DIR/bin/oxide-ce"
    if [[ "$BUILD_GUI" == true ]]; then
        GUI_TARGET_DIR="$SRC_DIR/gui/target/$TARGET/release"
        [[ -f "$GUI_TARGET_DIR/$GUI_BIN" ]] || GUI_TARGET_DIR="$SRC_DIR/gui/target/release"
        if [[ -f "$GUI_TARGET_DIR/$GUI_BIN" ]]; then
            cp "$GUI_TARGET_DIR/$GUI_BIN" "$INSTALL_DIR/bin/oxide-gui"
            ok "Installed GUI to $INSTALL_DIR/bin/oxide-gui"
        fi
    fi
elif [[ "$INSTALL" == true ]]; then
    warn "--install requires --release; skipping"
fi

echo ""
ok "$PROJECT build complete"
