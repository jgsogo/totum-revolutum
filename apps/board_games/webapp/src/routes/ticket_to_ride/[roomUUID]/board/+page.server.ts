import type { PageServerLoad } from './$types';
import { MapData } from '../../../../../../games/ticket_to_ride/maps/js/index'
import { readFile } from 'fs/promises';
import { join } from 'path';



export const load: PageServerLoad = async () => {
    // TODO: Somehow we need to be able to choose the map
    const filepath = join('apps/board_games/games/ticket_to_ride/maps', 'usa/data.bin');
    console.log(`[backend] Get the map data for USA: ${filepath}`);

    const buf = await readFile(filepath);
    const arrayBuffer: ArrayBuffer = buf.buffer.slice(
        buf.byteOffset,
        buf.byteOffset + buf.byteLength
    );

    const usa_data: MapData = MapData.create_from_array(arrayBuffer);
    console.log(`[backend] USA map is loaded: ${usa_data.name()}`);

    return {
        // usa_data,
        // game_types: await db.get_game_types(),
    };
};
