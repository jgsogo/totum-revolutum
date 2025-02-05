import { create, fromBinary, toBinary } from "@bufbuild/protobuf";
import { LastTransactionsRequest as LastTransactionsRequestProto, LastTransactionsRequestSchema, LastTransactionsResponse as LastTransactionsResponseProto, LastTransactionsResponseSchema } from "../protos/transaction_pb.js";
import { IncomingMessageConstructor, OutgoingMessage, staticImplements } from "./message.js";
import { MovementDirection } from "./movement.js";
import { MovementDirection as MovementDirectionProto } from "../protos/movement_pb.js";
import { Transaction } from "./transaction.js";
import { Buffer } from 'buffer';

export class LastTransactionsRequest extends OutgoingMessage {
    private readonly data: LastTransactionsRequestProto;

    constructor(account_pk: number, account_movement_direction: MovementDirection, n_transactions?: number) {
        super();

        let mov_direction = undefined;
        switch (account_movement_direction) {
            case MovementDirection.In:
                mov_direction = MovementDirectionProto.In;
                break;
            case MovementDirection.Out:
                mov_direction = MovementDirectionProto.Out;
                break;
            default:
                throw new Error(`Unhandled MovementDirection ${account_movement_direction}`);
        }
        this.data = create(LastTransactionsRequestSchema, { accountPk: BigInt(account_pk), nTransactions: n_transactions, accountMovementDirection: mov_direction });
    }

    toBinary(): Uint8Array {
        return toBinary(LastTransactionsRequestSchema, this.data);
    }
}

export class LastTransactionsResponse {
    private readonly proto: LastTransactionsResponseProto;

    constructor(proto: LastTransactionsResponseProto) {
        this.proto = proto;
    }

    static create_from(data: ArrayBuffer): LastTransactionsResponse {
        const context: LastTransactionsResponseProto = fromBinary(LastTransactionsResponseSchema, Buffer.from(data, 0, data.byteLength));
        return new LastTransactionsResponse(context);
    }

    transactions(): Transaction[] {
        return this.proto.transactions.map((value) => new Transaction(value))
    }

}
staticImplements<IncomingMessageConstructor<LastTransactionsResponse>>(LastTransactionsResponse);
