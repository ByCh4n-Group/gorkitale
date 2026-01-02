# Gorkitale

[![CI](https://github.com/YOUR_USERNAME/YOUR_REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/YOUR_USERNAME/YOUR_REPO/actions/workflows/ci.yml)

A retro-style Linux operating system simulation and game built with Rust and Tetra.

TODO:
- multi lang support for memes and the entire game for en and tr.
- add put mechanism(idol statue).
- tebliğ can make the characters Muslim(need muslim version of characters).
- add kurban(adha) mechanism, (toriel; "anne keçi kurban edilecek").
- add pray mechanism.
- (idk but it's required)
- the lore.

| | | |
|:---:|:---:|:---:|
| <img width="100%" alt="image" src="https://github.com/user-attachments/assets/e5f8997f-7297-4ace-84d7-21c4eee3f7ef" /> | <img width="100%" alt="image" src="https://github.com/user-attachments/assets/b272ffd5-6580-478e-885d-b77ba425c696" /> | <img width="100%" alt="image" src="https://github.com/user-attachments/assets/d5d825dd-4db6-4f59-9aa2-29b7f6100f45" /> |
| <img width="100%" alt="image" src="https://github.com/user-attachments/assets/8774a62a-4fd1-4cde-b05e-42cdb7f801bc" /> | <img width="100%" alt="image" src="https://github.com/user-attachments/assets/9d07eab0-d351-4aa4-99bb-d2eac5e0701d" /> | <img width="100%" alt="image" src="https://github.com/user-attachments/assets/acaa32e1-abc6-4cd8-a72d-fb8ac997feb7" /> |
| <img width="100%" alt="image" src="https://github.com/user-attachments/assets/c199c901-a246-4b2e-a996-281e2107e0fd" /> | <img width="100%" alt="image" src="https://github.com/user-attachments/assets/28590cd6-f869-437f-9d3d-8eade34afe78" /> | |

## Overview

Welcome to **Gorkitale 1.0 LTS**. This project simulates the experience of booting up an old-school Linux machine, complete with systemd boot logs and a TTY login screen.

Beyond the terminal lies a surreal adventure. Log in, explore the system, and launch into a world where you must interact with mysterious characters and uncover the secrets of the "VibeCoded" kernel.

## Features

*   **Realistic Boot Sequence:** Watch the system services start up with a nostalgic typing effect.
*   **Interactive Login:** Log in as `root` or create your own user to access the system.
*   **Functional Shell:** A working terminal environment where you can run commands like `neofetch`, `users`, and `startx`.
*   **Adventure Mode:** Launch the graphical game world to explore different stages and interact with NPCs.
*   **Story Elements:** Engage with characters and uncover a narrative involving "Teblig", "Cihad", and "Tekfir".
*   **Combat & Defense:** Defend yourself against threats in specific game stages.

## Shell Commands

The in-game terminal supports a variety of commands to interact with the system:

*   `startx`: Starts the graphical adventure mode (Game).
*   `adduser <username>`: Creates a new user profile.
*   `users`: Lists all registered users on the system.
*   `neofetch`: Displays system information and the VibeCoded logo.
*   `music`: Toggles the background music (Disco Mode).
*   `config`: Opens the system configuration menu.
*   `logout`: Logs out of the current session.
*   `reboot`: Reboots the simulated system.
*   `shutdown`: Powers off the system.
*   `clear`: Clears the terminal screen.
*   `whoami`: Displays the current user's name.
*   `uname -a`: Prints system kernel information.

## Combat & Dialogues

In combat encounters (like against Sans), you have unique interaction options beyond just attacking ("Cihad").

### Tekfir (Act)
You can declare the opponent as various types of non-believers. Here are some of their reactions:

*   **Müşrik:** "Ona Müşrik dedin. Sana güldü." / "Ona Müşrik dedin. 'Sen de kimsin?' dedi."
*   **Fasık:** "Ona Fasık dedin. Umursamadı." / "Ona Fasık dedin. Esneyerek cevap verdi."
*   **Münafık:** "Ona Münafık dedin. Omuz silkti." / "Ona Münafık dedin. 'Kanıtın var mı?' dedi."
*   **Kafir:** "Ona Kafir dedin. Sırıttı." / "Ona Kafir dedin. 'Bunu iltifat sayarım' dedi."
*   **Zındık:** "Ona Zındık dedin. Kahkaha attı." / "Ona Zındık dedin. 'Eski moda bir hakaret' dedi."
*   **Tağut:** "Ona Tağut dedin. Göz kırptı." / "Ona Tağut dedin. 'Gücümü kabul ediyorsun' dedi."
*   **Deccal:** "Ona Deccal dedin. 'Tek gözüm bile yeter' dedi." / "Ona Deccal dedin. Alnını gösterdi."
*   **Ebu Cehil:** "Ona Ebu Cehil dedin. 'Cehalet mutluluktur' dedi." / "Ona Ebu Cehil dedin. Karpuz fırlattı."
*   **Yecüc:** "Ona Yecüc dedin. 'Mecüc nerede?' diye sordu." / "Ona Yecüc dedin. Duvarı kemirmeye başladı."

### Tebliğ (Mercy)
You can try to preach and guide them to the right path. Results may vary:

*   "Ona İslam'ı anlattın. Sana güldü."
*   "Tövbe etmesini söyledin. Umursamadı."
*   "Cehennem ateşinden bahsettin. Omuz silkti."
*   "Ona hidayet diledin. Hala sırıtıyor."
*   "Ona Kuran okudun. Rahatsız oldu."
*   "Ona hadis anlattın. Kulaklarını tıkadı."
*   "Ona ölümü hatırlattın. Ürperdi ama belli etmedi."
*   "Ona cenneti anlattın. 'İlgilenmiyorum' dedi."
*   "Ona selam verdin. Almadı."
*   "Ona dua ettin. Gözlerini devirdi."
*   "Ona zemzem ikram ettin. 'Kola yok mu?' dedi."
*   "Ona misvak uzattın. 'Diş fırçam var' dedi."
*   "Ona takke takmaya çalıştın. Kafasını çekti."
*   "Ona tesbih hediye ettin. Boncuk sandı."

## How to Play

### Prerequisites
*   Rust (latest stable version)
*   SDL2 development libraries (required by Tetra)

### Running the Game
```bash
git clone https://github.com/ByCh4n-Group/takfirtale
cd takfirtale
cargo run
```

### Controls

**System (Boot/Shell):**
*   `Enter`: Confirm / Execute command
*   `Esc`: Go back
*   `Up/Down Arrows`: Navigate menus

**Adventure Mode:**
*   `WASD` or `Arrow Keys`: Move character
*   `Space`: Interact

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
*Developed by ByCh4n-Group*
