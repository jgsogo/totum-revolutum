import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';
import { get_game } from '$lib/server/database';


export const load: LayoutServerLoad = async ({ params }) => {
    const roomUUID = params.roomUUID;

    // Check if the given roomUUID exists
    const game = await get_game(roomUUID);
    if (game === undefined) {
        console.log(`[backend] There is no game running in room ${roomUUID}. Redirecting to root`);
        redirect(307, `/`);
    }

    // Check if the given room is playing TicketToRide
    if (game.game_type_id !== 'ticket_to_ride') {
        console.log(`[backend] Game mismatch in room ${roomUUID}. Expected 'ticket_to_ride', found '${game.game_type_id}'. Redirecting to root`);
        redirect(307, `/`);
    }
};
