import { useState } from "react";

import { login } from "../api/loginApi";

export function useLogin() {
  const [loading, setLoading] = useState(false);
  const [token, setToken] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const doLogin = async (username: string, password: string) => {
    setLoading(true);
    setError(null);
    try {
      const resp = await login(username, password);
      setToken(resp.token);
      setLoading(false);
      return resp;
    } catch (e: any) {
      setError("ログイン失敗");
      setLoading(false);
      throw e;
    }
  };

  // ログイン状態・エラー状態・ローディング状態・ログイン実行関数を返却
  return { loading, token, error, doLogin };
}
