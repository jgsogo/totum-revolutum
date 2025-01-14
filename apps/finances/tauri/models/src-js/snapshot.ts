import { Snapshot as SnapshotProto } from "../protos/snapshot_pb.js";
import { MoneyAmount } from "./money_amount.js";
import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";

export class Snapshot {
    private readonly snapshot: SnapshotProto;

    constructor(snapshot: SnapshotProto) {
        this.snapshot = snapshot;
    }

    pk(): number {
        return Number(this.snapshot.pk);
    }

    dateValue(): DateWrapper {
        return new DateWrapper(this.snapshot.dateValue!);
    }

    amount(): MoneyAmount {
        return new MoneyAmount(this.snapshot.amount!);
    }
}
