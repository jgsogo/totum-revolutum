import { startGame } from '../../../../../../engine/protocol/engine_client';
import { json } from '@sveltejs/kit';
import type { Empty } from "@bufbuild/protobuf/wkt";

export async function POST({ request, params }) {
  const { roomId } = params;
  const { gameType } = await request.json();

  try {
    console.log(`[backend] POST request: addParticipant(roomID=${roomId}, gameType=${gameType})`);
    const response: Empty = await startGame(roomId, gameType);
    console.log('[backend] Game started');
    return json({ok: true}, {status: 200});
  } catch (err) {
    console.error(`[backend] gRPC error starting game in room ${roomId}:`, err);
    return json({ ok: false, error: 'gRPC call failed' }, { status: 500 });
  }
}
