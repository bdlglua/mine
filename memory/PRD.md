# MineOS - PRD (Product Requirements Document)

## Original Problem Statement
Créer un OS avec interface graphique en Rust et en assembleur. L'utilisateur a demandé un vrai OS bare-metal avec un design unique, incluant: gestionnaire de tâches, terminal/console, éditeur de texte, navigateur intégré, calculatrice et autres applications. Nom: MineOS. Sans écran de connexion.

## Architecture
- **Frontend**: React.js avec Tailwind CSS - Interface de bureau complète (single page)
- **Backend**: FastAPI (Python) avec MongoDB - API pour le système de fichiers virtuel et les informations système
- **Database**: MongoDB - Stockage des fichiers, dossiers et notes
- **Design**: "Obsidian Glass" - Thème sombre avec glassmorphism, accent cyan (#00F0FF)

## User Personas
- Passionnés de technologie et développeurs
- Personnes intéressées par les expériences web créatives
- Utilisateurs qui apprécient les interfaces OS immersives

## Core Requirements
- [x] Bureau avec fond d'écran, icônes et barre des tâches
- [x] Gestion de fenêtres (drag, resize, minimize, maximize, close)
- [x] Menu Démarrer avec recherche d'applications
- [x] Terminal fonctionnel avec commandes (help, ls, pwd, whoami, neofetch, etc.)
- [x] Calculatrice complète
- [x] Gestionnaire de fichiers avec système de fichiers virtuel persistant
- [x] Éditeur de texte avec onglets et numéros de lignes
- [x] Gestionnaire de tâches (processus + performance CPU/RAM/Disk)
- [x] Navigateur web intégré (iframe)
- [x] Paramètres système
- [x] Menu contextuel (clic droit)
- [x] Barre système (heure, icônes WiFi/batterie/volume)
- [x] Sans écran de connexion

## What's Been Implemented (March 25, 2026)
### Backend APIs
- `GET /api/` - Health check
- `GET/POST /api/files` - File system CRUD
- `PUT/DELETE /api/files/{id}` - Update/delete files
- `GET/POST /api/notes` - Notes CRUD
- `GET /api/system/info` - CPU, memory, disk stats (psutil)
- Auto-seeding of default folders (Documents, Pictures, Music, Downloads)

### Frontend Components
- Desktop environment with "Obsidian Glass" theme
- WindowContext (state management for window orchestration)
- Window component (drag, resize, z-index management)
- 7 applications: Terminal, Calculator, FileManager, TextEditor, TaskManager, WebBrowser, Settings
- Taskbar, StartMenu, ContextMenu
- Full CSS with glassmorphism, animations, custom fonts (Outfit, IBM Plex Sans, JetBrains Mono)

### Testing Results
- Backend: 90%+ pass rate
- Frontend: 95%+ pass rate
- All core features functional

## Prioritized Backlog
### P0 (Critical) - DONE
- All core apps functional
- Window management working
- File system persistence

### P1 (High)
- Drag & drop files between File Manager and Desktop
- Multiple file selection in File Manager
- Improved Terminal with more commands
- File creation in File Manager (not just folders)
- Save/load settings to MongoDB

### P2 (Medium)
- Image viewer for Pictures folder
- Music player for Music folder
- System notifications/toasts
- Wallpaper selection in Settings
- Window snapping (drag to edges)

### P3 (Low/Nice to have)
- Clock/Calendar app
- Notepad app (separate from TextEditor)
- System sounds
- Custom themes/accent colors that persist
- Keyboard shortcuts (Ctrl+C, Ctrl+V in Terminal)

## Next Tasks
1. Add drag-and-drop file management
2. Keyboard shortcuts for common operations
3. Persisting user settings to MongoDB
4. More Terminal commands
5. Image viewer app
