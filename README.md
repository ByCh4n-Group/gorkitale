# VibeCoded Linux Simulation

A retro-style Linux operating system simulation and game built with Rust and Tetra.

<img width="800" height="632" alt="image" src="https://github.com/user-attachments/assets/c0ac4d5b-6e44-4cf3-83a3-ed259c8ec5e9" />
<img width="800" height="632" alt="image" src="https://github.com/user-attachments/assets/d57034f1-ad5b-4142-bc7b-d8620492570c" />
<img width="800" height="632" alt="image" src="https://github.com/user-attachments/assets/91c3a2c2-5af5-4ee0-ac03-952339a1b446" />
<img width="800" height="632" alt="image" src="https://github.com/user-attachments/assets/b699a9d1-22dc-4f56-a50e-ea50e4be47af" />

## Overview

Welcome to **VibeCoded Linux 1.0 LTS**. This project simulates the experience of booting up an old-school Linux machine, complete with systemd boot logs, a TTY login screen, and a graphical desktop environment (X11).

Hidden within the system is a story about a corrupted kernel and a mysterious entity named "Glitch". Can you purge the corruption and save the system?

## Features

*   **Realistic Boot Sequence:** Watch the system services start up with a nostalgic typing effect.
*   **Interactive Login:** Log in as `root` to access the system.
*   **Shell Menu:** Navigate through system options like starting the X Server or configuring the system.
*   **Desktop Environment:** A simulated GUI with icons, windows, and a taskbar.
*   **Story Mode:** Interact with NPCs via IRC, use the Terminal to find system secrets, and unlock hidden game modes.
*   **System_Def Minigame:** A top-down shooter where you defend the memory blocks from corruption.

## How to Play

### Prerequisites
*   Rust (latest stable version)
*   SDL2 development libraries (required by Tetra)

### Running the Game
```bash
git clone https://github.com/ByCh4n-Group/linux_vibecoded_game
cd linux_vibecoded_game
cargo run
```

### Controls

**System:**
*   `Enter`: Confirm / Select
*   `Esc`: Go back / Close window
*   `Up/Down Arrows`: Navigate menus

**Desktop:**
*   `Mouse Left Click`: Interact with icons and windows
*   `Keyboard`: Type in Terminal and Chat windows

**System_Def (Minigame):**
*   `W, A, S, D`: Move Player
*   `Mouse Left Click`: Shoot
*   `R`: Restart Game (if Game Over)

## Story Guide (Spoilers!)

1.  **Login:** Username: `root`, Password: (any)
2.  **Start X Server:** Select option 1 from the menu.
3.  **Check Mail:** Read the message about system corruption.
4.  **Contact Glitch:** Open the "Chat" icon to talk to the contact.
5.  **Prove Identity:** Use the "Terminal" to run `sys_check` and find the integrity hash.
6.  **Unlock Deep Scan:** Provide the hash to Glitch to upgrade your defense protocols.
7.  **Purge:** Launch "System_Def" and survive.

## License

This project is open source.

---
*Developed by ByCh4n-Group*
