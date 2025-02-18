import type { Account, MainContext } from "../../../../../../models/src-js";


/** @type {import('./$types').LayoutLoad} */
export async function load({ url, parent }) {
    const { main_context }: { main_context: MainContext } = await parent();

    let from_account: Account | null = null;
    if (url.searchParams.has('from')) {
        let from = url.searchParams.get('from')!;
        let from_account_pk = parseInt(from, 10);
        from_account = main_context.find_account(from_account_pk)!;
    }

    let to_account: Account | null = null;
    if (url.searchParams.has('to')) {
        let to = url.searchParams.get('to')!;
        let to_account_pk = parseInt(to, 10);
        to_account = main_context.find_account(to_account_pk)!;
    }

    return { from_account, to_account };
}
