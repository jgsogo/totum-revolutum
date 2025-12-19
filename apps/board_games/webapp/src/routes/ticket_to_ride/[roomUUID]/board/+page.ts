import type { PageLoad } from './$types';
import { MapData } from '../../../../../../games/ticket_to_ride/maps/js';

export const load: PageLoad = async () => {
    const filepath = "/ticket_to_ride/maps/usa/data.bin";
    const routes_svg = "/ticket_to_ride/maps/usa/routes.svg";

    console.log(`[frontend] Fetch data.bin for USA map: ${filepath}`);

    const data_bin = await fetch(filepath).then(r => r.arrayBuffer());
    const map_data = MapData.create_from_array(data_bin);


	return {
        map_data,
        routes_svg
	};
};
