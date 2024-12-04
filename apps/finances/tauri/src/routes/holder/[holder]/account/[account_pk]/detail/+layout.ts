import { error } from '@sveltejs/kit';
import { custodian_details, account_snapshots, account_movements } from '$lib/commands';
import { Holder } from '$lib/models/Holder';
import type { Custodian } from '$lib/models/Custodian.js';
import type { Account } from '$lib/models/Account.js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent, depends }) {
    depends('invalidate:account');

    const { all_accounts_for_holder } = await parent();

    // Find the account for the input params
    let account_pk = parseInt(params.account_pk, 10)
    let account: Account | undefined = all_accounts_for_holder.find((acc: Account) => {return acc.pk == account_pk;});
    if (!account) {
        error(400, "Account not accessible for this Holder");
    }

    // Get snapshots and movements
    let snapshots = await account_snapshots(account);
    let movements = await account_movements(account);

    return {
        account,
        snapshots,
        movements,
    };

}
