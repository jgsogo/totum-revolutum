import { sendAction } from '../../../../../../../engine/protocol/engine_client';
import { json } from '@sveltejs/kit';
import type { Empty } from "@bufbuild/protobuf/wkt";
import { Action, ActionPlaceMark } from '../../../../../../../games/tic_tac_toe/webapp/src/index';

export async function POST({ request, params }) {
  const { roomUUID } = params;
  const { player_session, cell_id } = await request.json();

  try {
    console.log(`[backend] POST request: placeMark(roomID=${roomUUID}, player_session=${player_session}, cell_id=${cell_id})`);

    let action_place_mark = new ActionPlaceMark(cell_id);
    let action = new Action(action_place_mark);
    const response: Empty = await sendAction(roomUUID, player_session, action.toBinary());

    return json({ ok: true }, { status: 200 });
  } catch (err) {
    console.error(`[backend] gRPC error placing mark in room ${roomUUID}:`, err);
    return json({ ok: false, error: 'gRPC call failed' }, { status: 500 });
  }
}
