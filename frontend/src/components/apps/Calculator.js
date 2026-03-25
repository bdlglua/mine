import React, { useState } from 'react';

export const Calculator = () => {
  const [display, setDisplay] = useState('0');
  const [expression, setExpression] = useState('');
  const [newNumber, setNewNumber] = useState(true);

  const handleNumber = (num) => {
    if (newNumber) {
      setDisplay(num);
      setNewNumber(false);
    } else {
      setDisplay(prev => prev === '0' ? num : prev + num);
    }
  };

  const handleOperator = (op) => {
    setExpression(prev => prev + display + ' ' + op + ' ');
    setNewNumber(true);
  };

  const handleEquals = () => {
    try {
      const fullExpr = expression + display;
      const sanitized = fullExpr.replace(/[^0-9+\-*/.() ]/g, '');
      const result = eval(sanitized);
      const formatted = Number.isInteger(result) ? String(result) : result.toFixed(8).replace(/\.?0+$/, '');
      setDisplay(formatted);
      setExpression('');
      setNewNumber(true);
    } catch {
      setDisplay('Error');
      setExpression('');
      setNewNumber(true);
    }
  };

  const handleClear = () => {
    setDisplay('0');
    setExpression('');
    setNewNumber(true);
  };

  const handleBackspace = () => {
    if (display.length > 1) {
      setDisplay(prev => prev.slice(0, -1));
    } else {
      setDisplay('0');
      setNewNumber(true);
    }
  };

  const handlePercent = () => {
    setDisplay(prev => String(parseFloat(prev) / 100));
    setNewNumber(true);
  };

  const handleSign = () => {
    setDisplay(prev => prev.startsWith('-') ? prev.slice(1) : '-' + prev);
  };

  const buttons = [
    { label: 'C', action: handleClear, cls: 'danger' },
    { label: '+/-', action: handleSign, cls: '' },
    { label: '%', action: handlePercent, cls: '' },
    { label: '/', action: () => handleOperator('/'), cls: 'operator' },
    { label: '7', action: () => handleNumber('7') },
    { label: '8', action: () => handleNumber('8') },
    { label: '9', action: () => handleNumber('9') },
    { label: '*', action: () => handleOperator('*'), cls: 'operator' },
    { label: '4', action: () => handleNumber('4') },
    { label: '5', action: () => handleNumber('5') },
    { label: '6', action: () => handleNumber('6') },
    { label: '-', action: () => handleOperator('-'), cls: 'operator' },
    { label: '1', action: () => handleNumber('1') },
    { label: '2', action: () => handleNumber('2') },
    { label: '3', action: () => handleNumber('3') },
    { label: '+', action: () => handleOperator('+'), cls: 'operator' },
    { label: '0', action: () => handleNumber('0') },
    { label: '.', action: () => handleNumber('.') },
    { label: 'DEL', action: handleBackspace, cls: '' },
    { label: '=', action: handleEquals, cls: 'equals' },
  ];

  return (
    <div data-testid="calculator-app" style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      <div className="calc-display">
        <div className="calc-display-expr">{expression || '\u00A0'}</div>
        <div className="calc-display-result" data-testid="calc-display">{display}</div>
      </div>
      <div className="calc-grid" style={{ flex: 1 }}>
        {buttons.map((btn) => (
          <button
            key={btn.label}
            className={`calc-btn ${btn.cls || ''}`}
            onClick={btn.action}
            data-testid={`calc-btn-${btn.label}`}
          >
            {btn.label === '*' ? '\u00D7' : btn.label === '/' ? '\u00F7' : btn.label}
          </button>
        ))}
      </div>
    </div>
  );
};
