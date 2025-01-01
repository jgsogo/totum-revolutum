import { create, toBinary, toJson, fromBinary, fromJson, JsonObject } from "@bufbuild/protobuf";
import { AccountSchema } from './index.js';
import { describe, test, expect } from 'vitest';

describe('Account roundtrip', () => {
    let account = create(AccountSchema, { name: 'account name' });
    test('test_binary', ()=> {
        const bytes = toBinary(AccountSchema, account);
        expect(fromBinary(AccountSchema, bytes).name).toBe(account.name)
    });

    test('test_json', ()=> {
        const json: JsonObject = toJson(AccountSchema, account)! as JsonObject;
        expect(json['name']).toBe(account.name);
        expect(fromJson(AccountSchema, json).name).toBe(account.name)
    });
});
