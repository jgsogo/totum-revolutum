import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from '../$types';
import { sendAction, joinGame } from '../../../../../../../engine/protocol/engine_client';
import type { Participant as ParticipantProto } from '../../../../../../../engine/protocol/engine_pb';
import { Action, Player } from '../../../../../../../games/tic_tac_toe/webapp/src';
import type { Empty } from "@bufbuild/protobuf/wkt";

export const load: PageServerLoad = async ({ params, cookies }) => {
    const roomUUID = params.roomUUID;
    const mark = params.mark;

    // Get/assign a session-id. We assign a different session-id per game/UUID, so the
    // same user can play multiple games at the same time from the same browser.
    let session_id = cookies.get('session-id');
    if (session_id === undefined) {
        session_id = crypto.randomUUID();
    }
    cookies.set('session-id', session_id, {
        path: `/tic_tac_toe/${roomUUID}`,
        secure: false, // FIXME: We need this if using HTTP (https://github.com/jshttp/cookie#secure)
    });

    // Add or retrieve me as a participant
    try {
        console.log(`[backend] GRPC request: joinGame(roomUUID=${roomUUID}, participantUUID=${session_id})`);
        const player = mark === 'X' ? Player.X : Player.O;
        let action = Action.create_join_game(player);
        const response: Empty = await joinGame(roomUUID, session_id, action.toBinary());
        return {
            player
        };
    } catch (err) {
        console.error(`[backend] gRPC error trying to add participant:`, err);
        redirect(307, `/tic_tac_toe/${roomUUID}/play/rejected`);
    }

};
