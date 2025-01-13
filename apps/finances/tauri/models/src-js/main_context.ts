import { MainContext as MainContextProto } from "../protos/main_context_pb.js";
import { Account as AccountProto } from "../protos/account_pb.js";
import { Account } from "./account.js";
import { Holder } from "../protos/holder_pb.js";
import { TransactionGroup } from "../protos/transaction_pb.js";
import { MovementType } from "../protos/movement_pb.js";

export class MainContext {
    private readonly main_context: MainContextProto;

    constructor(main_context: MainContextProto) {
        this.main_context = main_context;
    }

    accounts(): Account[] {
        return this.main_context.accounts.map((value: AccountProto) => new Account(value))
    }

    holders(): Holder[] {
        return this.main_context.holders;
    }

    find_account(pk: number): Account | undefined {
        return this.accounts().find((value: Account) => value.pk() === pk);
    }

    transaction_groups(): TransactionGroup[] {
        return this.main_context.transactionGroups;
    }

    movement_types(): MovementType[] {
        return this.main_context.movementTypes;
    }
}
