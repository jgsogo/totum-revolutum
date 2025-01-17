import type { MainContext, Account } from '../../../../../../../../models/src-js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ url, parent }) {
    const { main_context }: { main_context: MainContext } = await parent();

    let from_account: Account | null = null;
    let from = url.searchParams.get('from');
    if (from) {
        let from_account_pk = parseInt(from, 10)
        from_account = main_context.find_account(from_account_pk)!
    }

    let to_account: Account | null = null;
    let to = url.searchParams.get('to');
    if (to) {
        let to_account_pk = parseInt(to, 10)
        to_account = main_context.find_account(to_account_pk)!
    }

    // let all_transaction_groups: TransactionGroup[] = main_context.transaction_groups();
    // let all_accounts: Account[] = main_context.accounts();
    // let all_movementtypes: MovementType[] = main_context.movement_types();
    // let base_ccy: string = app_state.base_ccy();

    return { from_account, to_account };
}
