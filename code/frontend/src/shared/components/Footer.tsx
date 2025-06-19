import React, { useEffect, useCallback } from 'react';
import './Footer.css';
import type { FooterProps, FKey } from '../types/footer';

// F1-F12 key array
const F_KEYS: FKey[] = Array.from({ length: 12 }, (_, i) => `F${i + 1}` as FKey);

/**
 * Footer component
 * Handles F1-F12 key/button press and executes callbacks
 */
const Footer: React.FC<FooterProps> = ({ onFKeyPress, buttonNames }) => {
  useEffect(() => {
    /**
     * Handle F1-F12 keydown event
     */
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key.startsWith('F') && !isNaN(Number(e.key.slice(1)))) {
        const idx = Number(e.key.slice(1));
        if (idx >= 1 && idx <= 12) {
          e.preventDefault(); // Prevent browser default
          document.getElementById(`footer-btn-${e.key}`)?.classList.add('active');
          setTimeout(() => {
            document.getElementById(`footer-btn-${e.key}`)?.classList.remove('active');
          }, 150);
          onFKeyPress?.[e.key as FKey]?.();
        }
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onFKeyPress]);

  /**
   * Handle button click
   */
  const handleClick = useCallback((key: FKey) => {
    onFKeyPress?.[key]?.();
  }, [onFKeyPress]);

  return (
    <footer className="footer">
      {/* Render F1-F12 buttons dynamically */}
      {F_KEYS.map((key) => {
        const name = buttonNames?.[key];
        if (!name) return null; // Hide button if no name assigned
        return (
          <button
            key={key}
            id={`footer-btn-${key}`}
            className="footer-btn"
            onClick={() => handleClick(key)}
          >
            <span style={{ display: 'block', fontSize: '0.7em', fontWeight: 'normal' }}>{key}</span>
            <span style={{ display: 'block', fontSize: '0.7em', fontWeight: 'lighter' }}>{name}</span>
          </button>
        );
      })}
    </footer>
  );
};

export default Footer;
