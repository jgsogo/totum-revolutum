import {sayHello} from './index';

describe('concatenate module', () => {
    test('test_concatenate', ()=> {
        expect(sayHello()).toBe('Hi')
    });
});
