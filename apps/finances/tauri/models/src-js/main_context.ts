import { MainContext as MainContextProto, MainContextSchema } from "../protos/main_context_pb.js";
import { Account as AccountProto } from "../protos/account_pb.js";
import { MovementType as MovementTypeProto } from "../protos/movement_pb.js";
import { Account } from "./account.js";
import { Holder } from "../protos/holder_pb.js";
import { TransactionGroup } from "../protos/transaction_pb.js";
import { MovementType } from './movement_type.js';
import { staticImplements, IncomingMessageConstructor } from "./message.js";
import { fromBinary } from "@bufbuild/protobuf";
import { Buffer } from 'buffer';

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
        return this.main_context.movementTypes.map((value: MovementTypeProto) => new MovementType(value));
    }
}
staticImplements<IncomingMessageConstructor<MainContext>>(MainContext);
