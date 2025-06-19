import { useState, useCallback } from 'react';

export function useQrReader() {
  const [qrResult, setQrResult] = useState<string | null>(null);
  const [qrError, setQrError] = useState<string | null>(null);

  const handleScan = useCallback((result: string) => {
    setQrResult(result);
  }, []);

  const handleError = useCallback((error: string) => {
    setQrError(error);
  }, []);

  return {
    qrResult,
    qrError,
    handleScan,
    handleError,
    setQrResult,
    setQrError,
  };
}
