import { AccountContext as AccountContextProto } from "../protos/account_context_pb.js";
import { Snapshot as SnapshotProto } from "../protos/snapshot_pb.js";
import { Movement as MovementProto } from "../protos/movement_pb.js";
import { Account } from "./account.js";
import { Movement } from "./movement.js";
import { Snapshot } from "./snapshot.js";

export class AccountContext {
    private readonly account_context: AccountContextProto;

    constructor(account_context: AccountContextProto) {
        this.account_context = account_context;
    }

    account(): Account {
        return new Account(this.account_context.account!);
    }

    snapshots(): Snapshot[] {
        return this.account_context.snapshots.map((value: SnapshotProto) => new Snapshot(value))
    }

    movements(): Movement[] {
        return this.account_context.movements.map((value: MovementProto) => new Movement(value))
    }
}
