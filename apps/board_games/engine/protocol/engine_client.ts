import {
  CommandRequest as CommandRequestProto,
  CommandRequestSchema,
  CommandResponse as CommandResponseProto,
  EngineService,
  NewRoomRequest as NewRoomRequestProto,
  NewRoomRequestSchema,
  StartGameRequest as StartGameRequestProto,
  StartGameRequestSchema,
  GetOrCreateParticipantRequest as GetOrCreateParticipantRequestProto,
  GetOrCreateParticipantRequestSchema,
  Participant as ParticipantProto,
  SendGameActionRequest as SendGameActionRequestProto,
  SendGameActionRequestSchema,
} from './engine_pb.js';
import { Empty } from "@bufbuild/protobuf/wkt";
import grpc from '@grpc/grpc-js';
import { create } from "@bufbuild/protobuf";
import { createConnectRpcClient } from 'grpc-es-bridge/connectrpc';

const address = 'localhost:50051';
const credentials = grpc.credentials.createInsecure();
const client = createConnectRpcClient(EngineService, address, credentials)



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

export async function getOrCreateParticipant(room_uuid: string, participant_uuid: string, participant_role: string, player_number: number | undefined): Promise<ParticipantProto> {
  console.log(`[backend] getOrCreateParticipant(room_uuid=${room_uuid}, participant_uuid=${participant_uuid}, participant_role=${participant_role}, player_number=${player_number})`);
  const request: GetOrCreateParticipantRequestProto = create(GetOrCreateParticipantRequestSchema, { roomUuid: room_uuid, participantUuid: participant_uuid, participantRole: participant_role, playerNumber: player_number });
  return await client.getOrCreateParticipant(request);
}

export async function sendGameAction(room_uuid: string, participant_uuid: string, payload: Uint8Array): Promise<Empty> {
  console.log(`[backend] sendGameAction(room_uuid=${room_uuid}, participant_uuid=${participant_uuid}, payload)`);
  const request: SendGameActionRequestProto = create(SendGameActionRequestSchema, { roomUuid: room_uuid, participantUuid: participant_uuid, payload });
  return await client.sendGameAction(request);
}
