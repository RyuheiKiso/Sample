import React, { useState, useRef } from 'react';

// サンドボックス用に共通コンポーネントをimport

import Header from '../../shared/components/Header';
import Footer from '../../shared/components/Footer';


import QrReader from '../../shared/components/QrReader';
import { useQrReader } from '../../shared/hooks/useQrReader';
import BarcodeReader, { BarcodeReaderResult } from '../../shared/components/BarcodeReader';

// CountdownPanelを追加
import CountdownPanel, { CountdownPanelHandle } from '../../shared/components/CountdownPanel';

import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';
import InputLabel from '@mui/material/InputLabel';
import FormControl from '@mui/material/FormControl';

const componentOptions = [
  { label: 'Header', value: 'Header' },
  { label: 'Footer', value: 'Footer' },
  { label: 'QrReader', value: 'QrReader' },
  { label: 'BarcodeReader', value: 'BarcodeReader' },
  { label: 'CountdownPanel', value: 'CountdownPanel' }, // 追加
];

const Sandbox: React.FC = () => {
  // QrReaderのスキャン結果・エラー管理
  const { qrResult, qrError, handleScan, handleError } = useQrReader();

  // FooterのFキー名とコールバック例
  const buttonNames = { F1: 'テスト', F2: 'キャンセル' };
  const onFKeyPress = {
    F1: () => alert('F1が押されました'),
    F2: () => alert('F2が押されました'),
  };


  // BarcodeReaderの検出結果
  const [barcodeResult, setBarcodeResult] = useState<BarcodeReaderResult | null>(null);
  // ドロップダウンで選択されたコンポーネント名
  const [selectedComponent, setSelectedComponent] = useState('Header');

  // CountdownPanel用
  const countdownRef = useRef<CountdownPanelHandle>(null);

  return (
    <div style={{ padding: 24 }}>
      <h1>Sandbox Page</h1>
      <div style={{ marginBottom: 24 }}>
        <FormControl variant="outlined" size="small" style={{ minWidth: 200 }}>
          <InputLabel id="component-select-label">表示コンポーネント</InputLabel>
          <Select
            labelId="component-select-label"
            value={selectedComponent}
            label="表示コンポーネント"
            onChange={e => setSelectedComponent(e.target.value as string)}
          >
            {componentOptions.map(opt => (
              <MenuItem key={opt.value} value={opt.value}>{opt.label}</MenuItem>
            ))}
          </Select>
        </FormControl>
      </div>
      {selectedComponent === 'Header' && (
        <section style={{ marginBottom: 24 }}>
          <h2>Header</h2>
          <Header title="サンドボックス" />
        </section>
      )}
      {selectedComponent === 'Footer' && (
        <section style={{ marginBottom: 24 }}>
          <h2>Footer</h2>
          <Footer buttonNames={buttonNames} onFKeyPress={onFKeyPress} />
        </section>
      )}
      {selectedComponent === 'QrReader' && (
        <section style={{ marginBottom: 24 }}>
          <h2>QrReader</h2>
          <QrReader onScan={handleScan} onError={handleError} width={300} height={300} />
          <div style={{ marginTop: 8 }}>スキャン結果: {qrResult ?? ''}</div>
          {qrError && (
            <div style={{ color: 'red', marginTop: 8 }}>エラー: {qrError}</div>
          )}
        </section>
      )}
      {selectedComponent === 'BarcodeReader' && (
        <section style={{ marginBottom: 24 }}>
          <h2>BarcodeReader</h2>
          <BarcodeReader onDetected={setBarcodeResult} width={300} height={200} />
          <div style={{ marginTop: 8 }}>
            検出結果: {barcodeResult ? `${barcodeResult.text} (${barcodeResult.format})` : ''}
          </div>
        </section>
      )}
      {selectedComponent === 'CountdownPanel' && (
        <section style={{ marginBottom: 24 }}>
          <h2>CountdownPanel</h2>
          <CountdownPanel
            ref={countdownRef}
            limitSeconds={90}
            warnSeconds={10}
            warnColor="red"
            height={120}
            width={320}
            fontSize={32}
          />
          <div style={{ marginTop: 12 }}>
            <button onClick={() => countdownRef.current?.start()}>開始</button>
            <button onClick={() => countdownRef.current?.stop()} style={{ marginLeft: 8 }}>停止</button>
            <button onClick={() => countdownRef.current?.reset()} style={{ marginLeft: 8 }}>リセット</button>
          </div>
          <div style={{ marginTop: 12 }}>
            <span>※画面幅で自動的にPC/スマホ表示が切り替わります。</span>
          </div>
        </section>
      )}
      {/* 必要に応じて他のshared/componentsもここでテスト可能 */}
    </div>
  );
};

export default Sandbox;