import { HolderContext as HolderContextProto } from "../protos/holder_context_pb.js";
import { Account as AccountProto, AccountCategory } from "../protos/account_pb.js";
import { Custodian } from "../protos/custodian_pb.js";
import { Account } from "./account.js";
import { Holder } from "../protos/holder_pb.js";

export class HolderContext {
    private readonly holder_context: HolderContextProto;
    private readonly accounts: Account[];

    constructor(holder_context: HolderContextProto) {
        this.holder_context = holder_context;
        this.accounts = this.holder_context.accounts.map((value: AccountProto) => new Account(value))
    }

    holder(): Holder {
        return this.holder_context.holder!;
    }

    filter_accounts(category: AccountCategory): Account[] {
        return this.accounts.filter((account) => account.type().category === category);
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

    grouped_by_custodian(): Map<Custodian, Account[]> {
        return this.accounts.reduce((store, account: Account) => {
            let key = account.custodian();
            if (!store.has(key)) {
                store.set(key, [account])
            } else {
                store.get(key)!.push(account)
            }
            return store;
        }, new Map<Custodian, Account[]>())
    }
}
