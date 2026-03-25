import React, { useState, useEffect, useCallback } from 'react';
import { Folder, File, ArrowLeft, ArrowRight, Plus, Trash2, Home } from 'lucide-react';
import axios from 'axios';

const API = `${process.env.REACT_APP_BACKEND_URL}/api`;

export const FileManager = () => {
  const [files, setFiles] = useState([]);
  const [currentPath, setCurrentPath] = useState('/');
  const [pathHistory, setPathHistory] = useState(['/']);
  const [historyIdx, setHistoryIdx] = useState(0);
  const [selected, setSelected] = useState(null);
  const [loading, setLoading] = useState(false);

  const loadFiles = useCallback(async (path) => {
    setLoading(true);
    try {
      const res = await axios.get(`${API}/files`, { params: { parent_path: path } });
      setFiles(res.data);
    } catch (err) {
      console.error('Failed to load files', err);
    }
    setLoading(false);
  }, []);

  useEffect(() => {
    loadFiles(currentPath);
  }, [currentPath, loadFiles]);

  const navigate = (path) => {
    setCurrentPath(path);
    setSelected(null);
    const newHistory = [...pathHistory.slice(0, historyIdx + 1), path];
    setPathHistory(newHistory);
    setHistoryIdx(newHistory.length - 1);
  };

  const goBack = () => {
    if (historyIdx > 0) {
      const newIdx = historyIdx - 1;
      setHistoryIdx(newIdx);
      setCurrentPath(pathHistory[newIdx]);
      setSelected(null);
    }
  };

  const goForward = () => {
    if (historyIdx < pathHistory.length - 1) {
      const newIdx = historyIdx + 1;
      setHistoryIdx(newIdx);
      setCurrentPath(pathHistory[newIdx]);
      setSelected(null);
    }
  };

  const goHome = () => navigate('/');

  const handleDoubleClick = (file) => {
    if (file.type === 'folder') {
      navigate(file.path);
    }
  };

  const createFolder = async () => {
    const name = prompt('Folder name:');
    if (!name) return;
    try {
      await axios.post(`${API}/files`, {
        name,
        path: currentPath === '/' ? `/${name}` : `${currentPath}/${name}`,
        type: 'folder',
        parent_path: currentPath,
      });
      loadFiles(currentPath);
    } catch (err) {
      console.error('Failed to create folder', err);
    }
  };

  const deleteSelected = async () => {
    if (!selected) return;
    try {
      await axios.delete(`${API}/files/${selected.id}`);
      setSelected(null);
      loadFiles(currentPath);
    } catch (err) {
      console.error('Failed to delete', err);
    }
  };

  const quickPaths = [
    { name: 'Home', path: '/' },
    { name: 'Documents', path: '/Documents' },
    { name: 'Pictures', path: '/Pictures' },
    { name: 'Music', path: '/Music' },
    { name: 'Downloads', path: '/Downloads' },
  ];

  return (
    <div className="file-manager" data-testid="filemanager-app">
      <div className="file-manager-sidebar">
        {quickPaths.map((qp) => (
          <div
            key={qp.path}
            className={`file-manager-sidebar-item ${currentPath === qp.path ? 'active' : ''}`}
            onClick={() => navigate(qp.path)}
            data-testid={`fm-nav-${qp.name.toLowerCase()}`}
          >
            <Folder size={16} />
            {qp.name}
          </div>
        ))}
      </div>
      <div className="file-manager-main">
        <div className="file-manager-toolbar">
          <button className="os-window-btn" onClick={goBack} disabled={historyIdx <= 0} data-testid="fm-back-btn">
            <ArrowLeft size={16} />
          </button>
          <button className="os-window-btn" onClick={goForward} disabled={historyIdx >= pathHistory.length - 1} data-testid="fm-forward-btn">
            <ArrowRight size={16} />
          </button>
          <button className="os-window-btn" onClick={goHome} data-testid="fm-home-btn">
            <Home size={16} />
          </button>
          <input className="file-manager-path" value={currentPath} readOnly data-testid="fm-path-bar" />
          <button className="os-window-btn" onClick={createFolder} data-testid="fm-new-folder-btn" title="New Folder">
            <Plus size={16} />
          </button>
          <button className="os-window-btn" onClick={deleteSelected} disabled={!selected} data-testid="fm-delete-btn" title="Delete">
            <Trash2 size={16} />
          </button>
        </div>
        <div className="file-manager-grid">
          {loading ? (
            <div style={{ gridColumn: '1/-1', textAlign: 'center', padding: '40px', color: '#475569' }}>Loading...</div>
          ) : files.length === 0 ? (
            <div style={{ gridColumn: '1/-1', textAlign: 'center', padding: '40px', color: '#475569' }}>Empty folder</div>
          ) : (
            files.map((file) => (
              <div
                key={file.id}
                className={`file-item ${selected?.id === file.id ? 'selected' : ''}`}
                onClick={() => setSelected(file)}
                onDoubleClick={() => handleDoubleClick(file)}
                data-testid={`file-item-${file.name}`}
              >
                {file.type === 'folder' ? (
                  <Folder size={36} color="#00F0FF" />
                ) : (
                  <File size={36} color="#94A3B8" />
                )}
                <span className="file-item-name">{file.name}</span>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
};
