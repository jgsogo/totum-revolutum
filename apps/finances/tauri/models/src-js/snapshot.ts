import { Snapshot as SnapshotProto, SnapshotSchema } from "../protos/snapshot_pb.js";
import { MoneyAmount, NewMoneyAmount } from "./money_amount.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";
import { create, toBinary } from "@bufbuild/protobuf";
import { OutgoingMessage } from "./message.js";


export class Snapshot {
    private readonly proto: SnapshotProto;

    constructor(proto: SnapshotProto) {
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
}

export class NewSnapshot extends OutgoingMessage {
    private data: SnapshotProto;

    constructor() {
        super();
        this.data = create(SnapshotSchema, {});
    }

    toBinary(): Uint8Array {
        return toBinary(SnapshotSchema, this.data);
    }

    setAccountPk(account_pk: number) {
        this.data.accountPk = BigInt(account_pk)
    }

    setDate(date: DateWrapper) {
        this.data.dateValue = date.as_proto();
    }

    set_money_amount(money_amount: NewMoneyAmount) {
        this.data.amount = money_amount.as_proto();
    }
}
