#!/bin/bash

# Elite Night Light - Professional Installer
# Author: Jeevan
# License: GPL-3.0

set -e

# Get the full path of the directory where the script is located
BASE_DIR=$(cd "$(dirname "$0")" && pwd)
APP_ID="io.github.kernel_ux.EliteNightLight"

echo "--------------------------------------------------"
echo "ELITE NIGHT LIGHT: INSTALLER"
echo "--------------------------------------------------"

if [ "$EUID" -ne 0 ]; then
  echo "Error: This script must be run with sudo."
  echo "Please run: sudo ./install.sh"
  exit 1
fi

# 1. Install Applet and Scripts
echo "[1/4] Installing applet and scripts..."
chmod +x "$BASE_DIR/bin/"*
killall cosmic-applet-night-light || true
cp "$BASE_DIR/bin/cosmic-applet-night-light" /usr/local/bin/
cp "$BASE_DIR/bin/toggle-night-mode" /usr/local/bin/
chmod +x /usr/local/bin/toggle-night-mode

# 2. Patch Compositor
echo "[2/4] Patching COSMIC compositor..."
if cp "$BASE_DIR/bin/cosmic-comp" /usr/bin/cosmic-comp 2>/dev/null; then
    echo "   -> Compositor updated successfully."
else
    echo "   -> Compositor busy, attempting backup and replace..."
    mv /usr/bin/cosmic-comp /usr/bin/cosmic-comp.old
    cp "$BASE_DIR/bin/cosmic-comp" /usr/bin/cosmic-comp
fi

# 3. Install Desktop Entry
echo "[3/4] Registering applet with COSMIC..."
cp "$BASE_DIR/res/$APP_ID.desktop" /usr/share/applications/

# 4. Update Panel Config
echo "[4/4] Adding applet to your panel wings..."
ACTUAL_USER=$(logname || echo $SUDO_USER)
USER_HOME=$(eval echo ~$ACTUAL_USER)
PANEL_CONFIG="$USER_HOME/.config/cosmic/com.system76.CosmicPanel.Panel/v1/plugins_wings"

if [ -f "$PANEL_CONFIG" ]; then
    if ! grep -q "$APP_ID" "$PANEL_CONFIG"; then
        sed -i '/"com.system76.CosmicAppletInputSources",/a \    "'"$APP_ID"'",' "$PANEL_CONFIG"
        echo "   -> Applet added to panel configuration for $ACTUAL_USER."
    else
        echo "   -> Applet already in panel configuration."
    fi
fi

echo "Finalizing installation..."
killall -u $ACTUAL_USER cosmic-panel || true

echo "--------------------------------------------------"
echo "SUCCESS: Elite Night Light is now live!"
echo "NOTE: If you don't see the color change, please logout and back in."
echo "--------------------------------------------------"
