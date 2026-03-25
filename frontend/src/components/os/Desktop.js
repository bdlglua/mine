import React, { useState, useEffect, useCallback } from 'react';
import { useWindows } from '@/contexts/WindowContext';
import { Taskbar } from '@/components/os/Taskbar';
import { StartMenu } from '@/components/os/StartMenu';
import { ContextMenu } from '@/components/os/ContextMenu';
import { Window } from '@/components/os/Window';
import { Terminal } from '@/components/apps/Terminal';
import { Calculator } from '@/components/apps/Calculator';
import { FileManager } from '@/components/apps/FileManager';
import { TextEditor } from '@/components/apps/TextEditor';
import { TaskManager } from '@/components/apps/TaskManager';
import { WebBrowser } from '@/components/apps/WebBrowser';
import { Settings } from '@/components/apps/Settings';
import {
  TerminalSquare, Calculator as CalcIcon, FolderOpen, FileText,
  Activity, Globe, SettingsIcon, Monitor
} from 'lucide-react';
import axios from 'axios';

const API = `${process.env.REACT_APP_BACKEND_URL}/api`;

const APP_CONFIG = {
  terminal: { title: 'Terminal', icon: <TerminalSquare size={16} color="#00F0FF" />, desktopIcon: <TerminalSquare size={36} color="#00F0FF" />, component: Terminal },
  calculator: { title: 'Calculator', icon: <CalcIcon size={16} color="#00F0FF" />, desktopIcon: <CalcIcon size={36} color="#00F0FF" />, component: Calculator },
  filemanager: { title: 'Files', icon: <FolderOpen size={16} color="#00F0FF" />, desktopIcon: <FolderOpen size={36} color="#00F0FF" />, component: FileManager },
  texteditor: { title: 'Text Editor', icon: <FileText size={16} color="#00F0FF" />, desktopIcon: <FileText size={36} color="#00F0FF" />, component: TextEditor },
  taskmanager: { title: 'Task Manager', icon: <Activity size={16} color="#00F0FF" />, desktopIcon: <Activity size={36} color="#00F0FF" />, component: TaskManager },
  webbrowser: { title: 'Browser', icon: <Globe size={16} color="#00F0FF" />, desktopIcon: <Globe size={36} color="#00F0FF" />, component: WebBrowser },
  settings: { title: 'Settings', icon: <SettingsIcon size={16} color="#00F0FF" />, desktopIcon: <SettingsIcon size={36} color="#00F0FF" />, component: Settings },
};

const DESKTOP_ICONS = [
  { appId: 'terminal', label: 'Terminal' },
  { appId: 'filemanager', label: 'Files' },
  { appId: 'texteditor', label: 'Text Editor' },
  { appId: 'calculator', label: 'Calculator' },
  { appId: 'webbrowser', label: 'Browser' },
  { appId: 'taskmanager', label: 'Task Manager' },
  { appId: 'settings', label: 'Settings' },
];

export const Desktop = () => {
  const { windows, openWindow } = useWindows();
  const [startMenuOpen, setStartMenuOpen] = useState(false);
  const [contextMenu, setContextMenu] = useState(null);
  const [selectedIcon, setSelectedIcon] = useState(null);

  useEffect(() => {
    const seedFS = async () => {
      try { await axios.post(`${API}/seed`); } catch {}
    };
    seedFS();
  }, []);

  const handleOpenApp = useCallback((appId) => {
    const config = APP_CONFIG[appId];
    if (config) {
      openWindow(appId, config.title, config.icon);
      setStartMenuOpen(false);
    }
  }, [openWindow]);

  const handleDesktopClick = (e) => {
    setContextMenu(null);
    setStartMenuOpen(false);
    setSelectedIcon(null);
  };

  const handleContextMenu = (e) => {
    e.preventDefault();
    setContextMenu({ x: e.clientX, y: e.clientY });
    setStartMenuOpen(false);
  };

  const handleIconClick = (e, appId) => {
    e.stopPropagation();
    setSelectedIcon(appId);
    setContextMenu(null);
  };

  const handleIconDoubleClick = (appId) => {
    handleOpenApp(appId);
  };

  const renderAppContent = (appId) => {
    const config = APP_CONFIG[appId];
    if (!config) return null;
    const Component = config.component;
    return <Component />;
  };

  return (
    <div className="mineos-desktop" onClick={handleDesktopClick} onContextMenu={handleContextMenu} data-testid="mineos-desktop">
      <div className="desktop-wallpaper" />
      <div className="desktop-noise" />
      <div className="desktop-grid" />

      {/* Desktop Icons */}
      <div className="desktop-icons" data-testid="desktop-icons">
        {DESKTOP_ICONS.map((icon) => (
          <div
            key={icon.appId}
            className={`desktop-icon ${selectedIcon === icon.appId ? 'selected' : ''}`}
            onClick={(e) => handleIconClick(e, icon.appId)}
            onDoubleClick={() => handleIconDoubleClick(icon.appId)}
            data-testid={`desktop-icon-${icon.appId}`}
          >
            {APP_CONFIG[icon.appId].desktopIcon}
            <span className="desktop-icon-label">{icon.label}</span>
          </div>
        ))}
      </div>

      {/* Windows */}
      {windows.map((win) => (
        <Window key={win.id} windowData={win}>
          {renderAppContent(win.appId)}
        </Window>
      ))}

      {/* Start Menu */}
      {startMenuOpen && (
        <StartMenu
          onClose={() => setStartMenuOpen(false)}
          onOpenApp={handleOpenApp}
          appConfig={APP_CONFIG}
        />
      )}

      {/* Context Menu */}
      {contextMenu && (
        <ContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          onClose={() => setContextMenu(null)}
          onOpenApp={handleOpenApp}
        />
      )}

      {/* Taskbar */}
      <Taskbar
        startMenuOpen={startMenuOpen}
        onToggleStartMenu={() => { setStartMenuOpen(prev => !prev); setContextMenu(null); }}
        appConfig={APP_CONFIG}
      />
    </div>
  );
};
