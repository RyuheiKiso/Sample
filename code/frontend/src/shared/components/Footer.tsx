import React, { useEffect } from 'react';
import './Footer.css';


// F1～F12のキー配列
const F_KEYS = Array.from({ length: 12 }, (_, i) => `F${i + 1}`);


/**
 * フッターコンポーネントのProps
 * - onFKeyPress: F1～F12ボタン押下時のコールバック群
 * - buttonNames: F1～F12ボタンの割り当て名
 */
type FooterProps = {
  /**
   * F1～F12ボタン押下時のコールバック群。例: { F1: () => {}, F2: () => {}, ... }
   */
  onFKeyPress?: Partial<Record<`F${number}`, () => void>>;
  /**
   * F1～F12ボタンの割り当て名。例: { F1: 'ログイン', F2: 'キャンセル', ... }
   */
  buttonNames?: Partial<Record<`F${number}`, string>>;
};



/**
 * フッターコンポーネント
 * F1～F12キーやボタン押下でコールバックを実行
 */
const Footer: React.FC<FooterProps> = ({ onFKeyPress, buttonNames }) => {
  useEffect(() => {
    /**
     * F1～F12キー押下時の処理
     */
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key.startsWith('F') && !isNaN(Number(e.key.slice(1)))) {
        const idx = Number(e.key.slice(1));
        if (idx >= 1 && idx <= 12) {
          e.preventDefault(); // ブラウザのデフォルト動作を抑止
          document.getElementById(`footer-btn-${e.key}`)?.classList.add('active');
          setTimeout(() => {
            document.getElementById(`footer-btn-${e.key}`)?.classList.remove('active');
          }, 150);
          // コールバック呼び出し
          onFKeyPress?.[e.key as keyof typeof onFKeyPress]?.();
        }
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [onFKeyPress]);

  /**
   * ボタンクリック時の処理
   * @param key F1～F12
   */
  const handleClick = (key: string) => {
    onFKeyPress?.[key as keyof typeof onFKeyPress]?.();
  };

  return (
    <footer className="footer">
      {/* F1～F12ボタンを動的に生成 */}
      {F_KEYS.map((key) => {
        const name = buttonNames?.[key as keyof typeof buttonNames];
        if (!name) return null; // 割り当て名がない場合はボタン自体を表示しない
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
