import React, { createContext, useContext, useState, useCallback, useRef } from 'react';

const WindowContext = createContext(null);

const DEFAULT_SIZES = {
  terminal: { width: 700, height: 450 },
  calculator: { width: 320, height: 480 },
  filemanager: { width: 750, height: 500 },
  texteditor: { width: 750, height: 520 },
  taskmanager: { width: 700, height: 500 },
  webbrowser: { width: 900, height: 600 },
  settings: { width: 700, height: 480 },
  about: { width: 400, height: 380 },
};

export const WindowProvider = ({ children }) => {
  const [windows, setWindows] = useState([]);
  const [focusedId, setFocusedId] = useState(null);
  const zIndexRef = useRef(100);

  const openWindow = useCallback((appId, title, icon) => {
    setWindows(prev => {
      const existing = prev.find(w => w.appId === appId);
      if (existing) {
        setFocusedId(existing.id);
        return prev.map(w => w.id === existing.id ? { ...w, minimized: false, zIndex: ++zIndexRef.current } : w);
      }
      const size = DEFAULT_SIZES[appId] || { width: 600, height: 400 };
      const offset = prev.length * 30;
      const newWin = {
        id: `win-${Date.now()}`,
        appId,
        title,
        icon,
        x: 80 + offset,
        y: 40 + offset,
        width: size.width,
        height: size.height,
        minimized: false,
        maximized: false,
        zIndex: ++zIndexRef.current,
        prevBounds: null,
      };
      setFocusedId(newWin.id);
      return [...prev, newWin];
    });
  }, []);

  const closeWindow = useCallback((id) => {
    setWindows(prev => prev.filter(w => w.id !== id));
    setFocusedId(null);
  }, []);

  const minimizeWindow = useCallback((id) => {
    setWindows(prev => prev.map(w => w.id === id ? { ...w, minimized: true } : w));
    setFocusedId(null);
  }, []);

  const maximizeWindow = useCallback((id) => {
    setWindows(prev => prev.map(w => {
      if (w.id !== id) return w;
      if (w.maximized) {
        return { ...w, maximized: false, ...(w.prevBounds || {}), zIndex: ++zIndexRef.current };
      }
      return {
        ...w,
        maximized: true,
        prevBounds: { x: w.x, y: w.y, width: w.width, height: w.height },
        x: 0, y: 0,
        width: window.innerWidth,
        height: window.innerHeight - 48,
        zIndex: ++zIndexRef.current,
      };
    }));
    setFocusedId(id);
  }, []);

  const focusWindow = useCallback((id) => {
    setWindows(prev => prev.map(w => w.id === id ? { ...w, zIndex: ++zIndexRef.current, minimized: false } : w));
    setFocusedId(id);
  }, []);

  const updateWindowPosition = useCallback((id, x, y) => {
    setWindows(prev => prev.map(w => w.id === id ? { ...w, x, y } : w));
  }, []);

  const updateWindowSize = useCallback((id, width, height) => {
    setWindows(prev => prev.map(w => w.id === id ? { ...w, width, height } : w));
  }, []);

  return (
    <WindowContext.Provider value={{
      windows, focusedId, openWindow, closeWindow, minimizeWindow,
      maximizeWindow, focusWindow, updateWindowPosition, updateWindowSize,
    }}>
      {children}
    </WindowContext.Provider>
  );
};

export const useWindows = () => {
  const ctx = useContext(WindowContext);
  if (!ctx) throw new Error('useWindows must be used within WindowProvider');
  return ctx;
};
