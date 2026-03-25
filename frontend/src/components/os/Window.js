import React, { useRef, useCallback, useEffect, useState } from 'react';
import { useWindows } from '@/contexts/WindowContext';
import { Minus, Square, X, Maximize2, Minimize2 } from 'lucide-react';

export const Window = ({ windowData, children }) => {
  const { closeWindow, minimizeWindow, maximizeWindow, focusWindow, updateWindowPosition, updateWindowSize, focusedId } = useWindows();
  const windowRef = useRef(null);
  const dragRef = useRef({ dragging: false, startX: 0, startY: 0, origX: 0, origY: 0 });
  const resizeRef = useRef({ resizing: false, startX: 0, startY: 0, origW: 0, origH: 0, origX: 0, origY: 0, dir: '' });
  const [localPos, setLocalPos] = useState({ x: windowData.x, y: windowData.y });
  const [localSize, setLocalSize] = useState({ w: windowData.width, h: windowData.height });
  const isFocused = focusedId === windowData.id;

  useEffect(() => {
    if (!dragRef.current.dragging) setLocalPos({ x: windowData.x, y: windowData.y });
  }, [windowData.x, windowData.y]);

  useEffect(() => {
    if (!resizeRef.current.resizing) setLocalSize({ w: windowData.width, h: windowData.height });
  }, [windowData.width, windowData.height]);

  const handleMouseDownDrag = useCallback((e) => {
    if (windowData.maximized) return;
    e.preventDefault();
    focusWindow(windowData.id);
    dragRef.current = { dragging: true, startX: e.clientX, startY: e.clientY, origX: localPos.x, origY: localPos.y };

    const handleMove = (e) => {
      const dx = e.clientX - dragRef.current.startX;
      const dy = e.clientY - dragRef.current.startY;
      const nx = dragRef.current.origX + dx;
      const ny = Math.max(0, dragRef.current.origY + dy);
      setLocalPos({ x: nx, y: ny });
    };

    const handleUp = () => {
      dragRef.current.dragging = false;
      document.removeEventListener('mousemove', handleMove);
      document.removeEventListener('mouseup', handleUp);
      updateWindowPosition(windowData.id, localPos.x, localPos.y);
    };

    document.addEventListener('mousemove', handleMove);
    document.addEventListener('mouseup', handleUp);
  }, [windowData.id, windowData.maximized, localPos, focusWindow, updateWindowPosition]);

  const handleResize = useCallback((e, dir) => {
    e.preventDefault();
    e.stopPropagation();
    focusWindow(windowData.id);
    resizeRef.current = { resizing: true, startX: e.clientX, startY: e.clientY, origW: localSize.w, origH: localSize.h, origX: localPos.x, origY: localPos.y, dir };

    const handleMove = (e) => {
      const dx = e.clientX - resizeRef.current.startX;
      const dy = e.clientY - resizeRef.current.startY;
      let nw = resizeRef.current.origW;
      let nh = resizeRef.current.origH;
      let nx = resizeRef.current.origX;
      let ny = resizeRef.current.origY;

      if (dir.includes('e')) nw = Math.max(300, resizeRef.current.origW + dx);
      if (dir.includes('s')) nh = Math.max(200, resizeRef.current.origH + dy);
      if (dir.includes('w')) { nw = Math.max(300, resizeRef.current.origW - dx); nx = resizeRef.current.origX + dx; }
      if (dir.includes('n')) { nh = Math.max(200, resizeRef.current.origH - dy); ny = resizeRef.current.origY + dy; }

      setLocalSize({ w: nw, h: nh });
      setLocalPos({ x: nx, y: ny });
    };

    const handleUp = () => {
      resizeRef.current.resizing = false;
      document.removeEventListener('mousemove', handleMove);
      document.removeEventListener('mouseup', handleUp);
      updateWindowSize(windowData.id, localSize.w, localSize.h);
      updateWindowPosition(windowData.id, localPos.x, localPos.y);
    };

    document.addEventListener('mousemove', handleMove);
    document.addEventListener('mouseup', handleUp);
  }, [windowData.id, localSize, localPos, focusWindow, updateWindowSize, updateWindowPosition]);

  if (windowData.minimized) return null;

  return (
    <div
      ref={windowRef}
      className={`os-window window-opening ${isFocused ? 'focused' : ''}`}
      style={{
        left: localPos.x,
        top: localPos.y,
        width: localSize.w,
        height: localSize.h,
        zIndex: windowData.zIndex,
        borderRadius: windowData.maximized ? 0 : undefined,
      }}
      onMouseDown={() => focusWindow(windowData.id)}
      data-testid={`window-${windowData.appId}`}
    >
      <div className="os-window-titlebar" onMouseDown={handleMouseDownDrag} onDoubleClick={() => maximizeWindow(windowData.id)}>
        <div className="os-window-titlebar-left">
          {windowData.icon}
          <span className="os-window-title">{windowData.title}</span>
        </div>
        <div className="os-window-controls">
          <button className="os-window-btn" onClick={(e) => { e.stopPropagation(); minimizeWindow(windowData.id); }} data-testid={`minimize-${windowData.appId}`}>
            <Minus size={14} />
          </button>
          <button className="os-window-btn" onClick={(e) => { e.stopPropagation(); maximizeWindow(windowData.id); }} data-testid={`maximize-${windowData.appId}`}>
            {windowData.maximized ? <Minimize2 size={14} /> : <Maximize2 size={14} />}
          </button>
          <button className="os-window-btn close" onClick={(e) => { e.stopPropagation(); closeWindow(windowData.id); }} data-testid={`close-${windowData.appId}`}>
            <X size={14} />
          </button>
        </div>
      </div>
      <div className="os-window-content">
        {children}
      </div>
      {!windowData.maximized && (
        <>
          <div className="resize-handle resize-handle-se" onMouseDown={(e) => handleResize(e, 'se')} />
          <div className="resize-handle resize-handle-e" onMouseDown={(e) => handleResize(e, 'e')} />
          <div className="resize-handle resize-handle-s" onMouseDown={(e) => handleResize(e, 's')} />
          <div className="resize-handle resize-handle-w" onMouseDown={(e) => handleResize(e, 'w')} />
          <div className="resize-handle resize-handle-n" onMouseDown={(e) => handleResize(e, 'n')} />
        </>
      )}
    </div>
  );
};
