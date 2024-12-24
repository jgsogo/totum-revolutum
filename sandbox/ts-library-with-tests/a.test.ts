import {A} from './a'

describe('concatenate module', () => {
    test('test_concatenate', ()=> {
        expect(A).toBe('a')
        // let msg = Account.fromJson({ name: 'account name' });
        // let account = Account.fromBinary(msg.toBinary());
        // expect(account.name).toBe('account name')
    });
});
