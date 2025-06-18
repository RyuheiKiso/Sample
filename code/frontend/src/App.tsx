import React from 'react';
// import logo from './logo.svg';
import './App.css';
import Login from './features/login';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        {/* <img src={logo} className="App-logo" alt="logo" /> */}
        <h1>ログイン</h1>
        <Login />
      </header>
    </div>
  );
}

export default App;
