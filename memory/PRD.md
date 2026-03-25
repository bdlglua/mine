# MineOS - PRD (Product Requirements Document)

## Original Problem Statement
Créer un OS avec interface graphique en Rust et en assembleur. L'utilisateur a demandé un vrai OS bare-metal avec un design unique, incluant: gestionnaire de tâches, terminal/console, éditeur de texte, navigateur intégré, calculatrice et autres applications. Nom: MineOS. Sans écran de connexion.

## Architecture

### Partie 1: Kernel Bare-Metal (Rust + Assembleur)
- **Assembleur x86_64 (NASM)**: Bootloader Multiboot2, GDT, pagination, ISR stubs, I/O ports
- **Rust (no_std)**: Kernel avec pilotes VGA, framebuffer, clavier PS/2, PIC 8259, gestion mémoire
- **GUI**: Desktop complet avec fenêtres, barre des tâches, icônes, widgets
- **Apps**: Terminal (10+ commandes), Calculatrice
- **Build**: cargo-bootimage → image bootable 117KB

### Partie 2: Démo Web (React + Python + MongoDB)
- **Frontend**: React.js - Environnement de bureau interactif complet
- **Backend**: FastAPI - API système de fichiers virtuel
- **Design**: "Obsidian Glass" - Thème sombre avec glassmorphism

## What's Been Implemented (March 25, 2026)

### Kernel Bare-Metal (3020 lignes - 2599 Rust + 421 ASM)
**Assembleur (asm/)**:
- boot.asm: Bootloader Multiboot2 complet (GDT 64-bit, pagination, Real→Protected→Long Mode)
- interrupts.asm: ISR stubs (exceptions CPU + IRQs matériels), fonctions I/O port

**Rust (src/)**:
- drivers/: VGA texte, framebuffer 32-bit (pixel/ligne/rect/texte), clavier PS/2, PIC 8259, mémoire (4MB heap)
- gui/: Desktop, gestionnaire fenêtres, barre des tâches, police bitmap 8x16, widgets UI
- apps/: Terminal (help/ls/whoami/neofetch/calc/reboot/shutdown), Calculatrice

**Compilation**: ✅ Réussie - Image bootable générée (117KB)

### Démo Web (7 applications)
- Terminal, Calculatrice, Gestionnaire de fichiers, Éditeur de texte
- Gestionnaire de tâches, Navigateur web, Paramètres
- Testée: 90%+ backend, 95%+ frontend

## Prioritized Backlog
### P0 - DONE
- Kernel qui compile et génère une image bootable
- Bootloader ASM avec Multiboot2 + Long Mode
- Pilotes matériels (VGA, clavier, PIC, mémoire)
- GUI framework (desktop, fenêtres, taskbar)
- Terminal fonctionnel avec commandes

### P1 (High)
- Support souris PS/2
- Mode VBE/VESA pour framebuffer haute résolution
- Système de fichiers FAT16/FAT32
- Pilote ATA/IDE pour disque dur

### P2 (Medium)
- Multitâche préemptif
- Pilote réseau (RTL8139 ou E1000)
- Plus d'applications GUI
- Gestion des processus

### P3 (Low)
- USB support
- Son (AC97/HDA)
- Stack TCP/IP
- Navigateur web basique

## Next Tasks
1. Ajouter support souris PS/2
2. Implémenter mode VBE pour graphiques haute résolution
3. Ajouter système de fichiers FAT16
4. Multitâche coopératif
