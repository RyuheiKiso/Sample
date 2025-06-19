
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

import { LoginRequest } from "../../../proto/auth";
import { AuthServiceClient } from "../../../proto/auth.client";

// .envのREACT_APP_BACKEND_URLを参照
const backendUrl = process.env.REACT_APP_BACKEND_URL || "http://localhost:50051";

const transport = new GrpcWebFetchTransport({
  baseUrl: backendUrl, // サーバーのgRPCエンドポイント
});

const client = new AuthServiceClient(transport);

export async function login(username: string, password: string) {
  const req: LoginRequest = { username, password };
  const { response } = await client.login(req);
  return response;
}
