import React, { useState, useMemo } from 'react';
import {
  TerminalSquare, Calculator, FolderOpen, FileText,
  Activity, Globe, Settings, Power, Monitor
} from 'lucide-react';

const ALL_APPS = [
  { id: 'terminal', label: 'Terminal', icon: <TerminalSquare size={28} color="#00F0FF" /> },
  { id: 'calculator', label: 'Calculator', icon: <Calculator size={28} color="#00F0FF" /> },
  { id: 'filemanager', label: 'Files', icon: <FolderOpen size={28} color="#00F0FF" /> },
  { id: 'texteditor', label: 'Text Editor', icon: <FileText size={28} color="#00F0FF" /> },
  { id: 'taskmanager', label: 'Task Manager', icon: <Activity size={28} color="#00F0FF" /> },
  { id: 'webbrowser', label: 'Browser', icon: <Globe size={28} color="#00F0FF" /> },
  { id: 'settings', label: 'Settings', icon: <Settings size={28} color="#00F0FF" /> },
];

export const StartMenu = ({ onClose, onOpenApp }) => {
  const [search, setSearch] = useState('');

  const filtered = useMemo(() => {
    if (!search.trim()) return ALL_APPS;
    return ALL_APPS.filter(app => app.label.toLowerCase().includes(search.toLowerCase()));
  }, [search]);

  return (
    <div className="start-menu" onClick={(e) => e.stopPropagation()} data-testid="start-menu">
      <div className="start-menu-header">
        <input
          className="start-menu-search"
          placeholder="Search apps..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          autoFocus
          data-testid="start-menu-search"
        />
      </div>
      <div className="start-menu-grid">
        {filtered.map((app) => (
          <div
            key={app.id}
            className="start-menu-item"
            onClick={() => onOpenApp(app.id)}
            data-testid={`start-menu-app-${app.id}`}
          >
            {app.icon}
            <span className="start-menu-item-label">{app.label}</span>
          </div>
        ))}
        {filtered.length === 0 && (
          <div style={{ gridColumn: '1/-1', textAlign: 'center', padding: 20, color: '#475569', fontSize: 13 }}>
            No apps found
          </div>
        )}
      </div>
      <div className="start-menu-footer">
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <Monitor size={16} color="#00F0FF" />
          <span style={{ fontFamily: 'Outfit, sans-serif', fontWeight: 600, fontSize: 14, color: '#F8FAFC' }}>MineOS</span>
          <span style={{ fontFamily: 'JetBrains Mono, monospace', fontSize: 10, color: '#475569' }}>v1.0.0</span>
        </div>
        <button className="start-menu-power-btn" data-testid="start-menu-power-btn">
          <Power size={14} />
          Power
        </button>
      </div>
    </div>
  );
};
