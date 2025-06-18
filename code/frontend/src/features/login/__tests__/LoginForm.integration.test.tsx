import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import LoginForm from "../components/LoginForm";

jest.mock("../api/loginApi", () => ({
  login: jest.fn(async (username: string, password: string) => ({
    token: "integration_token",
    user: { id: 1, username, displayName: "Integration" }
  }))
}));

describe("LoginForm Integration", () => {
  it("正しい入力でトークンが表示される", async () => {
    render(<LoginForm />);
    fireEvent.change(screen.getByLabelText("ユーザー名"), { target: { value: "alice" } });
    fireEvent.change(screen.getByLabelText("パスワード"), { target: { value: "alicepw" } });
    fireEvent.click(screen.getByText("ログイン"));
    await waitFor(() => {
      expect(screen.getByText(/トークン:/)).toBeInTheDocument();
    });
  });
});
