// import { create, toBinary, } from "@bufbuild/protobuf";
import { describe, expect, test } from 'vitest';
// import { Buffer } from 'buffer';

import { MapData, usa_map_data } from './index.js'


// import { BoardSchema, Player as PlayerProto } from "../../models/board_pb.js"
// import { Board, Player } from "./board.js";

describe('MapData - USA', () => {

    const usa_data: MapData = usa_map_data()

    test('name', () => {
        expect(usa_data.name()).toBe('USA');
    });
});
