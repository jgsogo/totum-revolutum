import { error } from '@sveltejs/kit';
import { get_holder_context } from '$lib/commands';
import { SidebarEntry } from "$lib/components/SidebarMenu/SidebarEntry.js"
import {
	LockSolid,
	ChartMixedDollarSolid,
	CashSolid,
	LandmarkSolid,
	ClipboardSolid,

} from 'flowbite-svelte-icons';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, depends }) {
	// TODO: Choose better default, see https://github.com/jgsogo/totum-revolutum/issues/637
	let holder_pk = params.holder === '<unknown>' ? 1 : parseInt(params.holder, 10);

	try {
		let holder_context = await get_holder_context(holder_pk);
		let holder = holder_context.holder();

		let menu = []
		// Menu - All accounts
		menu.push(new SidebarEntry(`All (${holder_context.accounts().length})`, ClipboardSolid, `/holder/${holder.pk()}/accounts/all`));

		// Menu - custodians
		let accounts_by_custodian = holder_context.grouped_by_custodian();
		let custodians_entry = new SidebarEntry(`By custodian (${accounts_by_custodian.size})`, LandmarkSolid);
		for (const [_, accounts] of accounts_by_custodian) {
			let custodian = accounts[0].custodian();
			custodians_entry.addChildren(`${custodian.name()} (${accounts.length})`, `/holder/${holder.pk()}/accounts/custodian/${custodian.pk() as unknown as number}`);
		}
		menu.push(custodians_entry);

		// Menu - other entries
		menu.push(new SidebarEntry(`Savings (${holder_context.savings_accounts().length})`, CashSolid, `/holder/${holder.pk()}/accounts/savings`));
		menu.push(new SidebarEntry(`Investment (${holder_context.investments_accounts().length})`, ChartMixedDollarSolid, `/holder/${holder.pk()}/accounts/investment`));
		menu.push(new SidebarEntry(`Retirement (${holder_context.retirement_accounts().length})`, LockSolid, `/holder/${holder.pk()}/accounts/retirement`));

		return {
			holder_context,
			menu,
		};
	}
	catch (e) {
		error(404, `Cannot get URL: ${JSON.stringify(e)}`);
	}
}
