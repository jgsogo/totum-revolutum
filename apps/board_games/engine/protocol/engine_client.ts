import {
  CommandRequest as CommandRequestProto,
  CommandRequestSchema,
  CommandResponse as CommandResponseProto,
  EngineService,
  NewRoomRequest as NewRoomRequestProto,
  NewRoomRequestSchema,
  StartGameRequest as StartGameRequestProto,
  StartGameRequestSchema,
  AddParticipantRequest as AddParticipantRequestProto,
  AddParticipantRequestSchema
} from './engine_pb.js';
import { Empty } from "@bufbuild/protobuf/wkt";
import grpc from '@grpc/grpc-js';
import { create } from "@bufbuild/protobuf";
import { createConnectRpcClient } from 'grpc-es-bridge/connectrpc';

const address = 'localhost:50051';
const credentials = grpc.credentials.createInsecure();
const client = createConnectRpcClient(EngineService, address, credentials)


export async function sendCommandToEngine(game_id: string, player_id: string, payload: Uint8Array): Promise<CommandResponseProto> {
  console.log(`[backend] Submit request using gRPC client`);
  const request: CommandRequestProto = create(CommandRequestSchema, { gameId: game_id, playerId: player_id, payload });
  return await client.submitCommand(request);
}

export async function createNewRoom(uuid: string, name: string): Promise<Empty> {
  console.log(`[backend] createNewRoom(uuid=${uuid}, name=${name})`);
  const request: NewRoomRequestProto = create(NewRoomRequestSchema, { uuid, name });
  return await client.createNewRoom(request);
}

export async function startGame(room_uuid: string, game_type: string): Promise<Empty> {
  console.log(`[backend] startGame(room_uuid=${room_uuid}, game_type=${game_type})`);
  const request: StartGameRequestProto = create(StartGameRequestSchema, { roomUuid: room_uuid, gameType: game_type });
  return await client.startGame(request);
}

export async function addParticipant(room_uuid: string, participant_uuid: string, participant_role: string): Promise<Empty> {
  console.log(`[backend] addParticipant(room_uuid=${room_uuid}, participant_uuid=${participant_uuid}, participant_role=${participant_role})`);
  const request: AddParticipantRequestProto = create(AddParticipantRequestSchema, { roomUuid: room_uuid, participantUuid: participant_uuid, participantRole: participant_role });
  return await client.addParticipant(request);
}
