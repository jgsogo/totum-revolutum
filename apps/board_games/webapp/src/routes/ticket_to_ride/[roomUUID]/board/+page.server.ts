import type { PageServerLoad } from './$types';
import { MapData } from '../../../../../../games/ticket_to_ride/maps/js/index'
import { readFile } from 'fs/promises';
import { join } from 'path';

const usa_dirpath = 'apps/board_games/games/ticket_to_ride/maps/usa';

export const load: PageServerLoad = async () => {
    // TODO: Somehow we need to be able to choose the map
    console.log(`[backend] Get map data for USA from dir: ${usa_dirpath}`);

    // - Read the proto.bin data
    const buf = await readFile(join(usa_dirpath, 'data.bin'));
    const arrayBuffer: ArrayBuffer = buf.buffer.slice(
        buf.byteOffset,
        buf.byteOffset + buf.byteLength
    );
    const usa_data: MapData = MapData.create_from_array(arrayBuffer);

    // - Read the routes SVG
    const routes_svg = await readFile(join(usa_dirpath, 'routes.svg'), 'utf-8')

    return {
        routes_svg
    };
};
