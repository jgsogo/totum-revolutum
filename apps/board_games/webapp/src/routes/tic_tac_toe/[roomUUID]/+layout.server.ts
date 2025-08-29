import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { get_game } from '$lib/server/database';


export const load: LayoutServerLoad = async ({ params }) => {
    const roomUUID = params.roomUUID;

    // Check if the given roomUUID exists
    const game = await get_game(roomUUID);
    if (game === undefined) {
        console.log(`[backend] There is no game running in room ${roomUUID}. Redirecting to a new room`);
        redirect(307, `/`);
    }
};
