import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import LoginForm from "./LoginForm";

jest.mock("../api/loginApi", () => ({
  login: jest.fn(async (username: string, password: string) => ({
    token: "mock_token",
    user: { id: 1, username, displayName: "Mock" }
  }))
}));

describe("LoginForm", () => {
  it("ユーザー名とパスワードを入力してログインできる", async () => {
    render(<LoginForm />);
    fireEvent.change(screen.getByLabelText("ユーザー名"), { target: { value: "alice" } });
    fireEvent.change(screen.getByLabelText("パスワード"), { target: { value: "alicepw" } });
    fireEvent.click(screen.getByText("ログイン"));
    await waitFor(() => {
      expect(screen.getByText(/トークン:/)).toBeInTheDocument();
    });
  });
});
