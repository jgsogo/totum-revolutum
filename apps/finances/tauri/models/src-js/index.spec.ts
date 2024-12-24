import { create, toBinary, toJson, fromBinary, fromJson } from "@bufbuild/protobuf";
import { AccountSchema} from './index';


describe('Account roundtrip', () => {
    let account = create(AccountSchema, { name: 'account name' });
    test('test_binary', ()=> {
        const bytes = toBinary(AccountSchema, account);
        expect(fromBinary(AccountSchema, bytes).name).toBe(account.name)
    });

    test('test_json', ()=> {
        const json = toJson(AccountSchema, account);
        expect(json['name']).toBe(account.name);
        expect(fromJson(AccountSchema, json).name).toBe(account.name)
    });
});
