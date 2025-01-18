import { NewTransaction as NewTransactionProto, NewTransactionSchema, TransactionGroup } from "../protos/transaction_pb.js";
import { create, toBinary } from "@bufbuild/protobuf";
import { OutgoingMessage } from "./message.js";
import { NewMovement } from "./movement.js";

export class NewTransaction extends OutgoingMessage {
    private data: NewTransactionProto;

    constructor(name: string, description?: string, transaction_group?: TransactionGroup) {
        super();
        this.data = create(NewTransactionSchema, { name, description, transactionGroupPk: transaction_group ? transaction_group.pk : undefined });
    }

    toBinary(): Uint8Array {
        return toBinary(NewTransactionSchema, this.data);
    }

    pushFromMovement(movement: NewMovement) {
        this.data.movementsFrom.push(movement.innerType());
    }

    pushToMovement(movement: NewMovement) {
        this.data.movementsTo.push(movement.innerType());
    }

}
