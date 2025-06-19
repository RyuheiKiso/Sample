

import React, { useRef } from 'react';

import LoginForm, { LoginFormHandle } from './components/LoginForm';
import Footer from '../../shared/components/Footer';
import Header from '../../shared/components/Header';



/**
 * ログイン画面コンポーネント
 * @returns ログイン画面
 */
const Login: React.FC = () => {
  // LoginFormのrefを管理
  const formRef = useRef<LoginFormHandle>(null);
  return (
    <>
      {/* ヘッダー */}
      <Header title="ログイン" />
      {/* ログインフォーム */}
      <LoginForm ref={formRef} />
      {/* フッター（F1キーでログイン実行） */}
      <Footer 
        onFKeyPress={{ F1: () => formRef.current?.submit() }}
        buttonNames={{ F1: 'ログイン' }}
      />
    </>
  );
};

export default Login;
// API, コンポーネント, フック, 型を再エクスポート
export * from "./api";
export * from "./components";
export * from "./hooks";
export * from "./types";
