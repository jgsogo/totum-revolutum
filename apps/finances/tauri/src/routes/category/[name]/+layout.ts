import { type Account, type MainContext, account_category_from_str, AccountCategory } from '../../../../models/src-js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, parent }) {
    const { main_context }: { main_context: MainContext } = await parent();

    // Filter all the accounts for the given category
    let category: AccountCategory = account_category_from_str(params.name)!;
    let category_accounts = main_context.accounts().filter((acc: Account) => acc.type().category() === category);

    return {
        category_accounts,
        category,
    };
}
