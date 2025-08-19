import { sendCommandToEngine } from '../../../../../../engine/protocol/engine_client';
import { json } from '@sveltejs/kit';
import type { CommandResponse } from '../../../../../../engine/protocol/engine_pb';

export async function POST({ request, params }) {
  const { roomId } = params;
  const data = new Uint8Array(await request.arrayBuffer());

  try {
    console.log(`[backend] POST request: ${data}`);
    const response: CommandResponse = await sendCommandToEngine("game_id", "player_id", data);
    console.log(`[backend] Got something from grpc: ${JSON.stringify(response)}`);
    return json({ ok: response.success, data: response.message });
  } catch (err) {
    console.error(`[backend] gRPC error sending command for room ${roomId}:`, err);
    return json({ ok: false, error: 'gRPC call failed' }, { status: 500 });
  }
}
