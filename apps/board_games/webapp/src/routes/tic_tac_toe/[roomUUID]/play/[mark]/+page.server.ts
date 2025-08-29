import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from '../$types';
import { getOrCreateParticipant } from '../../../../../../../engine/protocol/engine_client';
import type { Participant as ParticipantProto } from '../../../../../../../engine/protocol/engine_pb';

export const load: PageServerLoad = async ({ params, cookies }) => {
    const roomUUID = params.roomUUID;
    const mark = params.mark;

    // Get/assign a session-id. We assign a different session-id per game/UUID, so the
    // same user can play multiple games at the same time from the same browser.
    let session_id = cookies.get('session-id');
    if (session_id === undefined) {
        session_id = crypto.randomUUID();
    }
    cookies.set('session-id', session_id, { path: `/tic_tac_toe/${roomUUID}` });

    // Add or retrieve me as a participant
    try {
        const role = "player";
        console.log(`[backend] GRPC request: getOrCreateParticipant(roomUUID=${roomUUID}, participantUUID=${session_id}, role=${role})`);
        const participant: ParticipantProto = await getOrCreateParticipant(roomUUID, session_id, role);
        console.log(`[backend] Participant '${session_id}' added to the game. Participant is ${JSON.stringify(participant)}`);
        return {
            session_id,
            participant
        };
    } catch (err) {
        console.error(`[backend] gRPC error trying to add participant:`, err);
        redirect(307, `/tic_tac_toe/${roomUUID}/play/rejected`);
    }

};
