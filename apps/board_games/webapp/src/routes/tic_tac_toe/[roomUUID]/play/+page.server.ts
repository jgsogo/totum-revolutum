import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getOrCreateParticipant } from '../../../../../../engine/protocol/engine_client';
import { Participant as ParticipantProto } from '../../../../../../engine/protocol/engine_pb';

export const load: PageServerLoad = async ({ params, cookies }) => {
    // Get/assign a session-id
    let session_id = cookies.get('session-id');
    if (session_id === undefined) {
        session_id = crypto.randomUUID();
    }
    cookies.set('session-id', session_id, { path: '/' }); // FIXME: Make it per game/room so the same user can participate in multiple rooms?

    // Add or retrieve me as a participant
    const roomUUID = params.roomUUID;
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
