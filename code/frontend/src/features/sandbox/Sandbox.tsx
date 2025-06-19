import React from 'react';

// サンドボックス用に共通コンポーネントをimport

import Header from '../../shared/components/Header';
import Footer from '../../shared/components/Footer';

import QrReader from '../../shared/components/QrReader';
import { useQrReader } from '../../shared/hooks/useQrReader';


const Sandbox: React.FC = () => {
  // QrReaderのスキャン結果・エラー管理
  const { qrResult, qrError, handleScan, handleError } = useQrReader();

  // FooterのFキー名とコールバック例
  const buttonNames = { F1: 'テスト', F2: 'キャンセル' };
  const onFKeyPress = {
    F1: () => alert('F1が押されました'),
    F2: () => alert('F2が押されました'),
  };

  return (
    <div style={{ padding: 24 }}>
      <h1>Sandbox Page</h1>
      <section style={{ marginBottom: 24 }}>
        <h2>Header</h2>
        <Header title="サンドボックス" />
      </section>
      <section style={{ marginBottom: 24 }}>
        <h2>Footer</h2>
        <Footer buttonNames={buttonNames} onFKeyPress={onFKeyPress} />
      </section>
      <section style={{ marginBottom: 24 }}>
        <h2>QrReader</h2>
        <QrReader onScan={handleScan} onError={handleError} width={300} height={300} />
        <div style={{ marginTop: 8 }}>スキャン結果: {qrResult ?? ''}</div>
        {qrError && (
          <div style={{ color: 'red', marginTop: 8 }}>エラー: {qrError}</div>
        )}
      </section>
      {/* 必要に応じて他のshared/componentsもここでテスト可能 */}
    </div>
  );
};

export default Sandbox;
