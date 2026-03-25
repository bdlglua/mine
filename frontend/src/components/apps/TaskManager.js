import React, { useState, useEffect, useCallback } from 'react';
import { useWindows } from '@/contexts/WindowContext';
import axios from 'axios';

const API = `${process.env.REACT_APP_BACKEND_URL}/api`;

export const TaskManager = () => {
  const { windows, closeWindow } = useWindows();
  const [tab, setTab] = useState('processes');
  const [sysInfo, setSysInfo] = useState(null);

  const fetchSysInfo = useCallback(async () => {
    try {
      const res = await axios.get(`${API}/system/info`);
      setSysInfo(res.data);
    } catch (err) {
      console.error('Failed to fetch system info', err);
    }
  }, []);

  useEffect(() => {
    fetchSysInfo();
    const interval = setInterval(fetchSysInfo, 3000);
    return () => clearInterval(interval);
  }, [fetchSysInfo]);

  const formatBytes = (bytes) => {
    if (!bytes) return '0 B';
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return (bytes / Math.pow(1024, i)).toFixed(1) + ' ' + sizes[i];
  };

  return (
    <div className="task-manager" data-testid="taskmanager-app">
      <div className="task-manager-tabs">
        <div className={`task-manager-tab ${tab === 'processes' ? 'active' : ''}`} onClick={() => setTab('processes')} data-testid="tm-tab-processes">
          Processes
        </div>
        <div className={`task-manager-tab ${tab === 'performance' ? 'active' : ''}`} onClick={() => setTab('performance')} data-testid="tm-tab-performance">
          Performance
        </div>
      </div>
      <div className="task-manager-content">
        {tab === 'performance' && sysInfo && (
          <div className="task-manager-stats">
            <div className="task-manager-stat">
              <div className="task-manager-stat-label">CPU</div>
              <div className="task-manager-stat-value" data-testid="tm-cpu-value">{sysInfo.cpu_percent}%</div>
              <div className="progress-bar" style={{ marginTop: 8 }}>
                <div className="progress-bar-fill" style={{ width: `${sysInfo.cpu_percent}%` }} />
              </div>
            </div>
            <div className="task-manager-stat">
              <div className="task-manager-stat-label">Memory</div>
              <div className="task-manager-stat-value" data-testid="tm-mem-value">{sysInfo.memory.percent}%</div>
              <div className="task-manager-stat-sub">{formatBytes(sysInfo.memory.used)} / {formatBytes(sysInfo.memory.total)}</div>
              <div className="progress-bar" style={{ marginTop: 4 }}>
                <div className="progress-bar-fill" style={{ width: `${sysInfo.memory.percent}%` }} />
              </div>
            </div>
            <div className="task-manager-stat">
              <div className="task-manager-stat-label">Disk</div>
              <div className="task-manager-stat-value" data-testid="tm-disk-value">{sysInfo.disk.percent}%</div>
              <div className="task-manager-stat-sub">{formatBytes(sysInfo.disk.used)} / {formatBytes(sysInfo.disk.total)}</div>
              <div className="progress-bar" style={{ marginTop: 4 }}>
                <div className="progress-bar-fill" style={{ width: `${sysInfo.disk.percent}%` }} />
              </div>
            </div>
          </div>
        )}
        {tab === 'processes' && (
          <div className="task-manager-process-list">
            <div className="task-manager-process-header">
              <span>Name</span>
              <span>Status</span>
              <span>Memory</span>
              <span></span>
            </div>
            {windows.length === 0 ? (
              <div style={{ padding: '20px', textAlign: 'center', color: '#475569', fontSize: 13 }}>No running processes</div>
            ) : (
              windows.map((w) => (
                <div key={w.id} className="task-manager-process-row" data-testid={`tm-process-${w.appId}`}>
                  <span style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                    {w.icon}
                    {w.title}
                  </span>
                  <span style={{ color: w.minimized ? '#475569' : '#00F0FF', fontFamily: 'JetBrains Mono, monospace', fontSize: 12 }}>
                    {w.minimized ? 'Suspended' : 'Running'}
                  </span>
                  <span style={{ fontFamily: 'JetBrains Mono, monospace', fontSize: 12, color: '#94A3B8' }}>
                    {Math.floor(Math.random() * 50 + 10)} MB
                  </span>
                  <span>
                    <button className="task-manager-end-btn" onClick={() => closeWindow(w.id)} data-testid={`tm-end-${w.appId}`}>
                      End
                    </button>
                  </span>
                </div>
              ))
            )}
          </div>
        )}
      </div>
    </div>
  );
};
