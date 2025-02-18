import { get_account_context } from '$lib/commands';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
    // Find the account for the input params
    let account_pk: number = parseInt(params.pk, 10);

    let account_context = await get_account_context(account_pk);
    return {
        account_context,
    };

}
