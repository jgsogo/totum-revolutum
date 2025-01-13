import { create, toBinary, toJson, fromBinary, fromJson, JsonObject } from "@bufbuild/protobuf";
import { describe, test, expect } from 'vitest';
import { DateSchema } from '../protos/google/type/date_pb.js';
import { DateWrapper } from "./date.js";

describe('Date roundtrip', () => {
    let dec = create(DateSchema, { year: 2025, month: 1, day: 12 });

    test('test_binary', ()=> {
        const bytes = toBinary(DateSchema, dec);
        let recovered = fromBinary(DateSchema, bytes);
        expect(recovered.year).toBe(dec.year);
        expect(recovered.month).toBe(dec.month);
        expect(recovered.day).toBe(dec.day);
    });

    test('test_json', ()=> {
        const json: JsonObject = toJson(DateSchema, dec)! as JsonObject;
        expect(json['year']).toBe(dec.year);
        expect(json['month']).toBe(dec.month);
        expect(json['day']).toBe(dec.day);
        let recovered = fromJson(DateSchema, json);
        expect(recovered.year).toBe(dec.year);
        expect(recovered.month).toBe(dec.month);
        expect(recovered.day).toBe(dec.day);
    });
});

describe('DateWrapper as Date', () => {
    let proto = create(DateSchema, { year: 2025, month: 1, day: 12 });
    let value = new DateWrapper(proto);

    test('as_date', ()=> {
        expect(value.as_date()).toStrictEqual(new Date(2025, 1, 12));
    });
});
