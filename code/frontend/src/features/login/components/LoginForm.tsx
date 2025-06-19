
import React, { useState, useImperativeHandle, forwardRef } from "react";
import { login } from "../api/loginApi";
import { Box, Button, TextField, Typography, Paper, Alert } from "@mui/material";



export type LoginFormHandle = {
  submit: () => void;
};

const LoginForm = forwardRef<LoginFormHandle>((props, ref) => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [token, setToken] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = React.useCallback(async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    setError(null);
    try {
      const resp = await login(username, password);
      setToken(resp.token);
    } catch (err: any) {
      // gRPCエラーの内容で分岐
      let message = "予期しないエラーが発生しました。";
      const isNetworkError = (msg: string) =>
        msg.includes("Failed to fetch") || msg.includes("Load failed");
      if (err && typeof err === "object") {
        // gRPCのunauthenticated
        if (err.code === 16 || (err.message && err.message.includes("認証失敗"))) {
          message = "ユーザー名またはパスワードが正しくありません。再度ご確認のうえ、もう一度お試しください。";
        } else if (err.code === 14) {
          message = "ネットワークに接続できません。通信環境をご確認ください。";
        } else if (err.code && err.code >= 10 && err.code < 20) {
          message = "サーバーで問題が発生しました。しばらくしてから再度お試しください。";
        } else if (err.message) {
          if (typeof err.message === "string" && isNetworkError(err.message)) {
            message = "サーバーに接続できません。ネットワーク環境やサーバーの状態をご確認ください。";
          } else {
            message = err.message;
          }
        }
      } else if (typeof err === "string" && isNetworkError(err)) {
        message = "サーバーに接続できません。ネットワーク環境やサーバーの状態をご確認ください。";
      }
      setError(message);
    }
  }, [username, password]);

  useImperativeHandle(ref, () => ({
    submit: () => handleSubmit(),
  }), [handleSubmit]);

  return (
    <Box display="flex" justifyContent="center" alignItems="center" minHeight="80vh">
      <Paper elevation={6} sx={{ p: 4, minWidth: 350 }}>
        <Box component="form" onSubmit={handleSubmit}>
          <TextField
            label="ユーザー名"
            variant="outlined"
            fullWidth
            margin="normal"
            value={username}
            onChange={e => setUsername(e.target.value)}
            name="username"
            autoComplete="username"
          />
          <TextField
            label="パスワード"
            variant="outlined"
            fullWidth
            margin="normal"
            type="password"
            value={password}
            onChange={e => setPassword(e.target.value)}
            name="password"
            autoComplete="current-password"
          />
          <Button
            type="submit"
            variant="contained"
            color="primary"
            fullWidth
            sx={{ mt: 2 }}
          >
            ログイン
          </Button>
          {token && (
            <Alert severity="success" sx={{ mt: 2 }}>
              トークン: {token}
            </Alert>
          )}
          {error && (
            <Alert severity="error" sx={{ mt: 2 }}>
              {error}
            </Alert>
          )}
        </Box>
      </Paper>
    </Box>

  );
});

export default LoginForm;
