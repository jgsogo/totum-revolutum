import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { room_exists } from '$lib/server/database';


export const load: LayoutServerLoad = async ({ params }) => {
    const roomUUID = params.roomId;

    // Check if the given roomUUID exists
    const exists = await room_exists(roomUUID);
    if (!exists) {
        console.log(`[backend] The room '${roomUUID}' doesn't exist. Redirecting to a new room`);
        redirect(307, `/`);
    }
}
