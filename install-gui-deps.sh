#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────
#  install-gui-deps.sh — install system libraries for OXIDE GUI on Kali
#
#  Installs: WebKit2GTK-4.1, GTK3, cmake, libsoup3, appindicator
#  Safe to re-run (idempotent).
#
#  USAGE:  sudo ./install-gui-deps.sh
# ─────────────────────────────────────────────────────────────────────────
set -euo pipefail

echo "[*] Updating package lists ..."
apt-get update -qq

echo "[*] Installing OXIDE GUI system dependencies ..."
apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    cmake \
    libsoup-3.0-dev \
    libjavascriptcoregtk-4.1-dev \
    libayatana-appindicator3-dev \
    pkg-config \
    libssl-dev \
    glib-2.0-dev \
    pango1.0-dev \
    libgdk-pixbuf2.0-dev \
    libatk1.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    libglib2.0-dev

echo "[✓] OXIDE GUI dependencies installed."
