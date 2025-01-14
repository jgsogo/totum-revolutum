import { DateWrapper } from "../../../../../libraries/googleapis/src-js/date.js";
import { Account as AccountProto, AccountType } from "../protos/account_pb.js";
import { Custodian } from "../protos/custodian_pb.js";

export class Account {
    private readonly account: AccountProto;

    constructor(account: AccountProto) {
        this.account = account;
    }

    pk(): number {
        return Number(this.account.pk);
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

    ccy(): string {
        return this.account.currencyCode;
    }

    is_numerable(): boolean {
        return this.account.isNumerable;
    }

    open(): DateWrapper {
        return new DateWrapper(this.account.open!);
    }
}
