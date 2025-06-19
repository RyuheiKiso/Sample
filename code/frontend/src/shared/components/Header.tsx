
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import React from 'react';
import type { HeaderProps } from '../types/header';

/**
 * Header component
 * Displays screen title and logo. Changes color in development environment.
 */
const Header: React.FC<HeaderProps> = ({ title }) => {
  const isDev = process.env.NODE_ENV !== 'production';
  return (
    <AppBar position="static" sx={{
      backgroundColor: isDev ? '#d32f2f' : '#1976d2',
      boxShadow: 'none',
    }}>
      <Toolbar>
        <Box sx={{ display: 'flex', alignItems: 'center', flexGrow: 1 }}>
          {/* Logo image */}
          <img src="/logo192.png" alt="Logo" style={{ height: 40, marginRight: 16 }} />
          {/* Title */}
          <Typography variant="h6" component="div" sx={{ fontWeight: 'bold' }}>
            {title}
          </Typography>
        </Box>
      </Toolbar>
    </AppBar>
  );
};

export default Header;
