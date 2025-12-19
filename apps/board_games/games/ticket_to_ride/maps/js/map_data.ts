import { MapData as MapDataProto, MapDataSchema } from "../map_pb.js"
import { IncomingMessageConstructor, staticImplements } from "./message.js";
import { Buffer } from 'buffer';
import { fromBinary } from "@bufbuild/protobuf";




export class MapData {
    private readonly proto: MapDataProto;

    constructor(proto: MapDataProto) {
        this.proto = proto;
    }

    static create_from_array(data: ArrayBuffer): MapData {
        const proto: MapDataProto = fromBinary(MapDataSchema, Buffer.from(data, 0, data.byteLength)) as MapDataProto;
        return new MapData(proto);
    }

    name(): string {
        return this.proto.name;
    }
}
staticImplements<IncomingMessageConstructor<MapData>>(MapData);
