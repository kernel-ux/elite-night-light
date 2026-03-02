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

echo "[1/3] Installing GUI applet..."
# We use the actual compiled binary name from the release folder
cp "$BASE_DIR/src/applet/target/release/elite-night-light" "/usr/local/bin/$APP_ID"
chmod +x "/usr/local/bin/$APP_ID"

echo "[2/3] Installing scripts and desktop files..."
cp "$BASE_DIR/bin/toggle-night-mode" /usr/local/bin/
chmod +x /usr/local/bin/toggle-night-mode

# Correct the desktop file Exec path
cat > "/usr/share/applications/$APP_ID.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=Elite Night Light
Comment=Native hardware-level Night Light for COSMIC
Exec=$APP_ID
Icon=weather-clear-night-symbolic
Terminal=false
Categories=System;Settings;
Keywords=Night;Light;Blue;Filter;
X-CosmicApplet=true
NoDisplay=true
EOF

echo "[3/3] Patching COSMIC compositor..."
COMP_SRC="$BASE_DIR/src/cosmic-comp/target/release/cosmic-comp"

if [ ! -f "/usr/bin/cosmic-comp.backup" ]; then
    echo "   -> Creating backup of original compositor..."
    cp /usr/bin/cosmic-comp /usr/bin/cosmic-comp.backup
fi

# Replace binary
mv /usr/bin/cosmic-comp /usr/bin/cosmic-comp.old 2>/dev/null || true
cp "$COMP_SRC" /usr/bin/cosmic-comp
chmod +x /usr/bin/cosmic-comp
rm -f /usr/bin/cosmic-comp.old

# Lock package
apt-mark hold cosmic-comp > /dev/null

echo "[4/4] Adding applet to your panel wings..."
PANEL_CONF="/home/jimmy/.config/cosmic/com.system76.CosmicPanel.Panel/v1/plugins_wings"
if [ -f "$PANEL_CONF" ]; then
    if ! grep -q "$APP_ID" "$PANEL_CONF"; then
        # Insert before A11y applet
        sed -i "s/\"com.system76.CosmicAppletA11y\"/\"$APP_ID\", \"com.system76.CosmicAppletA11y\"/" "$PANEL_CONF"
        echo "   -> Applet added to panel wings."
    else
        echo "   -> Applet already in panel configuration."
    fi
fi

echo "Finalizing configuration..."

echo "--------------------------------------------------"
echo "SUCCESS: Elite Night Light is now live!"
echo "NOTE: Please LOG OUT and LOG BACK IN to apply everything."
echo "The moon icon will appear in your panel next to Accessibility."
echo "--------------------------------------------------"
