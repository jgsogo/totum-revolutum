// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { error } from '@sveltejs/kit';
import { get_custodian_details } from '$lib/commands';
import { Holder } from '$lib/models/Holder';
import type { Custodian } from '$lib/models/Custodian.js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent }) {
    const { accounts_by_custodian } = await parent();
    let custodian_pk = parseInt(params.custodian_pk, 10)
    let custodian = await get_custodian_details(custodian_pk);

    let accounts_for_custodian = accounts_by_custodian[custodian_pk];
    return {
        accounts_for_custodian,
        custodian,
    };

}
