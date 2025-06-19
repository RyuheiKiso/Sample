import React from 'react';
import ReactDOM from 'react-dom/client';

import './index.css';
import App from './App';
import reportWebVitals from './reportWebVitals';
import LoadingBackdrop from './shared/components/LoadingBackdrop';
import { LoadingProvider } from './shared/components/LoadingContext';
import { useLoading } from './shared/components/LoadingContext';

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);
function AppWithLoading() {
  const { loading } = useLoading();
  return <>
    <App />
    <LoadingBackdrop open={loading} message="通信中..." />
  </>;
}

root.render(
  <React.StrictMode>
    <LoadingProvider>
      <AppWithLoading />
    </LoadingProvider>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
