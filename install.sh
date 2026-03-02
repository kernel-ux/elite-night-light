#!/bin/bash

# Elite Night Light - Professional Installer
# Author: Jeevan
# License: GPL-3.0

set -e

BASE_DIR=$(cd "$(dirname "$0")" && pwd)
APP_ID="io.github.kernel_ux.EliteNightLight"

echo "--------------------------------------------------"
echo "ELITE NIGHT LIGHT: INSTALLER"
echo "--------------------------------------------------"

# 1. Dependency Check
echo "[1/5] Checking for build dependencies..."
REQUIRED_PKGS="rustc cargo libwayland-dev libxkbcommon-dev libgbm-dev libudev-dev libseat-dev pkg-config cmake git libinput-dev libdbus-1-dev libdisplay-info-dev libpipewire-0.3-dev"

MISSING_PKGS=""
for pkg in $REQUIRED_PKGS; do
    if ! dpkg -l | grep -q "^ii  $pkg " && ! dpkg -l | grep -q "^ii  $pkg:" ; then
        MISSING_PKGS="$MISSING_PKGS $pkg"
    fi
done

if [ -n "$MISSING_PKGS" ]; then
    echo "   -> The following packages are required but missing:$MISSING_PKGS"
    echo "   -> Installing missing dependencies (requires sudo)..."
    pkexec apt-get update
    pkexec apt-get install -y $MISSING_PKGS
else
    echo "   -> All build dependencies are satisfied."
fi

# 2. Build Components
echo "[2/5] Building Components (this may take a few minutes)..."
cd "$BASE_DIR/src/cosmic-comp"
cargo build --release

cd "$BASE_DIR/src/applet"
cargo build --release

# 3. Install Binaries
echo "[3/5] Installing binaries (requires sudo)..."
pkexec killall "$APP_ID" || true
pkexec cp "$BASE_DIR/src/applet/target/release/elite-night-light" "/usr/local/bin/$APP_ID"
pkexec chmod +x "/usr/local/bin/$APP_ID"

pkexec cp "$BASE_DIR/bin/toggle-night-mode" /usr/local/bin/
pkexec chmod +x /usr/local/bin/toggle-night-mode

# 4. Patch Compositor
echo "[4/5] Patching COSMIC compositor..."
COMP_SRC="$BASE_DIR/src/cosmic-comp/target/release/cosmic-comp"

if [ ! -f "/usr/bin/cosmic-comp.backup" ]; then
    echo "   -> Creating backup of original compositor..."
    pkexec cp /usr/bin/cosmic-comp /usr/bin/cosmic-comp.backup
fi

pkexec mv /usr/bin/cosmic-comp /usr/bin/cosmic-comp.old 2>/dev/null || true
pkexec cp "$COMP_SRC" /usr/bin/cosmic-comp
pkexec chmod +x /usr/bin/cosmic-comp
pkexec rm -f /usr/bin/cosmic-comp.old

pkexec apt-mark hold cosmic-comp > /dev/null

# 5. Panel & Desktop Entry
echo "[5/5] Finalizing configuration..."
pkexec cp "$BASE_DIR/res/$APP_ID.desktop" /usr/share/applications/

# Panel inject logic
PANEL_CONF="$HOME/.config/cosmic/com.system76.CosmicPanel.Panel/v1/plugins_wings"
if [ -f "$PANEL_CONF" ]; then
    if ! grep -q "\"$APP_ID\"" "$PANEL_CONF"; then
        sed -i "s/\"com.system76.CosmicAppletA11y\"/\"$APP_ID\",\n    \"com.system76.CosmicAppletA11y\"/" "$PANEL_CONF"
        echo "   -> Applet added to panel wings."
    fi
fi

echo "--------------------------------------------------"
echo "SUCCESS: Elite Night Light is now live!"
echo "NOTE: Please LOG OUT and LOG BACK IN to apply everything."
echo "--------------------------------------------------"
