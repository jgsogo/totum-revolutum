import { get_past_transactions } from '$lib/commands';
import { type MainContext, type Account, type Transaction, LastTransactionsRequest } from '../../../../../../../../models/src-js';
import { MovementDirection } from '../../../../../../../../models/src-js/movement';

/** @type {import('./$types').LayoutLoad} */
export async function load({ url, parent }) {
    const { main_context }: { main_context: MainContext } = await parent();

    let last_transactions: Transaction[] = [];

    let from_account: Account | null = null;
    if (url.searchParams.has('from')) {
        let from = url.searchParams.get('from')!;
        let from_account_pk = parseInt(from, 10);
        from_account = main_context.find_account(from_account_pk)!;

        console.log(`from: ${from}, from_account_pk: ${from_account_pk}`);
        let last_transactions_request = new LastTransactionsRequest(from_account_pk, MovementDirection.Out);
        last_transactions = (await get_past_transactions(last_transactions_request)).transactions()
    }

    let to_account: Account | null = null;
    if (url.searchParams.has('to')) {
        let to = url.searchParams.get('to')!;
        let to_account_pk = parseInt(to, 10);
        to_account = main_context.find_account(to_account_pk)!;

        // FIXME: Is it possible to have from and to accounts?
        console.log(`to: ${to}, to_account_pk: ${to_account_pk}`);
        let last_transactions_request = new LastTransactionsRequest(to_account_pk, MovementDirection.In);
        last_transactions = (await get_past_transactions(last_transactions_request)).transactions()
    }

    return { from_account, to_account, last_transactions };
}
