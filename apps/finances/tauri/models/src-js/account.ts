import { DateWrapper, CurrencyCode, currency_code_from_str } from "../../../../../libraries/googleapis/src-js/index.js";
import { AccountCategory as AccountCategoryProto, Account as AccountProto, AccountType as AccountTypeProto } from "../protos/account_pb.js";

import { Custodian } from "./custodian.js";
import { Snapshot } from "./snapshot.js";

export enum AccountCategory {
    Other = 0,
    Savings = 1,
    Investment = 2,
    Retirement = 3,
}


export class AccountType {
    private readonly data: AccountTypeProto;

    constructor(data: AccountTypeProto) {
        this.data = data;
    }

    pk(): number {
        return Number(this.data.pk);
    }

    name(): string {
        return this.data.name;
    }

    breadcrumb(): string[] | undefined {
        return this.data.breadcrumb;
    }

    category(): AccountCategory {
        switch (this.data.category!) {
            case AccountCategoryProto.Other:
                return AccountCategory.Other;
            case AccountCategoryProto.Savings:
                return AccountCategory.Savings;
            case AccountCategoryProto.Investment:
                return AccountCategory.Investment;
            case AccountCategoryProto.Retirement:
                return AccountCategory.Retirement;
            default:
                throw new Error(`Unknown AccountCategory ${this.data.category}`);
        }
    }

}

export class Account {
    private readonly account: AccountProto;

    constructor(account: AccountProto) {
        this.account = account;
    }

    pk(): number {
        return Number(this.account.pk);
    }

    custodian(): Custodian {
        return new Custodian(this.account.custodian!);
    }

    name(): string {
        return this.account.name;
    }

    identifier(): string | undefined {
        return this.account.identifier
    }

    type(): AccountType {
        return new AccountType(this.account.type!);
    }

    ccy(): CurrencyCode {
        return currency_code_from_str(this.account.currencyCode)!;
    }

    is_numerable(): boolean {
        return this.account.isNumerable;
    }

    open(): DateWrapper {
        return new DateWrapper(this.account.open!);
    }

    last_snapshot(): Snapshot | undefined {
        return this.account.lastSnapshot ? new Snapshot(this.account.lastSnapshot) : undefined;
    }
}
