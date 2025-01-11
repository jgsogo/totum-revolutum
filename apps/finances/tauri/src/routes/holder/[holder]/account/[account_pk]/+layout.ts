import { error } from '@sveltejs/kit';
import { Account } from '../../../../../../models/src-js';
import {get_account_context} from '$lib/commands';
import { debug } from '@tauri-apps/plugin-log';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, depends }) {
    depends('invalidate:account');

    // Find the account for the input params
    let account_pk: number = parseInt(params.account_pk, 10);

    let account_context = await get_account_context(account_pk);
    console.log(`Account context: ${JSON.stringify(account_context, (_, v) => typeof v === 'bigint' ? v.toString() : v)}`);

    return {
        account_context,
    };

}
