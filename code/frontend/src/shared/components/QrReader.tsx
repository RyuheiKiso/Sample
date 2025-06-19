import React, { useEffect, useRef, useId } from 'react';
import { Html5Qrcode } from 'html5-qrcode';

type QrReaderProps = {
  onScan: (result: string) => void;
  onError?: (error: string) => void;
  width?: number;
  height?: number;
};

const QrReader: React.FC<QrReaderProps> = ({ onScan, onError, width = 300, height = 300 }) => {
  const qrRef = useRef<HTMLDivElement>(null);
  const html5QrCodeRef = useRef<Html5Qrcode | null>(null);
  const uniqueId = useId();

  useEffect(() => {
    if (!qrRef.current) return;
    const id = `qr-reader-${uniqueId}`;
    qrRef.current.id = id;
    const qrCode = new Html5Qrcode(id);
    html5QrCodeRef.current = qrCode;
    qrCode
      .start(
        { facingMode: 'environment' },
        { fps: 10, qrbox: { width, height } },
        (decodedText) => {
          onScan(decodedText);
        },
        (err) => {
          if (onError) onError(err);
        }
      )
      .catch((err) => {
        if (onError) onError(err);
      });
    return () => {
      qrCode
        .stop()
        .catch(() => {})
        .finally(() => {
          qrCode.clear();
        });
    };
  }, [onScan, onError, width, height, uniqueId]);

  return <div ref={qrRef} style={{ width, height }} />;
};

export default QrReader;
