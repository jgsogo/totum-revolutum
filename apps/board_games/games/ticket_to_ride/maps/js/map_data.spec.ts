import { describe, expect, test } from 'vitest';


import { MapData } from './index.js'
import { USA_PROTO_FILEPATH } from '../usa/index.js';
import { readFileSync } from 'fs';



describe('MapData - USA', () => {
    const buf = readFileSync(USA_PROTO_FILEPATH);
    const arrayBuffer = buf.buffer.slice(
            buf.byteOffset,
            buf.byteOffset + buf.byteLength
        );

    const usa_data: MapData = MapData.create_from_array(arrayBuffer);

    test('name', () => {
        expect(usa_data.name()).toBe('USA');
    });
});
