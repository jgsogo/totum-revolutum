import { Account as AccountProto, AccountType } from "../protos/account_pb.js";
import { Custodian } from "../protos/custodian_pb.js";

export class Account {
    private readonly account: AccountProto;

    constructor(account: AccountProto) {
        this.account = account;
    }

    custodian(): Custodian {
        return this.account.custodian!;
    }

    type(): AccountType {
        return this.account.type!;
    }
}
