# 🌙 Elite Night Light for COSMIC

A professional, hardware-level Night Light integration for the COSMIC desktop environment on Pop!_OS.

## 🚀 Features
- **GPU-Accelerated Tinting:** Seamless orange tinting using custom fragment shaders.
- **Native Cursor Support:** The mouse cursor stays perfectly white and sharp, bypassing the tint.
- **Smart Scheduling:** Automatic activation from 7 PM to 7 AM with intelligent manual override.
- **Persistence:** Remembers your settings (intensity and mode) across reboots.
- **Lock-Free Engine:** Asynchronous DBus communication prevents desktop freezes.

## 🛠️ Installation

This project modifies the core system compositor. It is high-performance and stable, but it must be built from source to ensure compatibility with your hardware.

```bash
# 1. Clone the repository
git clone https://github.com/kernel-ux/elite-night-light.git
cd elite-night-light

# 2. Run the installer (this will build the project and may take a few minutes)
chmod +x install.sh
./install.sh

# 3. Apply changes
# Log out and Log back in to load the new engine.
```

## 🗑️ Uninstallation

If you want to return to the official Pop!_OS compositor and remove all Elite components:

```bash
# Run the uninstaller
chmod +x uninstall.sh
./uninstall.sh

# Log out and Log back in to restore the factory compositor.
```

## ⚖️ License
Licensed under the **GPL-3.0**. Built using code from the official System76 COSMIC project.

---
*Developed by Jeevan (kernel-ux). Not an official System76 project.*
