import React, { useState, useRef, useEffect, useCallback } from 'react';
import { Save, FilePlus, X } from 'lucide-react';
import axios from 'axios';

const API = `${process.env.REACT_APP_BACKEND_URL}/api`;

export const TextEditor = () => {
  const [tabs, setTabs] = useState([{ id: 'new-1', title: 'Untitled', content: '', saved: true }]);
  const [activeTab, setActiveTab] = useState('new-1');
  const textareaRef = useRef(null);
  const lineNumRef = useRef(null);
  const tabCounter = useRef(2);

  const activeDoc = tabs.find(t => t.id === activeTab);

  useEffect(() => {
    const loadFiles = async () => {
      try {
        const res = await axios.get(`${API}/files/all`);
        const textFiles = res.data.filter(f => f.type === 'file');
        if (textFiles.length > 0) {
          const fileTabs = textFiles.map(f => ({
            id: f.id,
            title: f.name,
            content: f.content || '',
            saved: true,
            fileId: f.id,
          }));
          setTabs(fileTabs);
          setActiveTab(fileTabs[0].id);
        }
      } catch (err) {
        console.error('Failed to load files', err);
      }
    };
    loadFiles();
  }, []);

  const handleScroll = useCallback(() => {
    if (textareaRef.current && lineNumRef.current) {
      lineNumRef.current.scrollTop = textareaRef.current.scrollTop;
    }
  }, []);

  const updateContent = (content) => {
    setTabs(prev => prev.map(t => t.id === activeTab ? { ...t, content, saved: false } : t));
  };

  const newTab = () => {
    const id = `new-${tabCounter.current++}`;
    setTabs(prev => [...prev, { id, title: 'Untitled', content: '', saved: true }]);
    setActiveTab(id);
  };

  const closeTab = (id) => {
    if (tabs.length === 1) return;
    const idx = tabs.findIndex(t => t.id === id);
    const newTabs = tabs.filter(t => t.id !== id);
    setTabs(newTabs);
    if (activeTab === id) {
      setActiveTab(newTabs[Math.min(idx, newTabs.length - 1)].id);
    }
  };

  const saveFile = async () => {
    if (!activeDoc) return;
    try {
      if (activeDoc.fileId) {
        await axios.put(`${API}/files/${activeDoc.fileId}`, { content: activeDoc.content });
      } else {
        const name = activeDoc.title === 'Untitled' ? prompt('File name:', 'untitled.txt') : activeDoc.title;
        if (!name) return;
        const res = await axios.post(`${API}/files`, {
          name,
          path: `/Documents/${name}`,
          type: 'file',
          content: activeDoc.content,
          parent_path: '/Documents',
        });
        setTabs(prev => prev.map(t => t.id === activeTab ? { ...t, title: name, fileId: res.data.id, saved: true } : t));
        return;
      }
      setTabs(prev => prev.map(t => t.id === activeTab ? { ...t, saved: true } : t));
    } catch (err) {
      console.error('Failed to save', err);
    }
  };

  const lineCount = (activeDoc?.content || '').split('\n').length;

  return (
    <div className="text-editor" data-testid="texteditor-app">
      <div className="text-editor-toolbar">
        <button className="text-editor-toolbar-btn" onClick={newTab} data-testid="editor-new-btn">
          <FilePlus size={14} /> New
        </button>
        <button className="text-editor-toolbar-btn" onClick={saveFile} data-testid="editor-save-btn">
          <Save size={14} /> Save
        </button>
      </div>
      <div className="text-editor-tabs">
        {tabs.map(tab => (
          <div
            key={tab.id}
            className={`text-editor-tab ${activeTab === tab.id ? 'active' : ''}`}
            onClick={() => setActiveTab(tab.id)}
            data-testid={`editor-tab-${tab.title}`}
          >
            {tab.title}{!tab.saved && ' *'}
            {tabs.length > 1 && (
              <span className="text-editor-tab-close" onClick={(e) => { e.stopPropagation(); closeTab(tab.id); }}>
                <X size={12} />
              </span>
            )}
          </div>
        ))}
      </div>
      <div className="text-editor-area">
        <div className="text-editor-line-numbers" ref={lineNumRef}>
          {Array.from({ length: lineCount }, (_, i) => (
            <div key={i} style={{ height: '1.6em' }}>{i + 1}</div>
          ))}
        </div>
        <textarea
          ref={textareaRef}
          className="text-editor-textarea"
          value={activeDoc?.content || ''}
          onChange={(e) => updateContent(e.target.value)}
          onScroll={handleScroll}
          spellCheck={false}
          data-testid="editor-textarea"
        />
      </div>
    </div>
  );
};
