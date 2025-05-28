import { create, toBinary, toJson, fromBinary, fromJson, JsonObject } from "@bufbuild/protobuf";
import { describe, test, expect } from 'vitest';
import { MoneySchema } from '../protos/google/type/money_pb.js';
import { Money } from "./money.js";

describe('Money proto roundtrip', () => {
    let money = create(MoneySchema, { currencyCode: 'USD', units: BigInt(1234), nanos: 0 });

    test('test_binary', ()=> {
        const bytes = toBinary(MoneySchema, money);
        let recovered = fromBinary(MoneySchema, bytes);
        expect(recovered.currencyCode).toBe(money.currencyCode);
        expect(recovered.units).toBe(money.units);
        expect(recovered.nanos).toBe(money.nanos);
    });

    test('test_json', ()=> {
        const json: JsonObject = toJson(MoneySchema, money)! as JsonObject;
        expect(json['currencyCode']).toBe(money.currencyCode);
        expect(json['units']).toBe(money.units.toString());
        expect(json['nanos']).toBe(undefined); // zero is not serialized (it's the default value)
        let recovered = fromJson(MoneySchema, json);
        expect(recovered.currencyCode).toBe(money.currencyCode);
        expect(recovered.units).toBe(money.units);
        expect(recovered.nanos).toBe(money.nanos);
    });
});


describe('Money USD units', () => {
    let proto = create(MoneySchema, { currencyCode: 'USD', units: BigInt(1234), nanos: 789_000_000 });
    let money = new Money(proto);

    test('test_as_number', ()=> {
        expect(money.amount()).toBe(1234.789);
    });

    test('test toString default', ()=> {
        expect(money.toString()).toEqual('1234,79\xa0USD'); // Use '\xa0', which is a non-breaking space
    });

    test('test toString es-ES', ()=> {
        expect(money.toString('es-ES')).toEqual('1234,79\xa0USD');
    });

    test('test toString en-US', ()=> {
        expect(money.toString('en-US')).toEqual('USD\xa01,234.79');
    });
});

describe('Money EUR units', () => {
    let proto = create(MoneySchema, { currencyCode: 'EUR', units: BigInt(1234), nanos: 789_000_000 });
    let money = new Money(proto);

    test('test_as_number', ()=> {
        expect(money.amount()).toBe(1234.789);
    });

    test('test toString default', ()=> {
        expect(money.toString()).toEqual('1234,79\xa0EUR');
    });

    test('test toString es-ES', ()=> {
        expect(money.toString('es-ES')).toEqual('1234,79\xa0EUR');
    });

    test('test toString en-US', ()=> {
        expect(money.toString('en-US')).toEqual('EUR\xa01,234.79');
    });
});

describe('Money rounded: five rounded up', () => {
    let proto = create(MoneySchema, { currencyCode: 'EUR', units: BigInt(1234), nanos: 785_000_000 });
    let money = new Money(proto);

    test('test_as_number', ()=> {
        expect(money.amount()).toBe(1234.785);
    });

    test('test toString default', ()=> {
        expect(money.toString()).toEqual('1234,79\xa0EUR');
    });

    test('test toString es-ES', ()=> {
        expect(money.toString('es-ES')).toEqual('1234,79\xa0EUR');
    });

    test('test toString en-US', ()=> {
        expect(money.toString('en-US')).toEqual('EUR\xa01,234.79');
    });
});
