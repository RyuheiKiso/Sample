

// アプリケーションのエントリーポイント
import React from 'react';


import './App.css';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Login from './features/login';
import Sandbox from './features/sandbox/Sandbox';



/**
 * アプリケーションのメインコンポーネント
 * ルーティング対応
 */
function App() {
  return (
    <div className="App">
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Login />} />
          <Route path="/sandbox" element={<Sandbox />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
