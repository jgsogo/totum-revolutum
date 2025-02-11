import { Snapshot as SnapshotProto, SnapshotSchema } from "../protos/snapshot_pb.js";
import { MoneyAmount } from "./money_amount.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";
import { create, toBinary } from "@bufbuild/protobuf";
import { OutgoingMessage } from "./message.js";


export class Snapshot extends OutgoingMessage {
    private readonly proto: SnapshotProto;

    constructor(proto: SnapshotProto) {
        super()
        this.proto = proto;
    }

    pk(): number {
        return Number(this.proto.pk);
    }

    date_value(): DateWrapper {
        return new DateWrapper(this.proto.dateValue!);
    }

    amount(): MoneyAmount {
        return new MoneyAmount(this.proto.amount!);
    }

    account_pk(): number {
        return Number(this.proto.accountPk);
    }

    static create_from(account_pk: number, date_value: DateWrapper, money_amount: MoneyAmount): Snapshot {
        const proto = create(SnapshotSchema, { accountPk: BigInt(account_pk), dateValue: date_value.as_proto(), amount: money_amount.as_proto() });
        return new Snapshot(proto)
    }

    toBinary(): Uint8Array {
        return toBinary(SnapshotSchema, this.proto);
    }


}
