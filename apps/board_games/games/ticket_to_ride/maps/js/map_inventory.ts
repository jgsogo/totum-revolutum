import { MapData } from "./map_data.js"

const USA_PROTO_FILEPATH = "usa/data.bin"

export function usa_map_data(): MapData {
    return MapData.create_from_protofile(USA_PROTO_FILEPATH);
}
