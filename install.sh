#!/bin/bash

# Elite Night Mode for COSMIC - One-Click Installer
# Developed by: Jeevan
# Date: February 27, 2026

set -e

# Get the full path of the directory where the script is located
BASE_DIR=$(cd "$(dirname "$0")" && pwd)

echo "--------------------------------------------------"
echo "ELITE NIGHT MODE: AUTOMATIC INSTALLER"
echo "--------------------------------------------------"

# 1. Ensure the script is run with sudo
if [ "$EUID" -ne 0 ]; then
  echo "Error: This script must be run with sudo."
  echo "Please run: sudo ./install.sh"
  exit 1
fi

# 2. Install Applet and Scripts
echo "[1/4] Installing applet and scripts..."
chmod +x "$BASE_DIR/bin/"*
cp "$BASE_DIR/bin/cosmic-applet-night-light" /usr/local/bin/
cp "$BASE_DIR/bin/toggle-night-mode" /usr/local/bin/
chmod +x /usr/local/bin/toggle-night-mode

# 3. Patch Compositor
echo "[2/4] Patching COSMIC compositor..."
if cp "$BASE_DIR/bin/cosmic-comp" /usr/bin/cosmic-comp 2>/dev/null; then
    echo "   -> Compositor updated successfully."
else
    echo "   -> Compositor busy, attempting backup and replace..."
    mv /usr/bin/cosmic-comp /usr/bin/cosmic-comp.old
    cp "$BASE_DIR/bin/cosmic-comp" /usr/bin/cosmic-comp
    echo "   -> Compositor replaced (restart required for tint to work)."
fi

# 4. Install Desktop Entry
echo "[3/4] Registering applet with COSMIC..."
cp "$BASE_DIR/res/com.system76.CosmicAppletNightLight.desktop" /usr/share/applications/

# 5. Update Panel Config (if user is in session)
echo "[4/4] Adding applet to your panel wings..."
# Since we are running as root, we need to find the actual user's home
ACTUAL_USER=$(logname || echo $SUDO_USER)
USER_HOME=$(eval echo ~$ACTUAL_USER)
PANEL_CONFIG="$USER_HOME/.config/cosmic/com.system76.CosmicPanel.Panel/v1/plugins_wings"

if [ -f "$PANEL_CONFIG" ]; then
    # Check if already exists
    if ! grep -q "com.system76.CosmicAppletNightLight" "$PANEL_CONFIG"; then
        # Better sed command to insert after InputSources
        sed -i '/"com.system76.CosmicAppletInputSources",/a \    "com.system76.CosmicAppletNightLight",' "$PANEL_CONFIG"
        echo "   -> Applet added to panel configuration for $ACTUAL_USER."
    else
        echo "   -> Applet already in panel configuration."
    fi
else
    echo "   -> WARNING: Panel config not found at $PANEL_CONFIG."
fi

# 6. Restart Services
echo "Finalizing installation..."
killall -u $ACTUAL_USER cosmic-panel || true
killall -u $ACTUAL_USER cosmic-applet-night-light || true

echo "--------------------------------------------------"
echo "SUCCESS: Elite Night Mode is now live!"
echo "Check your panel—it should be right next to the 'US' icon."
echo "NOTE: If you don't see the color change, please logout and back in."
echo "--------------------------------------------------"
