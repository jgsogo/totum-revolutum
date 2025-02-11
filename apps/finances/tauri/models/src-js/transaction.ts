import { Transaction as TransactionProto, TransactionGroup as TransactionGroupProto, TransactionSchema } from "../protos/transaction_pb.js";
import { create, toBinary } from "@bufbuild/protobuf";
import { OutgoingMessage } from "./message.js";
import { Movement } from "./movement.js";

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



export class Transaction extends OutgoingMessage {
    private proto: TransactionProto;

    constructor(proto: TransactionProto) {
        super()
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

    static create_from(name: string, description?: string, transaction_group?: TransactionGroup): Transaction {
        const proto = create(TransactionSchema, { name, description, group: transaction_group ? transaction_group.as_proto() : undefined });
        return new Transaction(proto)
    }

    as_proto(): TransactionProto {
        return this.proto
    }

    toBinary(): Uint8Array {
        return toBinary(TransactionSchema, this.proto);
    }

    pushFromMovement(movement: Movement) {
        this.proto.movementsFrom.push(movement.as_proto());
    }

    pushToMovement(movement: Movement) {
        this.proto.movementsTo.push(movement.as_proto());
    }

}
