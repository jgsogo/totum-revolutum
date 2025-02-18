import type { Account, MainContext } from '../../../../models/src-js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent }) {
    const { main_context }: { main_context: MainContext } = await parent();

    // Filter all the accounts for the given custodian
    let custodian_pk = parseInt(params.pk, 10);
    let custodian_accounts = main_context.accounts().filter((acc: Account) => acc.custodian().pk() === custodian_pk);
    let custodian = custodian_accounts[0].custodian();

    return {
        custodian_accounts,
        custodian,
    };
}
