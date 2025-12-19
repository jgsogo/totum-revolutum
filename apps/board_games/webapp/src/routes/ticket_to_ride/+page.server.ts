import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { createNewRoom, startGame } from '../../../../engine/protocol/engine_client';

export const load: PageServerLoad = async () => {
    const room_uuid = crypto.randomUUID();
    await createNewRoom(room_uuid, "room")
    const slug = "ticket_to_ride";
    await startGame(room_uuid, slug)
    redirect(307, `/room/${room_uuid}`);
};
