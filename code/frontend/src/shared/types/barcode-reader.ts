/**
 * BarcodeReader component types
 */
export type BarcodeReaderResult = {
  text: string;
  format: string;
};

export type BarcodeReaderProps = {
  onDetected: (result: BarcodeReaderResult) => void;
  width?: number;
  height?: number;
  facingMode?: 'user' | 'environment';
};
