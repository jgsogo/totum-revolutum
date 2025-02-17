import { create, toBinary, toJson, fromBinary, fromJson, JsonObject } from "@bufbuild/protobuf";
import { describe, test, expect } from 'vitest';
import { DateSchema } from '../protos/google/type/date_pb.js';
import { DateWrapper, sort_date_wrapper } from "./date.js";

describe('Date roundtrip', () => {
    let dec = create(DateSchema, { year: 2025, month: 1, day: 12 });

    test('test_binary', () => {
        const bytes = toBinary(DateSchema, dec);
        let recovered = fromBinary(DateSchema, bytes);
        expect(recovered.year).toBe(dec.year);
        expect(recovered.month).toBe(dec.month);
        expect(recovered.day).toBe(dec.day);
    });

    test('test_json', () => {
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


describe('DateWrapper create_from_yyyy_mm_dd', () => {
    let value = DateWrapper.create_from_yyyy_mm_dd(2025, 1, 12);

    test('toString', () => {
        expect(value.toString()).toBe("2025-01-12");
    });

});

describe('DateWrapper create_from_date', () => {
    let value = DateWrapper.create_from_date(new Date(2025, 0, 1));

    test('toString', () => {
        expect(value.toString()).toBe("2025-01-01");
    });

});

// describe('DateWrapper from Date UTC-1', () => {
//     let date = new Date("2025-01-19T00:00:00-0030");
//     let value = DateWrapper.create_from_date_utc(date);
//     test('toString', () => {
//         expect(value.toString()).toBe("2025-01-19");
//     });
// });

// describe('DateWrapper from Date UTC+1', () => {
//     let date = new Date("2025-01-19T00:00:00+0030");
//     let value = DateWrapper.create_from_date_utc(date);
//     test('toString', () => {
//         expect(value.toString()).toBe("2025-01-18");
//     });
// });


describe('sort_date_wrapper', () => {
    let d1 = DateWrapper.create_from_yyyy_mm_dd(2025, 1, 20);
    let d2 = DateWrapper.create_from_yyyy_mm_dd(2025, 1, 20);
    let d3 = DateWrapper.create_from_yyyy_mm_dd(2025, 1, 21);
    let d4 = DateWrapper.create_from_yyyy_mm_dd(2025, 2, 21);
    let d5 = DateWrapper.create_from_yyyy_mm_dd(2026, 2, 21);

    test('equals', () => {
        expect(sort_date_wrapper(d1, d2)).toBe(0);
        expect(sort_date_wrapper(d2, d1)).toBe(0);
    });
    test('day diff', () => {
        expect(sort_date_wrapper(d2, d3)).toBeLessThan(0);
        expect(sort_date_wrapper(d3, d2)).toBeGreaterThan(0);
    });
    test('month diff', () => {
        expect(sort_date_wrapper(d3, d4)).toBeLessThan(0);
        expect(sort_date_wrapper(d4, d3)).toBeGreaterThan(0);
    });
    test('year diff', () => {
        expect(sort_date_wrapper(d4, d5)).toBeLessThan(0);
        expect(sort_date_wrapper(d5, d4)).toBeGreaterThan(0);
    });

});
