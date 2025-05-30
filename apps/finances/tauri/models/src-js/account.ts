import { DateWrapper, CurrencyCode, currency_code_from_str } from "../../../../../libraries/googleapis/src-js/index.js";
import { AccountCategory as AccountCategoryProto, Account as AccountProto, AccountType as AccountTypeProto } from "../protos/account_pb.js";
import { Breadcrumb } from "./breadcrumb.js";

import { Custodian } from "./custodian.js";
import { Holder } from "./holder.js";
import { Snapshot } from "./snapshot.js";

export enum AccountCategory {
    Other = 'Other',
    Savings = 'Savings',
    Investment = 'Investment',
    Retirement = 'Retirement',
}

function capitalizeFirstLetter(val: string) {
    return String(val).charAt(0).toUpperCase() + String(val).slice(1);
}

function enumFromStringValue<T>(enm: { [s: string]: T }, value: string): T | undefined {
    return (Object.values(enm) as unknown as string[]).includes(value)
        ? value as unknown as T
        : undefined;
}

export function account_category_from_str(category: string): AccountCategory | undefined {
    return enumFromStringValue(AccountCategory, capitalizeFirstLetter(category.toLowerCase()));
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

    breadcrumb(): Breadcrumb | undefined {
        return this.data.breadcrumb ? new Breadcrumb(this.data.breadcrumb) : undefined;
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

    as_proto(): AccountTypeProto {
        return this.data;
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

    close(): DateWrapper | undefined {
        return this.account.close ? new DateWrapper(this.account.close) : undefined;
    }

    last_snapshot(): Snapshot | undefined {
        return this.account.lastSnapshot ? new Snapshot(this.account.lastSnapshot) : undefined;
    }

    as_proto(): AccountProto {
        return this.account;
    }

    /**
     * Return the list of holders owning the money in this account
     */
    holders(): Holder[] {
        return this.account.holders.map((v) => new Holder(v));
    }

    holded_by(holder: Holder): boolean {
        return this.account.holders.findIndex((v) => Number(v.pk) === holder.pk()) !== -1;
    }
}
