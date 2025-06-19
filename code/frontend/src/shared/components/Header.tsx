
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import React from 'react';




/**
 * ヘッダーコンポーネントのProps
 * @property title - 画面タイトル
 */
type HeaderProps = {
  title: string;
};


/**
 * ヘッダーコンポーネント
 * 画面タイトルとロゴを表示。開発環境では色を変更
 */
const Header: React.FC<HeaderProps> = ({ title }) => {
  // NODE_ENV で色を切り替え（開発時は赤、リリース時は青）
  const isDev = process.env.NODE_ENV !== 'production';
  return (
    <AppBar position="static" sx={{
      backgroundColor: isDev ? '#d32f2f' : '#1976d2',
      boxShadow: 'none',
    }}>
      <Toolbar>
        <Box sx={{ display: 'flex', alignItems: 'center', flexGrow: 1 }}>
          {/* ロゴ画像 */}
          <img src="/logo192.png" alt="Logo" style={{ height: 40, marginRight: 16 }} />
          {/* タイトル */}
          <Typography variant="h6" component="div" sx={{ fontWeight: 'bold' }}>
            {title}
          </Typography>
        </Box>
      </Toolbar>
    </AppBar>
  );
};

export default Header;
