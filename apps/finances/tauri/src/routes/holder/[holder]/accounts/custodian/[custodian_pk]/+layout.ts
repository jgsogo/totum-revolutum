// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { error } from '@sveltejs/kit';
// import { custodian_details } from '$lib/commands';
import { Holder } from '$lib/models/Holder';
import type { Custodian } from '$lib/models/Custodian.js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent }) {
    const { accounts_by_custodian } = await parent();
    let accounts_for_custodian = accounts_by_custodian[params.custodian_pk];
    return {
        accounts_by_custodian,
        accounts_for_custodian,
    };

}
