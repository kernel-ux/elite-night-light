#!/bin/bash

# Elite Night Light - Uninstaller
# Restores the system to factory state

set -e

APP_ID="io.github.kernel_ux.EliteNightLight"

echo "--------------------------------------------------"
echo "ELITE NIGHT LIGHT: UNINSTALLER"
echo "--------------------------------------------------"

echo "[1/3] Removing applet and scripts..."
pkexec killall "$APP_ID" || true
pkexec rm -f "/usr/local/bin/$APP_ID"
pkexec rm -f "/usr/local/bin/toggle-night-mode"
pkexec rm -f "/usr/share/applications/$APP_ID.desktop"
rm -f "$HOME/.config/elite-night-light.json"

echo "[2/3] Restoring original COSMIC compositor..."
pkexec apt-mark unhold cosmic-comp
pkexec apt-get install --reinstall cosmic-comp -y

echo "[3/3] Cleaning up panel configuration..."
PANEL_CONF="$HOME/.config/cosmic/com.system76.CosmicPanel.Panel/v1/plugins_wings"
if [ -f "$PANEL_CONF" ]; then
    sed -i "s/"$APP_ID",//" "$PANEL_CONF"
    sed -i "s/,"$APP_ID"//" "$PANEL_CONF"
    echo "   -> Applet removed from panel configuration."
fi

echo "--------------------------------------------------"
echo "SUCCESS: Elite Night Light has been removed."
echo "NOTE: Please LOG OUT and LOG BACK IN to complete restoration."
echo "--------------------------------------------------"
