# Gorkitale

[![CI](https://github.com/ByCh4n-Group/gorkitale/actions/workflows/ci.yml/badge.svg)](https://github.com/ByCh4n-Group/gorkitale/actions/workflows/ci.yml)

A retro-style takfir or teblig game.

TODO:
- multi lang support for memes and the entire game for en and tr.
- add put mechanism(idol statue).
- tebliğ can make the characters Muslim(need muslim version of characters).
- add kurban(adha) mechanism, (toriel; "anne keçi kurban edilecek").
- add pray mechanism.
- (idk but it's required)
- the lore.

|   |   |
|---|---|
| ![](https://github.com/user-attachments/assets/b035f7df-9ae8-4811-bd26-8ff1b602f8fd) | ![](https://github.com/user-attachments/assets/400f9206-e000-4c9c-b972-8c961cdea390) |
| ![](https://github.com/user-attachments/assets/55b85248-3123-485b-890d-c2975b00cc15) | ![](https://github.com/user-attachments/assets/362dc794-bf10-4634-9197-f4b259b213cc) |


## Overview

Welcome to **Gorkitale 1.0 LTS**..

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
*   SDL3 development libraries (required by Tetra)

### Running the Game
```bash
git clone https://github.com/ByCh4n-Group/takfirtale
cd takfirtale
cargo run
```

### Controls

**Adventure Mode:**
*   `WASD` or `Arrow Keys`: Move character
*   `Enter`: Interact

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
*Developed by ByCh4n-Group*
