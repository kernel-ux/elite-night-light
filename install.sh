#!/bin/bash

# Elite Night Mode for COSMIC - One-Click Installer
# Developed by: Jeevan
# Date: February 27, 2026

set -e

echo "--------------------------------------------------"
echo "ELITE NIGHT MODE: AUTOMATIC INSTALLER"
echo "--------------------------------------------------"

# 1. Ask for sudo permissions
if [ "$EUID" -ne 0 ]; then
  echo "This script needs to move files to system folders."
  echo "Please run it with pkexec or sudo if needed."
  # We will use pkexec internally for key steps.
fi

# 2. Install Binaries
echo "[1/4] Installing binaries..."
chmod +x ./bin/*
pkexec cp ./bin/cosmic-applet-night-light /usr/local/bin/
pkexec cp ./bin/cosmic-comp /usr/bin/cosmic-comp
pkexec cp ./bin/toggle-night-mode /usr/local/bin/
pkexec chmod +x /usr/local/bin/toggle-night-mode

# 3. Install Desktop Entry
echo "[2/4] Registering applet with COSMIC..."
pkexec cp ./res/com.system76.CosmicAppletNightLight.desktop /usr/share/applications/

# 4. Update Panel Config (if user is in session)
echo "[3/4] Adding applet to your panel wings..."
PANEL_CONFIG="$HOME/.config/cosmic/com.system76.CosmicPanel.Panel/v1/plugins_wings"

if [ -f "$PANEL_CONFIG" ]; then
    # Check if already exists
    if ! grep -q "com.system76.CosmicAppletNightLight" "$PANEL_CONFIG"; then
        # Better sed command to insert after InputSources
        sed -i '/"com.system76.CosmicAppletInputSources",/a \    "com.system76.CosmicAppletNightLight",' "$PANEL_CONFIG"
        echo "   -> Applet added to panel configuration."
    else
        echo "   -> Applet already in panel configuration."
    fi
else
    echo "   -> WARNING: Panel config not found at $PANEL_CONFIG."
    echo "      You may need to add it manually in COSMIC settings."
fi

# 5. Restart Services
echo "[4/4] Restarting COSMIC Panel..."
killall cosmic-panel || true
killall cosmic-applet-night-light || true

echo "--------------------------------------------------"
echo "SUCCESS: Elite Night Mode is now live!"
echo "Check your panel—it should be right next to the 'US' icon."
echo "--------------------------------------------------"
