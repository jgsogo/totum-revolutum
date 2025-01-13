import { Account as AccountProto, AccountType } from "../protos/account_pb.js";
import { Ccy } from "../protos/ccy_pb.js";
import { Custodian } from "../protos/custodian_pb.js";

export class Account {
    private readonly account: AccountProto;

    constructor(account: AccountProto) {
        this.account = account;
    }

    pk(): number {
        return this.account.pk as unknown as number;
    }

    custodian(): Custodian {
        return this.account.custodian!;
    }

    name(): string {
        return this.account.name;
    }

    identifier(): string | undefined {
        return this.account.identifier
    }

    type(): AccountType {
        return this.account.type!;
    }

    ccy(): Ccy {
        return this.account.ccy;
    }

    is_numerable(): boolean {
        return this.account.isNumerable;
    }
}
