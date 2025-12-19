import type { PageLoad } from './$types';
import { MapData } from '../../../../../../games/ticket_to_ride/maps/js';

export const load: PageLoad = async () => {
    console.log(`[frontend] Fetch data.bin for USA map`);
    const data_bin = await fetch("/ticket_to_ride/maps/usa/data.bin").then(r => r.arrayBuffer());

    const map_data = MapData.create_from_array(data_bin);
    console.log(`[frontend] Got MapData for ${map_data.name()}`);

	return {
        map_data
	};
};
