import * as db from '$lib/server/database';
import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
    // If there is already a 'game' in this room, redirect to it.
    let game = await db.get_game(params.roomId);
    if (game !== undefined) {
        redirect(307, `/${game.game_type_id}/${params.roomId}/admin`);
    }

	return {
        game_types: await db.get_game_types(),
	};
};
