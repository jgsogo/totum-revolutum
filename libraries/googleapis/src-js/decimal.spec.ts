import { create, toBinary, toJson, fromBinary, fromJson, JsonObject } from "@bufbuild/protobuf";
import { describe, test, expect } from 'vitest';
import { DecimalSchema } from '../protos/google/type/decimal_pb.js';
import { Decimal } from "./decimal.js";

describe('Decimal roundtrip', () => {
    let dec = create(DecimalSchema, { value: '1234.35' });

    test('test_binary', ()=> {
        const bytes = toBinary(DecimalSchema, dec);
        let recovered = fromBinary(DecimalSchema, bytes);
        expect(recovered.value).toBe(dec.value);
    });

    test('test_json', ()=> {
        const json: JsonObject = toJson(DecimalSchema, dec)! as JsonObject;
        expect(json['value']).toBe(dec.value);
        let recovered = fromJson(DecimalSchema, json);
        expect(recovered.value).toBe(dec.value);
    });
});

describe('Decimal as number', () => {

    test('as_number', ()=> {
        let proto = create(DecimalSchema, { value: '1234.35' });
        let dec = new Decimal(proto);
        expect(dec.as_number()).toBe(1234.35);
    });


    test('as_number', ()=> {
        let proto = create(DecimalSchema, { value: '45987349857634085409857349856430985' });
        let dec = new Decimal(proto);
        expect(dec.as_number()).toBe(4.598734985763409e+34);
    });
});
