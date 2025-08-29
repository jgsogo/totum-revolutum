import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { createNewRoom } from '../../../engine/protocol/engine_client';

export const load: PageServerLoad = async () => {
    const room_uuid = crypto.randomUUID();
    await createNewRoom(room_uuid, "room");
	redirect(307, `/room/${room_uuid}`);
};
