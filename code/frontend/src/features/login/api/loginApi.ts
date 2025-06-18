import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { AuthServiceClient } from "../../../proto/auth.client";
import { LoginRequest } from "../../../proto/auth";

const transport = new GrpcWebFetchTransport({
  baseUrl: "http://localhost:50051", // サーバーのgRPCエンドポイント
});

const client = new AuthServiceClient(transport);

export async function login(username: string, password: string) {
  const req: LoginRequest = { username, password };
  const { response } = await client.login(req);
  return response;
}
