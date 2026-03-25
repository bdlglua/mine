import React, { useState, useRef } from 'react';
import { ArrowLeft, ArrowRight, RotateCw, Globe, Search } from 'lucide-react';

export const WebBrowser = () => {
  const [url, setUrl] = useState('https://www.wikipedia.org');
  const [inputUrl, setInputUrl] = useState('https://www.wikipedia.org');
  const [loading, setLoading] = useState(false);
  const [history, setHistory] = useState(['https://www.wikipedia.org']);
  const [historyIdx, setHistoryIdx] = useState(0);
  const iframeRef = useRef(null);

  const navigate = (newUrl) => {
    let finalUrl = newUrl;
    if (!finalUrl.startsWith('http://') && !finalUrl.startsWith('https://')) {
      if (finalUrl.includes('.') && !finalUrl.includes(' ')) {
        finalUrl = 'https://' + finalUrl;
      } else {
        finalUrl = `https://www.google.com/search?igu=1&q=${encodeURIComponent(finalUrl)}`;
      }
    }
    setUrl(finalUrl);
    setInputUrl(finalUrl);
    setLoading(true);
    const newHistory = [...history.slice(0, historyIdx + 1), finalUrl];
    setHistory(newHistory);
    setHistoryIdx(newHistory.length - 1);
  };

  const goBack = () => {
    if (historyIdx > 0) {
      const newIdx = historyIdx - 1;
      setHistoryIdx(newIdx);
      setUrl(history[newIdx]);
      setInputUrl(history[newIdx]);
    }
  };

  const goForward = () => {
    if (historyIdx < history.length - 1) {
      const newIdx = historyIdx + 1;
      setHistoryIdx(newIdx);
      setUrl(history[newIdx]);
      setInputUrl(history[newIdx]);
    }
  };

  const refresh = () => {
    if (iframeRef.current) {
      iframeRef.current.src = url;
      setLoading(true);
    }
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      navigate(inputUrl);
    }
  };

  return (
    <div className="web-browser" data-testid="webbrowser-app">
      <div className="web-browser-toolbar">
        <button className="web-browser-nav-btn" onClick={goBack} disabled={historyIdx <= 0} data-testid="browser-back-btn">
          <ArrowLeft size={16} />
        </button>
        <button className="web-browser-nav-btn" onClick={goForward} disabled={historyIdx >= history.length - 1} data-testid="browser-forward-btn">
          <ArrowRight size={16} />
        </button>
        <button className="web-browser-nav-btn" onClick={refresh} data-testid="browser-refresh-btn">
          <RotateCw size={16} className={loading ? 'animate-spin' : ''} />
        </button>
        <div style={{ position: 'relative', flex: 1, display: 'flex', alignItems: 'center' }}>
          <Globe size={14} style={{ position: 'absolute', left: 10, color: '#475569' }} />
          <input
            className="web-browser-url"
            style={{ paddingLeft: 30 }}
            value={inputUrl}
            onChange={(e) => setInputUrl(e.target.value)}
            onKeyDown={handleKeyDown}
            data-testid="browser-url-input"
          />
        </div>
        <button className="web-browser-nav-btn" onClick={() => navigate(inputUrl)} data-testid="browser-go-btn">
          <Search size={16} />
        </button>
      </div>
      <iframe
        ref={iframeRef}
        className="web-browser-frame"
        src={url}
        title="MineOS Browser"
        onLoad={() => setLoading(false)}
        sandbox="allow-same-origin allow-scripts allow-popups allow-forms"
        data-testid="browser-iframe"
      />
    </div>
  );
};
