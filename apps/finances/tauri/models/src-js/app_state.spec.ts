import { create, toBinary, } from "@bufbuild/protobuf";
import { describe, expect, test } from 'vitest';
import { AppState } from "./app_state.js";
import { AppStateSchema, DatabaseConnectionSchema } from "../protos/app_state_pb.js";
import { Buffer } from 'buffer';
import { CurrencyCode } from "../../../../../libraries/googleapis/src-js/money.js";

describe('AppState', () => {
    let db = create(DatabaseConnectionSchema, { postgresUrl: 'postgres-url' });
    let app_state_proto = create(AppStateSchema, { baseCcy: 'EUR', baseMediaUrl: 'base-media-url', baseStaticUrl: 'base-static-url', baseUrl: 'base-url', db });
    let app_state_uint8_buffer = toBinary(AppStateSchema, app_state_proto);
    const arrayBuffer = Buffer.from(app_state_uint8_buffer.buffer, 0, app_state_uint8_buffer.byteLength);

    let app_state = AppState.create_from_array(arrayBuffer.buffer as ArrayBuffer);

    test('members', () => {
        expect(app_state.base_ccy()).toBe(CurrencyCode.EUR);
        expect(app_state.base_media_url()).toBe(app_state_proto.baseMediaUrl);
        expect(app_state.base_static_url()).toBe(app_state_proto.baseStaticUrl);
        expect(app_state.base_url()).toBe(app_state_proto.baseUrl);
    });
});
