import { MapData as MapDataProto, MapDataSchema, Background as BackgroundProto } from "../map_pb.js"
import { IncomingMessageConstructor, staticImplements } from "./message.js";
import { Buffer } from 'buffer';
import { fromBinary } from "@bufbuild/protobuf";


export class Image {
    private readonly proto: BackgroundProto;

    constructor(proto: BackgroundProto) {
        this.proto = proto;
    }

    filename(): string {
        return this.proto.filename;
    }

    rotate(): number {
        return this.proto.rotate;
    }

    scale(): number {
        return this.proto.scale;
    }

    translate(): [number, number] {
        return [this.proto.translateX, this.proto.translateY];
    }
}

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

    size(): [number, number] {
        return [this.proto.sizeX, this.proto.sizeY]
    }

    background(): Image | undefined {
        return this.proto.background ? new Image(this.proto.background) : undefined;
    }
}
staticImplements<IncomingMessageConstructor<MapData>>(MapData);
