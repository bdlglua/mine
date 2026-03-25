import React, { useState, useEffect } from 'react';
import { useWindows } from '@/contexts/WindowContext';
import { Hexagon, Wifi, Battery, Volume2 } from 'lucide-react';

export const Taskbar = ({ startMenuOpen, onToggleStartMenu, appConfig }) => {
  const { windows, focusWindow } = useWindows();
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    const interval = setInterval(() => setTime(new Date()), 1000);
    return () => clearInterval(interval);
  }, []);

  const formatTime = (d) => {
    return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  const formatDate = (d) => {
    return d.toLocaleDateString([], { month: 'short', day: 'numeric' });
  };

  return (
    <div className="os-taskbar" data-testid="taskbar">
      <button
        className={`taskbar-start-btn ${startMenuOpen ? 'active' : ''}`}
        onClick={(e) => { e.stopPropagation(); onToggleStartMenu(); }}
        data-testid="start-menu-button"
      >
        <Hexagon size={20} />
      </button>

      <div className="taskbar-apps" data-testid="taskbar-apps">
        {windows.map((win) => (
          <button
            key={win.id}
            className={`taskbar-app-btn ${!win.minimized ? 'active' : ''}`}
            onClick={(e) => { e.stopPropagation(); focusWindow(win.id); }}
            data-testid={`taskbar-app-${win.appId}`}
            style={{ position: 'relative' }}
          >
            {appConfig[win.appId]?.icon}
            <span>{win.title}</span>
          </button>
        ))}
      </div>

      <div className="taskbar-tray" data-testid="system-tray">
        <Wifi size={14} style={{ opacity: 0.7 }} />
        <Volume2 size={14} style={{ opacity: 0.7 }} />
        <Battery size={14} style={{ opacity: 0.7 }} />
        <span data-testid="taskbar-time">{formatTime(time)}</span>
        <span style={{ opacity: 0.6 }}>{formatDate(time)}</span>
      </div>
    </div>
  );
};
