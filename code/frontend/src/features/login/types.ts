export interface User {
  id: number;
  username: string;
  displayName: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}
