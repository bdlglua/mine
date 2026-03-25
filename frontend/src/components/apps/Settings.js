import React, { useState } from 'react';
import { Monitor, Palette, Bell, Info, Volume2 } from 'lucide-react';

export const Settings = () => {
  const [activeSection, setActiveSection] = useState('display');
  const [settings, setSettings] = useState({
    animations: true,
    soundEffects: false,
    notifications: true,
    darkMode: true,
    wallpaper: 'obsidian',
    accentColor: '#00F0FF',
    fontSize: 'medium',
  });

  const updateSetting = (key, value) => {
    setSettings(prev => ({ ...prev, [key]: value }));
  };

  const sections = [
    { id: 'display', label: 'Display', icon: <Monitor size={16} /> },
    { id: 'appearance', label: 'Appearance', icon: <Palette size={16} /> },
    { id: 'sound', label: 'Sound', icon: <Volume2 size={16} /> },
    { id: 'notifications', label: 'Notifications', icon: <Bell size={16} /> },
    { id: 'about', label: 'About', icon: <Info size={16} /> },
  ];

  const Toggle = ({ checked, onChange }) => (
    <div
      style={{
        width: 40, height: 22, borderRadius: 11, cursor: 'pointer',
        background: checked ? '#00F0FF' : 'rgba(255,255,255,0.1)',
        padding: 2, transition: 'background-color 0.2s',
        display: 'flex', alignItems: 'center',
      }}
      onClick={() => onChange(!checked)}
    >
      <div style={{
        width: 18, height: 18, borderRadius: '50%',
        background: checked ? '#05050A' : '#475569',
        transform: checked ? 'translateX(18px)' : 'translateX(0)',
        transition: 'transform 0.2s, background-color 0.2s',
      }} />
    </div>
  );

  return (
    <div className="settings" data-testid="settings-app">
      <div className="settings-sidebar">
        {sections.map(s => (
          <div
            key={s.id}
            className={`settings-sidebar-item ${activeSection === s.id ? 'active' : ''}`}
            onClick={() => setActiveSection(s.id)}
            data-testid={`settings-nav-${s.id}`}
          >
            {s.icon}
            {s.label}
          </div>
        ))}
      </div>
      <div className="settings-main">
        {activeSection === 'display' && (
          <>
            <h3 className="settings-section-title">Display</h3>
            <div className="settings-row">
              <div>
                <div className="settings-row-label">Animations</div>
                <div className="settings-row-desc">Enable window animations and transitions</div>
              </div>
              <Toggle checked={settings.animations} onChange={(v) => updateSetting('animations', v)} />
            </div>
            <div className="settings-row">
              <div>
                <div className="settings-row-label">Font Size</div>
                <div className="settings-row-desc">Adjust system font size</div>
              </div>
              <select
                value={settings.fontSize}
                onChange={(e) => updateSetting('fontSize', e.target.value)}
                style={{
                  background: 'rgba(255,255,255,0.06)', border: '1px solid rgba(255,255,255,0.1)',
                  color: '#F8FAFC', padding: '6px 10px', borderRadius: 6, outline: 'none',
                  fontFamily: 'IBM Plex Sans, sans-serif', fontSize: 13,
                }}
                data-testid="settings-fontsize-select"
              >
                <option value="small">Small</option>
                <option value="medium">Medium</option>
                <option value="large">Large</option>
              </select>
            </div>
          </>
        )}
        {activeSection === 'appearance' && (
          <>
            <h3 className="settings-section-title">Appearance</h3>
            <div className="settings-row">
              <div>
                <div className="settings-row-label">Dark Mode</div>
                <div className="settings-row-desc">Use dark theme (always on in MineOS)</div>
              </div>
              <Toggle checked={settings.darkMode} onChange={(v) => updateSetting('darkMode', v)} />
            </div>
            <div className="settings-row">
              <div>
                <div className="settings-row-label">Accent Color</div>
                <div className="settings-row-desc">Primary accent color for the system</div>
              </div>
              <div style={{ display: 'flex', gap: 8 }}>
                {['#00F0FF', '#FF5500', '#22C55E', '#A855F7', '#EAB308'].map(color => (
                  <div
                    key={color}
                    style={{
                      width: 28, height: 28, borderRadius: '50%', background: color, cursor: 'pointer',
                      border: settings.accentColor === color ? '2px solid #fff' : '2px solid transparent',
                      transition: 'border-color 0.15s',
                    }}
                    onClick={() => updateSetting('accentColor', color)}
                    data-testid={`settings-accent-${color}`}
                  />
                ))}
              </div>
            </div>
          </>
        )}
        {activeSection === 'sound' && (
          <>
            <h3 className="settings-section-title">Sound</h3>
            <div className="settings-row">
              <div>
                <div className="settings-row-label">Sound Effects</div>
                <div className="settings-row-desc">Play sound effects for system events</div>
              </div>
              <Toggle checked={settings.soundEffects} onChange={(v) => updateSetting('soundEffects', v)} />
            </div>
          </>
        )}
        {activeSection === 'notifications' && (
          <>
            <h3 className="settings-section-title">Notifications</h3>
            <div className="settings-row">
              <div>
                <div className="settings-row-label">Enable Notifications</div>
                <div className="settings-row-desc">Show system notifications</div>
              </div>
              <Toggle checked={settings.notifications} onChange={(v) => updateSetting('notifications', v)} />
            </div>
          </>
        )}
        {activeSection === 'about' && (
          <div className="about-content">
            <div className="about-logo">
              <Monitor size={36} color="#00F0FF" />
            </div>
            <div className="about-title">MineOS</div>
            <div className="about-version">v1.0.0</div>
            <div className="about-desc">
              A modern web-based operating system experience. Built with cutting-edge web technologies for a seamless desktop experience.
            </div>
            <div style={{ marginTop: 20, fontSize: 12, color: '#475569' }}>
              WebKernel 6.1 | {navigator.userAgent.split(' ').slice(-1)[0]}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
