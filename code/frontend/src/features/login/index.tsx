

import React, { useRef } from 'react';

import LoginForm, { LoginFormHandle } from './components/LoginForm';
import Footer from '../../shared/components/Footer';
import Header from '../../shared/components/Header';


const Login: React.FC = () => {
  const formRef = useRef<LoginFormHandle>(null);
  return (
    <>
      <Header title="ログイン" />
      <LoginForm ref={formRef} />
      <Footer 
        onFKeyPress={{ F1: () => formRef.current?.submit() }}
        buttonNames={{ F1: 'ログイン' }}
      />
    </>
  );
};

export default Login;
export * from "./api";
export * from "./components";
export * from "./hooks";
export * from "./types";
