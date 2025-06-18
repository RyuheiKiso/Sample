import React, { useState } from "react";
import { login } from "../api/loginApi";

const LoginForm: React.FC = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [token, setToken] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    try {
      const resp = await login(username, password);
      setToken(resp.token);
    } catch (err: any) {
      setError("ログイン失敗");
    }
  };

  return (
    <form onSubmit={handleSubmit}>
      <div>
        <label>ユーザー名</label>
        <input
          value={username}
          onChange={e => setUsername(e.target.value)}
          name="username"
        />
      </div>
      <div>
        <label>パスワード</label>
        <input
          type="password"
          value={password}
          onChange={e => setPassword(e.target.value)}
          name="password"
        />
      </div>
      <button type="submit">ログイン</button>
      {token && <div>トークン: {token}</div>}
      {error && <div style={{ color: "red" }}>{error}</div>}
    </form>
  );
};

export default LoginForm;
