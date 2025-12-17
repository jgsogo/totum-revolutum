import type { PageServerLoad } from './$types';


export const load: PageServerLoad = async () => {
    // TODO: Somehow we need to be able to choose the map
    const map_name = "usa"

    // Parse the 'textproto' file with the map information
    const content = await import(`$lib/ticket_to_ride/maps/maps/${map_name}.textproto?raw`)
        .then(m => m.default);

    console.log(file);

    return {
        map_name,
        // game_types: await db.get_game_types(),
    };
};
