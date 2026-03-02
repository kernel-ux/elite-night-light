# 🌙 Elite Night Light for COSMIC

A professional, hardware-level Night Light integration for the COSMIC desktop environment on Pop!_OS.

## 🚀 Core Features

### 🖥️ Hardware-Level Engine
- **GPU-Accelerated Tinting:** Seamless color transformation executed directly on the graphics card via custom OpenGL fragment shaders (`offscreen.frag`), ensuring zero CPU overhead.
- **Native White Cursor Support:** Utilizes layer-aware rendering to bypass the post-processing tint for the mouse cursor. The cursor remains pure white, sharp, and highly visible, solving a common Linux desktop issue.
- **Perfect Transparency:** Applies the color temperature adjustments *before* alpha channel re-multiplication, preventing dark edges and maintaining perfect system-wide transparency and shadows.

### 🧠 Intelligent Automation
- **Adaptive Manual Override:** The "Smart Schedule" doesn't just force a state. If you manually turn the light ON or OFF, it respects your choice and intelligently waits until the next scheduled phase change (7:00 AM or 7:00 PM) to re-engage automation.
- **Time Category Memory:** The applet remembers the current "phase" of the day. If you shut down your PC in the afternoon and turn it on at night, it automatically detects the phase change and applies the correct Night Light state during boot.
- **Instant Boot Application:** Settings are read and applied the millisecond the desktop loads, ensuring your screen is immediately comfortable without waiting for background timers to start.

### ⚙️ Professional Architecture
- **Lock-Free DBus Communication:** The `cosmic-comp` engine uses asynchronous, `LazyLock` Atomic variables to receive state changes. This means the 144Hz rendering thread never blocks or waits for the UI thread, preventing desktop freezes.
- **Native COSMIC Applet:** The UI is built using the official `libcosmic` 1.0 applet pattern, integrating perfectly into the panel layout alongside official system applets.
- **Zero-Warning Compilation:** Both the compositor engine and the GUI applet compile with zero warnings or errors, adhering to strict Rust coding standards.
- **One-Click Installer/Uninstaller:** Includes a robust shell script that handles compilation, binary placement, panel injection, and provides a safe, automatic restoration to the factory Pop!_OS state if needed.

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
