import { addParticipant } from '../../../../../../../engine/protocol/engine_client';
import { json } from '@sveltejs/kit';
import type { Empty } from "@bufbuild/protobuf/wkt";

export async function POST({ request, params }) {
  const { roomId } = params;
  const { participantUUID, participantRole } = await request.json();

  try {
    console.log(`[backend] POST request: addParticipant(roomID=${roomId}, participantUUID=${participantUUID}, participantRole=${participantRole})`);
    const response: Empty = await addParticipant(roomId, participantUUID, participantRole);
    console.log('[backend] Participant added');
    return json({ok: true}, {status: 200});
  } catch (err) {
    console.error(`[backend] gRPC error adding participant to room ${roomId}:`, err);
    return json({ ok: false, error: 'gRPC call failed' }, { status: 500 });
  }
}
