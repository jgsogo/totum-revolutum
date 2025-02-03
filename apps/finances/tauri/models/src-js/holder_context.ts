import { HolderContext as HolderContextProto, HolderContextSchema } from "../protos/holder_context_pb.js";
import { Account as AccountProto } from "../protos/account_pb.js";
import { Account, AccountCategory } from "./account.js";
import { Buffer } from 'buffer';
import { fromBinary } from "@bufbuild/protobuf";
import { IncomingMessageConstructor, staticImplements } from "./message.js";
import { Holder } from "./holder.js";

export class HolderContext {
    private readonly holder_context: HolderContextProto;
    private readonly _accounts: Account[];

    constructor(holder_context: HolderContextProto) {
        this.holder_context = holder_context;
        this._accounts = this.holder_context.accounts.map((value: AccountProto) => new Account(value))
    }

    static create_from(data: ArrayBuffer): HolderContext {
        const context: HolderContextProto = fromBinary(HolderContextSchema, Buffer.from(data, 0, data.byteLength));
        return new HolderContext(context);
    }

    holder(): Holder {
        return new Holder(this.holder_context.holder!);
    }

    accounts(): Account[] {
        return this._accounts;
    }

    private filter_accounts(category: AccountCategory): Account[] {
        return this._accounts.filter((account) => account.type().category() === category);
    }

    savings_accounts(): Account[] {
        return this.filter_accounts(AccountCategory.Savings)
    }

    investments_accounts(): Account[] {
        return this.filter_accounts(AccountCategory.Investment)
    }

    retirement_accounts(): Account[] {
        return this.filter_accounts(AccountCategory.Retirement)
    }

    other_accounts(): Account[] {
        return this.filter_accounts(AccountCategory.Other)
    }

    grouped_by_custodian(): Map<number, Account[]> {
        return this._accounts.reduce((store, account: Account) => {
            let key = Number(account.custodian().pk());
            if (!store.has(key)) {
                store.set(key, [account])
            } else {
                store.get(key)!.push(account)
            }
            return store;
        }, new Map<number, Account[]>())
    }
}
staticImplements<IncomingMessageConstructor<HolderContext>>(HolderContext);
