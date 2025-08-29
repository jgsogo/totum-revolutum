import { Board as BoardProto, BoardSchema } from "../../models/board_pb.js"
import { IncomingMessageConstructor, staticImplements } from "./message.js";
import { Buffer } from 'buffer';
import { fromBinary } from "@bufbuild/protobuf";

export class Board {
    private readonly board: BoardProto;

    constructor(board: BoardProto) {
        this.board = board;
    }

    static create_from_array(data: ArrayBuffer): Board {
        const board_proto: BoardProto = fromBinary(BoardSchema, Buffer.from(data, 0, data.byteLength)) as BoardProto;
        return new Board(board_proto);
    }

    status(): string {
        return this.board.boardStatus;
    }

    current_turn(): number | undefined {
        return this.board.turnState.case === 'currentTurn' ? this.board.turnState.value : undefined;
    }

    winner(): number | undefined {
        return this.board.turnState.case === 'winner' ? this.board.turnState.value : undefined;
    }

    draw(): boolean | undefined {
        return this.board.turnState.case === 'draw' ? this.board.turnState.value : undefined;
    }
}
staticImplements<IncomingMessageConstructor<Board>>(Board);
