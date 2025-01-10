import { error } from '@sveltejs/kit';
import { Account } from '../../../../../../models/src-js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent, depends }) {
    const { holder_context } = await parent();

    depends('invalidate:account');

    // Find the account for the input params
    let account_pk = parseInt(params.account_pk, 10)
    let account: Account | undefined = holder_context.accounts().find((acc: Account) => { return acc.pk() == account_pk; });
    if (!account) {
        error(400, "Account not accessible for this Holder");
    }

    return {
        account,
    };

}
