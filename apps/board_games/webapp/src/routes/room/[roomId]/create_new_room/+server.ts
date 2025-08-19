import { createNewRoom } from '../../../../../../engine/protocol/engine_client';
import { json } from '@sveltejs/kit';
import type { Empty } from "@bufbuild/protobuf/wkt";

export async function POST({ request, params }) {
  const { roomId } = params;
  const { name } = await request.json();

  try {
    console.log(`[backend] POST request: createNewRoom(roomID=${roomId}, name=${name})`);
    const response: Empty = await createNewRoom(roomId, name);
    console.log('[backend] Room created');
    return json({ok: true}, {status: 200});
  } catch (err) {
    console.error(`[backend] gRPC error creating room ${roomId}:`, err);
    return json({ ok: false, error: 'gRPC call failed' }, { status: 500 });
  }
}
