

import { Box, Button, TextField, Paper, Alert, Snackbar } from "@mui/material";
import React, { useState, useImperativeHandle, forwardRef, useEffect } from "react";
import { useLoading } from '../../../shared/components/LoadingContext';
import { login } from "../api/loginApi";
import type { LoginFormHandle } from './login-form.types';

export type { LoginFormHandle };

/**
 * ログインフォームコンポーネント
 * @param props - なし
 * @param ref - 外部からsubmitを呼び出すためのref
 */
const LoginForm = forwardRef<LoginFormHandle>((props, ref) => {
  // ユーザー名の状態
  const [username, setUsername] = useState("");
  // パスワードの状態
  const [password, setPassword] = useState("");
  // ログイントークン（成功時のみセット）
  const [token, setToken] = useState<string | null>(null);
  // エラーメッセージ（失敗時のみセット）
  const [error, setError] = useState<string | null>(null);
  // ローディング状態管理（グローバル）
  const { setLoading } = useLoading();
  // エラー用Snackbarの開閉状態
  const [errorOpen, setErrorOpen] = useState(false);

  /**
   * フォーム送信時の処理
   * @param e フォームイベント
   */
  const handleSubmit = React.useCallback(async (e?: React.FormEvent) => {
    if (e) e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      // 認証API呼び出し
      const resp = await login(username, password);
      // 成功時はトークンをセット
      setToken(resp.token);
    } catch (err: any) {
      // gRPCエラーの内容で分岐
      let message = "予期しないエラーが発生しました。";
      // ネットワークエラー判定関数
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
      // エラーメッセージをセット
      setError(message);
      setErrorOpen(true);
    } finally {
      setLoading(false);
    }
  }, [username, password, setLoading]);

  // エラーがセットされたらSnackbarを開く
  useEffect(() => {
    if (error) setErrorOpen(true);
  }, [error]);

  // 外部からsubmitを呼び出せるようにする
  useImperativeHandle(ref, () => ({
    submit: () => handleSubmit(),
  }), [handleSubmit]);

  return (
    <Box display="flex" justifyContent="center" alignItems="center" minHeight="80vh">
      <Paper elevation={6} sx={{ p: 4, minWidth: 350 }}>
        {/* ログインフォーム */}
        <Box component="form" onSubmit={handleSubmit}>
          {/* ユーザー名入力欄 */}
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
          {/* パスワード入力欄 */}
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
          {/* ログインボタン */}
          <Button
            type="submit"
            variant="contained"
            color="primary"
            fullWidth
            sx={{ mt: 2 }}
          >
            ログイン
          </Button>
          {/* 成功時のトークン表示 */}
          {token && (
            <Alert severity="success" sx={{ mt: 2 }}>
              {/* 認証成功時のトークン表示 */}
              トークン: {token}
            </Alert>
          )}
          {/* エラー通知（Snackbar+Alert） */}
          <Snackbar open={errorOpen} autoHideDuration={6000} onClose={() => setErrorOpen(false)} anchorOrigin={{ vertical: 'top', horizontal: 'center' }}>
            <Alert onClose={() => setErrorOpen(false)} severity="error" sx={{ width: '100%' }}>
              {error ? decodeURIComponent(error) : ''}
            </Alert>
          </Snackbar>
        </Box>
      </Paper>
    </Box>
  );
});

export default LoginForm;
