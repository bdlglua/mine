import React from 'react';
import { TerminalSquare, FolderOpen, Settings, RefreshCw, Info } from 'lucide-react';

export const ContextMenu = ({ x, y, onClose, onOpenApp }) => {
  const adjustedY = Math.min(y, window.innerHeight - 250);
  const adjustedX = Math.min(x, window.innerWidth - 220);

  const items = [
    { label: 'Open Terminal', icon: <TerminalSquare size={14} />, action: () => { onOpenApp('terminal'); onClose(); } },
    { label: 'Open Files', icon: <FolderOpen size={14} />, action: () => { onOpenApp('filemanager'); onClose(); } },
    { type: 'separator' },
    { label: 'Refresh Desktop', icon: <RefreshCw size={14} />, action: () => { window.location.reload(); } },
    { label: 'Settings', icon: <Settings size={14} />, action: () => { onOpenApp('settings'); onClose(); } },
    { type: 'separator' },
    { label: 'About MineOS', icon: <Info size={14} />, action: () => { onOpenApp('settings'); onClose(); } },
  ];

  return (
    <div
      className="context-menu"
      style={{ left: adjustedX, top: adjustedY }}
      onClick={(e) => e.stopPropagation()}
      data-testid="context-menu"
    >
      {items.map((item, i) => {
        if (item.type === 'separator') {
          return <div key={i} className="context-menu-separator" />;
        }
        return (
          <div
            key={i}
            className="context-menu-item"
            onClick={item.action}
            data-testid={`context-menu-${item.label.toLowerCase().replace(/\s/g, '-')}`}
          >
            {item.icon}
            {item.label}
          </div>
        );
      })}
    </div>
  );
};
