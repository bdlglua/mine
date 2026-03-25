import React, { useState, useRef, useEffect, useCallback } from 'react';

const COMMANDS = {
  help: () => `Available commands:
  help      - Show this help message
  clear     - Clear the terminal
  ls        - List files in current directory
  pwd       - Print working directory
  echo      - Echo text
  date      - Show current date/time
  whoami    - Show current user
  uname     - Show system information
  cat       - Display file contents
  mkdir     - Create directory
  touch     - Create file
  neofetch  - System information
  history   - Show command history
  calc      - Simple calculator (e.g. calc 2+2)`,
  pwd: () => '/home/user',
  whoami: () => 'user@mineos',
  uname: () => 'MineOS 1.0.0 x86_64 WebKernel',
  date: () => new Date().toString(),
  neofetch: () => `
       ___  ___  _                 ___  _____ 
       |  \\/  | (_)               /   |/  ___|
       | .  . |  _  _ __    ___  / /| |\\ \`--. 
       | |\\/| | | || '_ \\  / _ \\/ /_| | \`--. \\
       | |  | | | || | | ||  __/\\___  |/\\__/ /
       \\_|  |_/ |_||_| |_| \\___|    |_/\\____/ 

  OS: MineOS 1.0.0
  Kernel: WebKernel 6.1
  Shell: mine-sh 1.0
  Resolution: ${window.innerWidth}x${window.innerHeight}
  Terminal: MineOS Terminal
  CPU: WebAssembly vCPU
  Memory: ${Math.round(performance?.memory?.usedJSHeapSize / 1024 / 1024 || 128)} MiB / ${Math.round(performance?.memory?.totalJSHeapSize / 1024 / 1024 || 512)} MiB`,
};

export const Terminal = () => {
  const [lines, setLines] = useState([
    { type: 'output', text: 'MineOS Terminal v1.0.0' },
    { type: 'output', text: 'Type "help" for available commands.\n' },
  ]);
  const [input, setInput] = useState('');
  const [history, setHistory] = useState([]);
  const [historyIdx, setHistoryIdx] = useState(-1);
  const containerRef = useRef(null);
  const inputRef = useRef(null);

  useEffect(() => {
    if (containerRef.current) {
      containerRef.current.scrollTop = containerRef.current.scrollHeight;
    }
  }, [lines]);

  const focusInput = () => inputRef.current?.focus();

  const executeCommand = useCallback((cmd) => {
    const trimmed = cmd.trim();
    const parts = trimmed.split(' ');
    const command = parts[0].toLowerCase();
    const args = parts.slice(1).join(' ');

    let output = '';
    let isError = false;

    if (!trimmed) return;

    if (command === 'clear') {
      setLines([]);
      return;
    }

    if (command === 'echo') {
      output = args || '';
    } else if (command === 'ls') {
      output = 'Documents  Pictures  Music  Downloads  readme.txt';
    } else if (command === 'history') {
      output = history.map((h, i) => `  ${i + 1}  ${h}`).join('\n');
    } else if (command === 'calc') {
      try {
        const expr = args.replace(/[^0-9+\-*/.() ]/g, '');
        output = String(eval(expr));
      } catch {
        output = 'Error: Invalid expression';
        isError = true;
      }
    } else if (command === 'cat') {
      if (!args) { output = 'cat: missing operand'; isError = true; }
      else if (args === 'readme.txt') { output = 'Welcome to MineOS!\nThis is your personal operating system.'; }
      else { output = `cat: ${args}: No such file or directory`; isError = true; }
    } else if (command === 'mkdir' || command === 'touch') {
      output = args ? `${command}: created '${args}'` : `${command}: missing operand`;
      if (!args) isError = true;
    } else if (COMMANDS[command]) {
      output = COMMANDS[command]();
    } else {
      output = `mine-sh: ${command}: command not found`;
      isError = true;
    }

    setLines(prev => [
      ...prev,
      { type: 'input', text: trimmed },
      { type: isError ? 'error' : 'output', text: output },
    ]);
  }, [history]);

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      const cmd = input;
      setHistory(prev => [...prev, cmd]);
      setHistoryIdx(-1);
      executeCommand(cmd);
      setInput('');
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (history.length > 0) {
        const idx = historyIdx === -1 ? history.length - 1 : Math.max(0, historyIdx - 1);
        setHistoryIdx(idx);
        setInput(history[idx]);
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (historyIdx !== -1) {
        const idx = historyIdx + 1;
        if (idx >= history.length) { setHistoryIdx(-1); setInput(''); }
        else { setHistoryIdx(idx); setInput(history[idx]); }
      }
    }
  };

  return (
    <div className="terminal-container" ref={containerRef} onClick={focusInput} data-testid="terminal-app">
      {lines.map((line, i) => (
        <div key={i}>
          {line.type === 'input' ? (
            <div className="terminal-line">
              <span className="terminal-prompt">user@mineos:~$ </span>
              <span style={{ color: '#F8FAFC' }}>{line.text}</span>
            </div>
          ) : (
            <div className={line.type === 'error' ? 'terminal-error' : 'terminal-output'}>
              {line.text}
            </div>
          )}
        </div>
      ))}
      <div className="terminal-line">
        <span className="terminal-prompt">user@mineos:~$ </span>
        <input
          ref={inputRef}
          className="terminal-input"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={handleKeyDown}
          autoFocus
          spellCheck={false}
          data-testid="terminal-input"
        />
      </div>
    </div>
  );
};
