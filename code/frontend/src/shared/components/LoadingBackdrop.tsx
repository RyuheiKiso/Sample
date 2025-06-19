import React from 'react';
import Backdrop from '@mui/material/Backdrop';
import CircularProgress from '@mui/material/CircularProgress';

export type LoadingBackdropProps = {
  open: boolean;
  message?: string;
};

const LoadingBackdrop: React.FC<LoadingBackdropProps> = ({ open, message }) => (
  <Backdrop sx={{ color: '#fff', zIndex: (theme) => theme.zIndex.drawer + 1 }} open={open}>
    <CircularProgress color="inherit" />
    {message && <span style={{ marginLeft: 16, fontSize: 18 }}>{message}</span>}
  </Backdrop>
);

export default LoadingBackdrop;
