/**
 * QrReader component types
 */
export type QrReaderProps = {
  onScan: (result: string) => void;
  onError?: (error: string) => void;
  width?: number;
  height?: number;
};
