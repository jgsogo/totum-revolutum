import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { createNewRoom, startGame } from '../../../../engine/protocol/engine_client';

export const load: PageServerLoad = async () => {
    const room_uuid = crypto.randomUUID();
    await createNewRoom(room_uuid, "room")
    const tic_tac_toe_slug = "tic_tac_toe";
    await startGame(room_uuid, tic_tac_toe_slug)
    redirect(307, `/room/${room_uuid}`);
};
