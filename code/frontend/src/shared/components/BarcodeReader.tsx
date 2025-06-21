
import React, { useRef, useEffect, useState } from 'react';
import { BrowserMultiFormatReader } from '@zxing/browser';
import type { BarcodeReaderResult, BarcodeReaderProps } from '../types/barcode-reader';

export type { BarcodeReaderResult };

/**
 * BarcodeReader
 * シンプルなバーコードリーダー（カメラ映像からバーコード/QRコードを検出）
 * @param onDetected 検出時コールバック
 * @param width, height, facingMode カメラ設定
 */
const BarcodeReader: React.FC<BarcodeReaderProps> = ({
  onDetected,
  width = 320,
  height = 240,
  facingMode = 'environment',
}) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let reader: BrowserMultiFormatReader | null = null;
    let active = true;
    let stream: MediaStream | null = null;

    const constraints = {
      video: { width, height, facingMode },
    };
    navigator.mediaDevices.getUserMedia(constraints)
      .then((s) => {
        stream = s;
        if (videoRef.current) {
          videoRef.current.srcObject = stream;
        }
        reader = new BrowserMultiFormatReader();
        const decodePromise = reader.decodeFromVideoElement(videoRef.current!, (result, err) => {
          if (!active) return;
          if (result) {
            onDetected({ text: result.getText(), format: String(result.getBarcodeFormat()) });
          }
        });
        // decodePromiseはキャンセルできないが、activeフラグで検出コールバックを抑制
      })
      .catch((err) => {
        setError('カメラの取得に失敗しました: ' + err.message);
      });
    return () => {
      active = false;
      // decodeFromVideoElementの監視はactiveフラグで抑制するため、明示的なstopは不要
      if (stream) {
        stream.getTracks().forEach((track) => track.stop());
      }
    };
  }, [width, height, facingMode, onDetected]);



  return (
    <div>
      <video ref={videoRef} width={width} height={height} autoPlay muted playsInline style={{ border: '1px solid #ccc' }} />
      {error && <div style={{ color: 'red' }}>{error}</div>}
      <div style={{ fontSize: '0.8em', color: '#888' }}>
        Powered by @zxing/browser
      </div>
    </div>
  );
};

export default BarcodeReader;
