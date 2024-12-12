// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { get_all_accounts, get_all_movementtypes, get_all_transaction_groups } from '$lib/commands';
import type { Account } from '$lib/models/Account';

/** @type {import('./$types').LayoutLoad} */
export async function load({ url, parent }) {
    const { base_ccy } = await parent();

    let all_accounts = await get_all_accounts();
    let all_movementtypes_without_breadcrumbs = await get_all_movementtypes();
    let all_movementtypes = await Promise.all(all_movementtypes_without_breadcrumbs.map(async (movtype) => {
        await movtype.getBreadcrumbs(); // Populate all breadcrumbs
        return movtype;
    }))

    let from_account: Account | null = null;
    let from = url.searchParams.get('from');
    if (from) {
        let from_account_pk = parseInt(from, 10)
        from_account = all_accounts.find((acc: Account) => { return acc.pk == from_account_pk; })!;
    }

    let to_account: Account | null = null;
    let to = url.searchParams.get('to');
    if (to) {
        let to_account_pk = parseInt(to, 10)
        to_account = all_accounts.find((acc: Account) => { return acc.pk == to_account_pk; })!;
    }

    let all_transaction_groups = await get_all_transaction_groups();

    return { from_account, to_account, all_accounts, all_movementtypes, base_ccy, all_transaction_groups };
}
