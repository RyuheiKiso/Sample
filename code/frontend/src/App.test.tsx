import { render, screen } from '@testing-library/react';
import React from 'react';

import App from './App';

describe('App', () => {
  it('ログイン画面が表示される', () => {
    render(<App />);
    expect(screen.getByText(/ログイン/)).toBeInTheDocument();
  });
});