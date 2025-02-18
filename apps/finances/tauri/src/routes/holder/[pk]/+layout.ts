import type { Account, MainContext, Holder } from '../../../../models/src-js';
import { get_holder_context } from '$lib/commands';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent }) {
    const { main_context }: { main_context: MainContext } = await parent();

    // Filter all the accounts for the given custodian
    let holder_pk = parseInt(params.pk, 10);
    let holder_accounts = main_context.accounts().filter((acc: Account) => acc.holders().find((h: Holder) => h.pk() === holder_pk) !== undefined);
    let holder_context = await get_holder_context(holder_pk);

    return {
        holder_accounts,
        holder_context,
    };
}
