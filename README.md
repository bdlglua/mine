# MineOS - Operating System

```
       ___  ___  _                 ___  _____ 
       |  \/  | (_)               /   |/  ___|
       | .  . |  _  _ __    ___  / /| |\ `--. 
       | |\/| | | || '_ \  / _ \/ /_| | `--. \
       | |  | | | || | | ||  __/\___  |/\__/ /
       \_|  |_/ |_||_| |_| \___|    |_/\____/ 
```

## Un vrai OS bare-metal ecrit en Rust et Assembleur

MineOS est un systeme d'exploitation bare-metal avec interface graphique, ecrit en **Rust** et **Assembleur x86_64**. Il demarre directement sur le materiel (ou dans QEMU) sans aucun systeme d'exploitation hote.

---

## Architecture

### Assembleur (ASM x86_64 / NASM)
- **`asm/boot.asm`** - Bootloader Multiboot2 complet:
  - Header Multiboot2 avec requete framebuffer (1024x768x32bpp)
  - Verification CPUID et support Long Mode
  - Configuration GDT 64-bit (Global Descriptor Table)
  - Tables de pagination (identity mapping 1GB)
  - Transition: Real Mode → Protected Mode → Long Mode (64-bit)
  - Passage de controle au kernel Rust

- **`asm/interrupts.asm`** - Stubs d'interruptions:
  - Macros ISR avec/sans code d'erreur
  - Sauvegarde/restauration complete des registres
  - Exceptions CPU (0-19): Division par zero, Double Fault, Page Fault...
  - IRQs materiels (32-47): Timer, Clavier, COM, etc.
  - Fonctions I/O port (inb, outb, inw, outw) exposees au Rust via FFI

### Rust (no_std, no_main)
- **Kernel** (`src/main.rs`):
  - Point d'entree `kernel_main` via bootloader crate
  - Gestionnaire de panique avec affichage VGA
  - Allocateur de heap (alloc_error_handler)

- **Pilotes** (`src/drivers/`):
  - `vga_text.rs` - Pilote VGA mode texte (0xB8000)
  - `framebuffer.rs` - Pilote framebuffer graphique 32-bit:
    - Dessin de pixels, lignes, rectangles
    - Rectangles arrondis, cercles
    - Double buffering (anti-tearing)
    - Rendu de texte avec police bitmap 8x16
    - Palette "Obsidian Glass" complete
  - `keyboard.rs` - Pilote clavier PS/2:
    - Decodage scancodes Set 1
    - Buffer de touches circulaire
    - Support modificateurs (Shift, Ctrl, Alt)
    - Lecture bloquante/non-bloquante
  - `pic.rs` - Controleur d'interruptions 8259 PIC
  - `memory.rs` - Gestion memoire:
    - Allocateur de frames physiques
    - Pagination x86_64 (4 niveaux)
    - Heap kernel 4MB avec linked_list_allocator

- **Interface Graphique** (`src/gui/`):
  - `desktop.rs` - Environnement de bureau complet:
    - Fond d'ecran avec gradient et grille
    - Icones de bureau avec rendu pixel
    - Boucle principale de rendu
    - Gestion du clavier (raccourcis Ctrl+T, Alt+1-6)
  - `window.rs` - Gestionnaire de fenetres:
    - Creation/fermeture/focus de fenetres
    - Barre de titre avec boutons (fermer, minimiser, maximiser)
    - Z-ordering automatique
  - `taskbar.rs` - Barre des taches:
    - Bouton demarrer (hexagone)
    - Apps en cours d'execution
    - Zone systeme (WiFi, batterie, volume, horloge)
  - `font.rs` - Police bitmap 8x16 ASCII complete
  - `widgets.rs` - Composants UI (boutons, champs texte, barres de progression)

- **Applications** (`src/apps/`):
  - `terminal.rs` - Terminal complet avec commandes:
    - help, clear, echo, whoami, uname, ls, pwd, date
    - neofetch (affichage ASCII art), calc, reboot, shutdown
  - `calculator.rs` - Calculatrice avec operations +-*/

---

## Compilation et Execution

### Prerequis
```bash
# Installer Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly
rustup component add rust-src llvm-tools-preview

# Installer cargo-bootimage
cargo install bootimage

# Installer NASM
sudo apt install nasm

# Installer QEMU (pour tester)
sudo apt install qemu-system-x86
```

### Compiler
```bash
cd mineos-kernel
make build
```

### Executer dans QEMU
```bash
make run
```

### Creer une image ISO bootable
```bash
# Necessite grub-mkrescue
make iso
# L'image sera dans target/mineos.iso
```

---

## Raccourcis Clavier
| Raccourci | Action |
|-----------|--------|
| `Ctrl+T` | Ouvrir Terminal |
| `Ctrl+E` | Ouvrir Editeur |
| `Ctrl+W` | Fermer fenetre active |
| `Alt+1` | Terminal |
| `Alt+2` | Gestionnaire de fichiers |
| `Alt+3` | Editeur de texte |
| `Alt+4` | Calculatrice |
| `Alt+5` | Gestionnaire de taches |
| `Alt+6` | Parametres |

---

## Commandes Terminal
```
user@mineos:~$ help
user@mineos:~$ whoami
user@mineos:~$ uname
user@mineos:~$ ls
user@mineos:~$ neofetch
user@mineos:~$ calc 42+8
user@mineos:~$ echo Hello MineOS!
user@mineos:~$ clear
user@mineos:~$ reboot
user@mineos:~$ shutdown
```

---

## Structure du Projet
```
mineos-kernel/
├── Cargo.toml              # Configuration Rust
├── rust-toolchain.toml     # Toolchain nightly
├── Makefile                # Systeme de build
├── .cargo/
│   └── config.toml         # Target x86_64-unknown-none
├── asm/                    # Code ASSEMBLEUR
│   ├── boot.asm            # Bootloader Multiboot2 (GDT, paging, long mode)
│   └── interrupts.asm      # ISR stubs + I/O ports
└── src/                    # Code RUST
    ├── main.rs             # Point d'entree kernel
    ├── drivers/
    │   ├── mod.rs          # IDT, handlers d'interruptions
    │   ├── vga_text.rs     # VGA mode texte
    │   ├── framebuffer.rs  # Graphiques framebuffer 32-bit
    │   ├── keyboard.rs     # Pilote clavier PS/2
    │   ├── pic.rs          # PIC 8259
    │   └── memory.rs       # Gestion memoire + heap
    ├── gui/
    │   ├── mod.rs          # Module GUI
    │   ├── desktop.rs      # Bureau + boucle principale
    │   ├── window.rs       # Gestionnaire de fenetres
    │   ├── taskbar.rs      # Barre des taches
    │   ├── font.rs         # Police bitmap 8x16
    │   └── widgets.rs      # Widgets UI
    └── apps/
        ├── mod.rs          # Module applications
        ├── terminal.rs     # Terminal avec commandes
        └── calculator.rs   # Calculatrice
```

---

## Technologies
- **Langage principal**: Rust (nightly, `#![no_std]`)
- **Assembleur**: x86_64 NASM
- **Cibles**: x86_64 bare-metal (`x86_64-unknown-none`)
- **Bootloader**: bootloader crate 0.9 (Multiboot2)
- **Graphiques**: Framebuffer lineaire 32-bit BGRA
- **Interruptions**: PIC 8259 + IDT x86_64

---

## Licence
MIT - MineOS Project
