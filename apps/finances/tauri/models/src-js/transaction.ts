import { Transaction as TransactionProto, TransactionGroup as TransactionGroupProto, TransactionSchema } from "../protos/transaction_pb.js";
import { create, toBinary } from "@bufbuild/protobuf";
import { OutgoingMessage } from "./message.js";
import { Movement, NewMovement } from "./movement.js";

export class TransactionGroup {
    private proto: TransactionGroupProto;

    constructor(proto: TransactionGroupProto) {
        this.proto = proto;
    }

    pk(): number {
        return Number(this.proto.pk);
    }

    name(): string {
        return this.proto.name;
    }

    description(): string | undefined {
        return this.proto.description;
    }

    as_proto(): TransactionGroupProto {
        return this.proto;
    }
}



export class Transaction {
    private proto: TransactionProto;

    constructor(proto: TransactionProto) {
        this.proto = proto;
    }

    pk(): number {
        return Number(this.proto.pk);
    }

    name(): string {
        return this.proto.name;
    }

    description(): string | undefined {
        return this.proto.description;
    }

    group(): TransactionGroup | undefined {
        return this.proto.group ? new TransactionGroup(this.proto.group!) : undefined;
    }

    movements_from(): Movement[] {
        return this.proto.movementsFrom.map((value) => new Movement(value))
    }

    movements_to(): Movement[] {
        return this.proto.movementsTo.map((value) => new Movement(value))
    }
}

export class NewTransaction extends OutgoingMessage {

    private data: TransactionProto;

    constructor(name: string, description?: string, transaction_group?: TransactionGroup) {
        super();
        this.data = create(TransactionSchema, { name, description, group: transaction_group ? transaction_group.as_proto() : undefined });
    }

    toBinary(): Uint8Array {
        return toBinary(TransactionSchema, this.data);
    }


    pushFromMovement(movement: NewMovement) {
        this.data.movementsFrom.push(movement.as_proto());
    }

    pushToMovement(movement: NewMovement) {
        this.data.movementsTo.push(movement.as_proto());
    }

}
