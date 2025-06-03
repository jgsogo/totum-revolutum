import { create, fromBinary, toBinary } from "@bufbuild/protobuf";

import { SnapshotsRequest as SnapshotsRequestProto, SnapshotsRequestSchema, SnapshotsResponse as SnapshotsResponseProto, SnapshotsResponseSchema } from "../protos/snapshot_pb.js";
import { IncomingMessageConstructor, OutgoingMessage, staticImplements } from "./message.js";
import { Buffer } from 'buffer';
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";

import { Snapshot } from "./snapshot.js";

export class SnapshotsRequest extends OutgoingMessage {
    private readonly data: SnapshotsRequestProto;

    constructor(account_pk: number, start_date?: DateWrapper, end_date?: DateWrapper) {
        super();

        this.data = create(SnapshotsRequestSchema, { accountPk: BigInt(account_pk), startDate: start_date?.as_proto(), endDate: end_date?.as_proto() });
    }

    toBinary(): Uint8Array {
        return toBinary(SnapshotsRequestSchema, this.data);
    }
}

export class SnapshotsResponse {
    private readonly proto: SnapshotsResponseProto;

    constructor(proto: SnapshotsResponseProto) {
        this.proto = proto;
    }

    static create_from_array(data: ArrayBuffer): SnapshotsResponse {
        const context: SnapshotsResponseProto = fromBinary(SnapshotsResponseSchema, Buffer.from(data, 0, data.byteLength));
        return new SnapshotsResponse(context);
    }

    snapshots(): Snapshot[] {
        return this.proto.snapshots.map((value) => new Snapshot(value))
    }

    start_date(): DateWrapper | undefined {
        return this.proto.startDate ? new DateWrapper(this.proto.startDate) : undefined;
    }

    end_date(): DateWrapper | undefined {
        return this.proto.endDate ? new DateWrapper(this.proto.endDate) : undefined;
    }

}
staticImplements<IncomingMessageConstructor<SnapshotsResponse>>(SnapshotsResponse);
