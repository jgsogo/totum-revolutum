import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import * as db from '$lib/server/database';

export const load: PageServerLoad = async (params) => {
    // If there is already a 'game' in this room, redirect to it.
    let game = await db.get_game(params.roomId);
    if (game !== undefined) {
        redirect(307, `/${game.game_type_id}/${params.roomId}/board`);
    }
};
