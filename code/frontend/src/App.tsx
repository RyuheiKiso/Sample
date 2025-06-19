

// アプリケーションのエントリーポイント
import React from 'react';

import './App.css';
import Login from './features/login';


/**
 * アプリケーションのメインコンポーネント
 * 現状はログイン画面のみ表示
 */
function App() {
  return (
    <div className="App">
      {/* ログイン画面のみ表示 */}
      <Login />
    </div>
  );
}

export default App;
