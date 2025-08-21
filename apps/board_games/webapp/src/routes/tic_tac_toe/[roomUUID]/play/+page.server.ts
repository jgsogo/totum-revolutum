import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { addParticipant } from '../../../../../../engine/protocol/engine_client';
import type { Empty } from "@bufbuild/protobuf/wkt";

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
        // FIXME: Need getOrCreateParticipant()
        const role = "player";
        console.log(`[backend] GRPC request: addParticipant(roomUUID=${roomUUID}, participantUUID=${session_id}, role=${role})`);
        const response: Empty = await addParticipant(roomUUID, session_id, role);
        console.log(`[backend] Participant '${session_id}' added to the game!`);
    } catch (err) {
        console.error(`[backend] gRPC error trying to add participant:`, err);
        redirect(307, `/tic_tac_toe/${roomUUID}/play/rejected`);
    }

    return {
        session_id
    };
};
