import { create, toBinary, } from "@bufbuild/protobuf";
import { describe, expect, test } from 'vitest';
import { Buffer } from 'buffer';


import { BoardSchema, Player as PlayerProto } from "../../models/board_pb.js"
import { Board, Player } from "./board.js";

describe('AppState', () => {
    let board_proto = create(BoardSchema, {
        boardStatus: [
            PlayerProto.PLAYER_X, PlayerProto.PLAYER_O, PlayerProto.NONE,
            PlayerProto.NONE, PlayerProto.PLAYER_X, PlayerProto.NONE,
            PlayerProto.NONE, PlayerProto.NONE, PlayerProto.PLAYER_O,
        ], turnState: {
            case: "currentTurn",
            value: PlayerProto.PLAYER_X,
        }
    });
    let board_uint8_buffer = toBinary(BoardSchema, board_proto);
    const arrayBuffer = Buffer.from(board_uint8_buffer.buffer, 0, board_uint8_buffer.byteLength);

    const board = Board.create_from_array(arrayBuffer.buffer as ArrayBuffer);

    test('members', () => {
        expect(board.status()).toEqual([
            Player.PLAYER_X, Player.PLAYER_O, Player.NONE,
            Player.NONE, Player.PLAYER_X, Player.NONE,
            Player.NONE, Player.NONE, Player.PLAYER_O,
        ]);

        expect(board.current_turn()).toBe(Player.PLAYER_X);
    });
});
