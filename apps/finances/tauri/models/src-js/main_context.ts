import { MainContext as MainContextProto, MainContextSchema } from "../protos/main_context_pb.js";
import { Account, AccountType } from "./account.js";
import { MovementType } from './movement_type.js';
import { staticImplements, IncomingMessageConstructor } from "./message.js";
import { fromBinary } from "@bufbuild/protobuf";
import { Buffer } from 'buffer';
import { Holder } from "./holder.js";
import { TransactionGroup } from "./transaction.js";

export class MainContext {
    private readonly main_context: MainContextProto;

    constructor(main_context: MainContextProto) {
        this.main_context = main_context;
    }

    static create_from(data: ArrayBuffer): MainContext {
        const context: MainContextProto = fromBinary(MainContextSchema, Buffer.from(data, 0, data.byteLength));
        return new MainContext(context);
    }

    accounts(): Account[] {
        return this.main_context.accounts.map((value) => new Account(value))
    }

    holders(): Holder[] {
        return this.main_context.holders.map((value) => new Holder(value))
    }

    transaction_groups(): TransactionGroup[] {
        return this.main_context.transactionGroups.map((value) => new TransactionGroup(value))
    }

    movement_types(): MovementType[] {
        return this.main_context.movementTypes.map((value) => new MovementType(value));
    }

    account_types(): AccountType[] {
        return this.main_context.accountTypes.map((value) => new AccountType(value));
    }

    find_account(pk: number): Account | undefined {
        return this.accounts().find((value: Account) => value.pk() === pk);
    }
}
staticImplements<IncomingMessageConstructor<MainContext>>(MainContext);
