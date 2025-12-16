import { Board as BoardProto, BoardSchema, Winner as WinnerProto, Player as PlayerProto } from "../../models/board_pb.js"
import { IncomingMessageConstructor, staticImplements } from "./message.js";
import { Buffer } from 'buffer';
import { fromBinary } from "@bufbuild/protobuf";

export enum Player {
  PLAYER_X = 1,
  PLAYER_O,
  NONE,
}

export namespace Player {
    export function from_proto(value: PlayerProto): Player  {
        switch (value) {
            case PlayerProto.PLAYER_X:
                return Player.PLAYER_X;
            case PlayerProto.PLAYER_O:
                return Player.PLAYER_O;
            case PlayerProto.NONE:
                return Player.NONE;
        }
    }
}

export class BoardWinner {
    private readonly proto: WinnerProto;

    constructor(proto: WinnerProto) {
        this.proto = proto;
    }

    player(): Player {
        return Player.from_proto( this.proto.player);
    }

    line(): number[] {
        return this.proto.line
    }
}
export class Board {
    private readonly board: BoardProto;

    constructor(board: BoardProto) {
        this.board = board;
    }

    static create_from_array(data: ArrayBuffer): Board {
        const board_proto: BoardProto = fromBinary(BoardSchema, Buffer.from(data, 0, data.byteLength)) as BoardProto;
        return new Board(board_proto);
    }

    status(): Player[] {
        return this.board.boardStatus.map((p: PlayerProto) => Player.from_proto(p));
    }

    current_turn(): Player | undefined {
        return this.board.turnState.case === 'currentTurn' ? Player.from_proto( this.board.turnState.value) : undefined;
    }

    winner(): BoardWinner | undefined {
        return this.board.turnState.case === 'winner' ? new BoardWinner(this.board.turnState.value) : undefined;
    }

    draw(): boolean | undefined {
        return this.board.turnState.case === 'draw' ? this.board.turnState.value : undefined;
    }
}
staticImplements<IncomingMessageConstructor<Board>>(Board);
