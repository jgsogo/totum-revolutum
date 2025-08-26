import { create, toBinary, } from "@bufbuild/protobuf";
import { describe, expect, test } from 'vitest';
import { Buffer } from 'buffer';


import {  BoardSchema} from "../../models/board_pb.js"
import { Board } from "./board.js";

describe('AppState', () => {
    let board_proto = create(BoardSchema, { boardStatus: 'board-status', currentTurn: 42 });
    let board_uint8_buffer = toBinary(BoardSchema, board_proto);
    const arrayBuffer = Buffer.from(board_uint8_buffer.buffer, 0, board_uint8_buffer.byteLength);

    let board = Board.create_from_array(arrayBuffer.buffer as ArrayBuffer);

    test('members', () => {
        expect(board.status()).toBe("board-status");
    });
});
