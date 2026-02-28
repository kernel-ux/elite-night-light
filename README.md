# 🌙 Elite Night Light
### A community-driven Native Night Mode for COSMIC

![Elite Night Mode Demo](res/demo.gif)

Hey everyone! 👋 

This is a personal project I built to solve the "blue light headache" on the new COSMIC desktop. This project provides a **Native Night Light** experience by patching the compositor and providing a beautiful Rust-based applet for the panel.

### ⚖️ GPL-3.0 Compliance & Transparency
As required by the GPL-3.0 license, this repository contains the **full source code** for both the Applet and the modified COSMIC Compositor (`cosmic-comp`). I’ve made these changes transparent so the community can audit, learn, and improve the code.

**Key Changes:**
- Patched `cosmic-comp` to include a D-Bus interface for hardware-level color temperature control.
- Updated the RDNN to `io.github.kernel_ux` to respect official namespaces.
- Developed a native Rust applet using `libcosmic` for easy GUI control.

### 🌟 Why this project?
- **100% Native:** Built into the engine. No flickering or TTY-switching hacks.
- **Synced CLI:** Control everything from the terminal; the UI updates instantly.
- **Smart Schedule:** 7 PM - 7 AM automatic warming with manual override respect.

### 🚀 How to get it running

Open your terminal and run these 4 commands:

```bash
git clone https://github.com/kernel-ux/elite-night-light
cd elite-night-light
chmod +x install.sh
sudo ./install.sh
```

**Note:** Since this replaces the system compositor, you must **Log out and Log back in** for the changes to take effect. The installer also "locks" the `cosmic-comp` package so official system updates don't overwrite your Elite patch.

### 🔄 How to uninstall / Revert to Official
If you ever want to go back to the stock System76 compositor, just run:
```bash
sudo apt-mark unhold cosmic-comp
sudo apt install --reinstall cosmic-comp
```

### 🖥️ Terminal Commands
- `toggle-night-mode` : Toggle ON/OFF.
- `toggle-night-mode off` : Turn OFF explicitly.
- `toggle-night-mode 1` : Soft Mode.
- `toggle-night-mode 2` : Warm Mode.
- `toggle-night-mode 3` : Strong Mode.

---
*Developed by Jeevan (kernel-ux). Not an official System76 project.*
