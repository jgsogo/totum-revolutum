// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { error } from '@sveltejs/kit';
import { get_all_holders, get_base_media_url, get_base_static_url, get_all_accounts } from '$lib/commands';
import { Holder } from '$lib/models/Holder';
import type { Account } from '$lib/models/Account';

/** @type {import('./$types').LayoutLoad} */
export async function load({ url, parent }) {
    const { all_accounts_for_holder } = await parent();

    let from_account: Account | null = null;
    let from = url.searchParams.get('from');
    if (from) {
        let from_account_pk = parseInt(from, 10)
        from_account = all_accounts_for_holder.find((acc: Account) => {return acc.pk == from_account_pk;})!;
    }

    let to_account: Account | null = null;
    let to = url.searchParams.get('to');
    if (to) {
        let to_account_pk = parseInt(to, 10)
        to_account = all_accounts_for_holder.find((acc: Account) => {return acc.pk == to_account_pk;})!;
    }

    let all_accounts = await get_all_accounts();

    return { from_account, to_account, all_accounts };
}
