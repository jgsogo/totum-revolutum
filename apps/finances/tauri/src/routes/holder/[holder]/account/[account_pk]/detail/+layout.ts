import { error } from '@sveltejs/kit';
import { get_custodian_details, get_account_snapshots, get_account_movements } from '$lib/commands';
import { Holder } from '$lib/models/Holder';
import type { Custodian } from '$lib/models/Custodian.js';
import type { Account } from '$lib/models/Account.js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent }) {
    const { account } = await parent();

    // Get snapshots and movements
    let snapshots = await get_account_snapshots(account);
    let movements = await get_account_movements(account);

    return {
        account,
        snapshots,
        movements,
    };

}
