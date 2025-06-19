/**
 * Footer component types
 */
export type FKey = `F${1|2|3|4|5|6|7|8|9|10|11|12}`;

export type FooterProps = {
  /**
   * Callback for F1-F12 key/button press. Example: { F1: () => {}, F2: () => {}, ... }
   */
  onFKeyPress?: Partial<Record<FKey, () => void>>;
  /**
   * Button names for F1-F12. Example: { F1: 'Login', F2: 'Cancel', ... }
   */
  buttonNames?: Partial<Record<FKey, string>>;
};
