import { useState, useCallback } from 'react';

export function useQrReader() {
  const [qrResult, setQrResult] = useState<string | null>(null);
  const [qrError, setQrError] = useState<string | null>(null);

  const handleScan = useCallback(
    (result: string) => {
      setQrResult(result);
    },
    [setQrResult]
  );

  const handleError = useCallback(
    (error: string) => {
      setQrError(error);
    },
    [setQrError]
  );

  return {
    qrResult,
    qrError,
    handleScan,
    handleError,
    setQrResult,
    setQrError,
  };
}
