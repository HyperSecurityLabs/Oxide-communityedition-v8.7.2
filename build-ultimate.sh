#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════
#  OXIDE v8.5.0 — Ultimate Build Script
#  Installs Rust + deps, compiles main engine, proxy, hypersecurity & GUI
# ═══════════════════════════════════════════════════════════════════════════
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'; YELLOW='\033[1;33m'
JADE='\033[38;2;0;200;150m'; BOLD='\033[1m'; NC='\033[0m'

banner() {
    echo -e "${JADE}"
    echo '   ____       _     __  '
    echo '  / __ \_  __(_)___/ /__'
    echo ' / / / / |/_/ / __  / _ \'
    echo '/ /_/ />  </ / /_/ /  __/'
    echo '\____/_/|_/_/\__,_/\___/ '
    echo -e "${NC}"
    echo -e "${CYAN}${BOLD}OXIDE v8.6.9 — Ultimate Build${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# ── Prerequisite Verification (runs before any phase) ──
verify_prereqs() {
    echo -e "\n${YELLOW}${BOLD}[Prerequisite Verification]${NC}"
    local all_ok=true

    # Rust toolchain
    if command -v rustc &>/dev/null; then
        ok "rustc: $(rustc --version | cut -d' ' -f2)"
    else
        fail "rustc not found — install via 'curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh'"
        all_ok=false
    fi
    if command -v cargo &>/dev/null; then
        ok "cargo: $(cargo --version | cut -d' ' -f2)"
    else
        fail "cargo not found"
        all_ok=false
    fi
    if command -v rustup &>/dev/null; then
        ok "rustup: $(rustup --version | head -1 | cut -d' ' -f2)"
    else
        warn "rustup not found — updates disabled"
    fi

    # Build essentials
    for tool in cmake make pkg-config; do
        if command -v "$tool" &>/dev/null; then
            ok "$tool: $($tool --version 2>&1 | head -1 | grep -oP '[\d.]+' | head -1)"
        else
            fail "$tool not found"
            all_ok=false
        fi
    done

    # Python (for build_db.py)
    if command -v python3 &>/dev/null; then
        ok "python3: $(python3 --version | cut -d' ' -f2)"
    else
        warn "python3 not found — database build will be skipped"
    fi

    # dpkg system deps (Debian/Ubuntu)
    if command -v dpkg &>/dev/null; then
        local missing=""
        for p in build-essential pkg-config libssl-dev cmake libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev; do
            dpkg -s "$p" &>/dev/null || missing="$missing $p"
        done
        if [ -n "$missing" ]; then
            warn "Missing system packages:$missing"
            info "Will be installed in Phase 1"
        else
            ok "All system packages present"
        fi
    else
        warn "dpkg not available — skipping system package check"
    fi

    # Rust targets
    if command -v rustup &>/dev/null; then
        local targets=$(rustup target list --installed 2>/dev/null)
        if echo "$targets" | grep -q "x86_64-unknown-linux-gnu"; then
            ok "Target: x86_64-unknown-linux-gnu"
        else
            warn "Target x86_64-unknown-linux-gnu not installed"
        fi
        if echo "$targets" | grep -q "x86_64-pc-windows-gnu"; then
            ok "Target: x86_64-pc-windows-gnu"
        fi
    fi

    if ! $all_ok; then
        fail "Prerequisite check failed — fix above errors and re-run"
    fi
    echo ""
}

info()  { echo -e "  ${CYAN}▸${NC} $1"; }
ok()    { echo -e "  ${GREEN}✔${NC} $1"; }
warn()  { echo -e "  ${YELLOW}⚠${NC} $1"; }
fail()  { echo -e "  ${RED}✘${NC} $1"; exit 1; }

trap 'echo -e "\n  ${RED}✘ Build interrupted${NC}"; exit 1' INT TERM

# ── Phase 1: System dependencies ──
phase1() {
    echo -e "\n${YELLOW}${BOLD}[1/5] System Dependencies${NC}"
    local pkgs=""
    for p in build-essential pkg-config libssl-dev cmake libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev; do
        dpkg -s "$p" &>/dev/null || pkgs="$pkgs $p"
    done
    if [ -n "$pkgs" ]; then
        info "Installing:$pkgs"
        sudo apt update && sudo apt install -y $pkgs
        ok "System deps installed"
    else
        ok "All system deps present"
    fi
}

# ── Phase 2: Rust (only installs if missing) ──
phase2() {
    echo -e "\n${YELLOW}${BOLD}[2/5] Rust Toolchain${NC}"
    if ! command -v rustc &>/dev/null; then
        info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
        source "$HOME/.cargo/env"
        ok "Rust installed"
    else
        local ver=$(rustc --version | cut -d' ' -f2)
        ok "Rust $ver already installed (skipping update)"
    fi
}

# ── Phase 3: Workspace build (main + oxide-proxy + hypersecurity) ──
phase3() {
    echo -e "\n${YELLOW}${BOLD}[3/5] Workspace — Main Engine + Proxy + Hypersecurity${NC}"
    info "Building with release profile (LTO, stripped)..."
    cd "$SCRIPT_DIR"
    cargo build --release -j"$(nproc)" 2>&1 | while IFS= read -r line; do
        echo -e "  ${CYAN}${NC}$line"
    done
    if [ -f "target/release/oxide" ]; then
        cp target/release/oxide oxide 2>/dev/null || true
        ok "Main binary: target/release/oxide ($(du -h target/release/oxide | cut -f1))"
    else
        fail "Main binary not found"
    fi
}

# ── Phase 4: GUI ──
phase4() {
    echo -e "\n${YELLOW}${BOLD}[4/5] GUI — Desktop Application${NC}"
    if [ ! -d "$SCRIPT_DIR/gui" ]; then
        warn "gui/ directory not found, skipping"
        return
    fi
    cd "$SCRIPT_DIR/gui"
    cargo build --release -j"$(nproc)" 2>&1 | while IFS= read -r line; do
        echo -e "  ${CYAN}${NC}$line"
    done
    if [ -f "target/release/oxide-gui" ]; then
        ok "GUI binary: gui/target/release/oxide-gui ($(du -h target/release/oxide-gui | cut -f1))"
    else
        warn "GUI binary not found (gui may have build errors)"
    fi
}

# ── Phase 5: Verify ──
phase5() {
    echo -e "\n${YELLOW}${BOLD}[5/5] Verification${NC}"
    cd "$SCRIPT_DIR"
    local ok=true
    for bin in oxide; do
        if [ -f "$bin" ]; then
            ok "$bin — ready"
        else
            warn "$bin — missing"
            ok=false
        fi
    done
    if [ -f "gui/target/release/oxide-gui" ]; then
        ok "oxide-gui — ready"
    fi
    echo ""
    if $ok; then
        echo -e "  ${GREEN}${BOLD}✔ OXIDE built successfully${NC}"
        echo -e "  ${CYAN}   ./oxide${NC}"
        echo -e "  ${CYAN}   gui/target/release/oxide-gui${NC}"
    else
        echo -e "  ${YELLOW}${BOLD}⚠ Build completed with warnings${NC}"
    fi
}

# ── Main ──
clear
banner
verify_prereqs
phase1
phase2
phase3
phase4
phase5
echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${JADE}${BOLD}  OXIDE v8.5.0 Ready — Forge the hunt.${NC}"
echo ""
