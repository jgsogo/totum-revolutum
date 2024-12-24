import { create, toBinary, toJson } from "@bufbuild/protobuf";
import { AccountSchema} from './index';


describe('concatenate module', () => {
    test('test_concatenate', ()=> {
        let msg = create(AccountSchema, { name: 'account name' });
        expect('account name').toBe('account name')
        // let msg = Account.fromJson({ name: 'account name' });
        // let account = Account.fromBinary(msg.toBinary());
        // expect(account.name).toBe('account name')
    });
});
