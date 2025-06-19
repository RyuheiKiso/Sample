
import React from 'react';

import './App.css';
import Login from './features/login';

function App() {
  return (
    <div className="App">
      {/* ログイン画面のみ表示 */}
      <Login />
    </div>
  );
}

export default App;
