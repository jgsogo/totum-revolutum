import { CommandRequest as CommandRequestProto, CommandRequestSchema, CommandResponse as CommandResponseProto, EngineService } from './engine_pb.js';
import grpc from '@grpc/grpc-js';
import { create } from "@bufbuild/protobuf";
import { createConnectRpcClient } from 'grpc-es-bridge/connectrpc';

const address = 'localhost:50051';
const credentials = grpc.credentials.createInsecure();
const client = createConnectRpcClient(EngineService, address, credentials)


// export const engineClient = new EngineService(
//   'localhost:50051',  // FIXME:
//   grpc.credentials.createInsecure()
// );

export async function sendCommandToEngine(game_id: string, player_id: string, payload: Uint8Array): Promise<CommandResponseProto> {
  const request: CommandRequestProto = create(CommandRequestSchema, { gameId: game_id, playerId: player_id, payload });
  console.log(`[backend] Submit request using gRPC client`);
  return await client.submitCommand(request);
  // return new Promise((resolve, reject) => {
  //   // const request_binary = toBinary(CommandRequestSchema, request);

  //   client.submitCommand(request, (err: grpc.ServiceError | null, response?: CommandResponseProto) => {
  //     if (err) return reject(err);
  //     console.log('Response:', response);
  //     resolve();
  //   })
  // });
}
