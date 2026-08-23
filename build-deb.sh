#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

DEB_DIR="oxide-ce-debian"
DEB_OUT="oxide-ce_8.7.2community-edition_amd64.deb"

echo "[*] Checking prerequisites..."
for cmd in dpkg-deb install; do
    if ! command -v "$cmd" &>/dev/null; then
        echo "[-] Missing: $cmd"
        exit 1
    fi
done 2>/dev/null

echo "[*] Removing old .deb..."
rm -f "$DEB_OUT"

echo "[*] Copying binary..."
if [ ! -f "oxide" ]; then
    echo "[-] oxide binary not found — run 'cargo build --release' first"
    exit 1
fi
cp oxide "$DEB_DIR/usr/bin/oxide-ce-bin"
chmod 755 "$DEB_DIR/usr/bin/oxide-ce" "$DEB_DIR/usr/bin/oxide-ce-bin"

echo "[*] Building .deb..."
dpkg-deb --build "$DEB_DIR"

echo "[*] Renaming..."
mv "${DEB_DIR}.deb" "$DEB_OUT"

echo "[+] Done: $DEB_OUT ($(du -h "$DEB_OUT" | cut -f1))"
