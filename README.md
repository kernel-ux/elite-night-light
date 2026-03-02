# 🌙 Elite Night Light for COSMIC

A professional, hardware-level Night Light integration for the COSMIC desktop environment on Pop!_OS.

## 🚀 Features
- **GPU-Accelerated Tinting:** Seamless orange tinting using custom fragment shaders.
- **Native Cursor Support:** The mouse cursor stays perfectly white and sharp, bypassing the tint.
- **Smart Scheduling:** Automatic activation from 7 PM to 7 AM with intelligent manual override.
- **Persistence:** Remembers your settings (intensity and mode) across reboots.
- **Lock-Free Engine:** Asynchronous DBus communication prevents desktop freezes.

## 🛠️ Components
1. **The Engine:** A patched version of `cosmic-comp` with custom rendering logic.
2. **The Applet:** A native COSMIC panel applet for easy control.
3. **CLI Tools:** `toggle-night-mode` for instant terminal control.

## ⚖️ License & Credits
This project includes code from the official [COSMIC Compositor](https://github.com/pop-os/cosmic-comp), which is licensed under the **GPL-3.0**. 

In accordance with the GPL-3.0 license:
- This project is also licensed under **GPL-3.0**.
- Modified source files include notices of changes.
- Original copyrights belong to System76 and the COSMIC contributors.

---
*Developed by Jeevan (kernel-ux). Not an official System76 project.*
