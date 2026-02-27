# 🌙 COSMIC Elite Night Mode
### Built with 🧡 by a student, for the community.

> **⚠️ Note on the Demo Video:** You won't see the screen change color in this video! This is because Wayland screen recorders capture the frame *before* the compositor applies our custom "Elite" Night Mode tint. On your actual monitor, the screen will turn a beautiful warm orange! 🧡

<video src="res/demo.mp4" width="100%" controls autoplay loop muted></video>

Hey everyone! 👋 

So, I've been diving into the new **Pop!_OS 24.04 LTS** with the **COSMIC** desktop environment. It's super fast and looks amazing, but since it's still in its early stages (pre-alpha), I noticed one big thing was missing that I really needed: a native **Night Mode**. 

My eyes were killing me during late-night coding sessions, so I decided to take matters into my own hands! 🚀

### 🎓 What is this?
This is a personal project I built to solve the "blue light headache." Since this specific version of COSMIC doesn't have Night Light built-in yet, I patched the compositor engine and wrote a native Rust applet to make it happen.

I'm just a **student**, not a professional developer, so please forgive me if you find any bugs! 😅 I'm learning as I go, but I wanted to share this because I didn't want anyone else to have the headache of trying to build this from scratch. It's all **open source**, so feel free to tweak the code and make it even better!

### 🌟 Why use this one?
I searched around and tried different things, but I wanted something that felt "Elite." Here’s why this version is cool:
- **It's Native:** This isn't just a slow script running in the background. It's built directly into the COSMIC engine (`cosmic-comp`), so it's lightning fast and uses almost zero resources. ⚡
- **It's Smarter:** It has a built-in schedule! It automatically warms up your screen at **7 PM** and clears it up at **7 AM**. ⏰
- **User First:** Even with "Auto" mode on, it won't fight you. If you manually turn it off or change the intensity, it stays that way until the next day/night cycle.
- **Easy Mode:** I've already compiled the code for you. You don't need to spend hours installing Rust or waiting for builds—just run the installer and you're good to go!

### 🚀 How to get it running
1. Download or clone this folder.
2. Open your terminal in this folder.
3. Run this command:
   ```bash
   chmod +x install.sh && ./install.sh
   ```
4. Boom! The icon should appear in your panel (I put it right next to the language/US icon).

### 🖥️ Terminal Commands
If you're a terminal fan like me, you can also control everything with these commands:
- `toggle-night-mode` : Just flips it ON or OFF.
- `toggle-night-mode 1` : Set to **Soft** mode.
- `toggle-night-mode 2` : Set to **Warm** mode.
- `toggle-night-mode 3` : Set to **Strong** mode.

### 🛠 Tech Stuff (If you're curious)
- **OS:** Pop!_OS 24.04 LTS (Noble)
- **Desktop:** COSMIC (Wayland)
- **Language:** 100% Rust 🦀

I really hope this makes your COSMIC experience a little bit better. If you like it, let me know! If it breaks... well, remember I'm just a student! hahahah. Enjoy! 🧡🌙

---
*Created by Jeevan (jimmy)*
