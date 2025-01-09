import { error } from '@sveltejs/kit';
import { get_holder_details, get_all_accounts_for_holder, get_all_savings_accounts_for_holder, get_all_retirement_accounts_for_holder, get_all_investment_accounts_for_holder, get_holder_context } from '$lib/commands';
import {SidebarEntry} from "$lib/components/SidebarMenu/SidebarEntry.js"
import { Account } from '$lib/models/Account';
import {
	LockSolid,
	ChartMixedDollarSolid,
	CashSolid,
	LandmarkSolid,
	ClipboardSolid,

} from 'flowbite-svelte-icons';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params, depends }) {
	depends('invalidate:refresh');

	// TODO: Choose better default, see https://github.com/jgsogo/totum-revolutum/issues/637
	let holder_pk = params.holder === '<unknown>' ? 1 : parseInt(params.holder, 10);

	try {
		let holder_context = await get_holder_context(holder_pk);
		let holder = await get_holder_details(holder_pk);

		let menu = []
		let all_accounts_for_holder: Account[] = await get_all_accounts_for_holder(holder);
		// const all_accounts_for_holder: Account[] = holder_context.accounts;

		// Menu - All accounts
		menu.push(new SidebarEntry(`All (${all_accounts_for_holder.length})`, ClipboardSolid, `/holder/${holder.pk}/accounts/all`));

		// Menu - custodians
		console.log("Accounts classified by custodian");
		let accounts_by_custodian = Object.groupBy(all_accounts_for_holder, ({custodian}) => custodian?.pk as unknown as number);
		// let accounts_by_custodian = Object.groupBy(all_accounts_for_holder, ({custodian}) => custodian.pk);
		let custodians_entry = new SidebarEntry(`By custodian (${Object.entries(accounts_by_custodian).length})`, LandmarkSolid);
		for (const [_, value] of Object.entries(accounts_by_custodian)) {
			if (value) {
				const custodian = value[0].custodian!;
				custodians_entry.addChildren(`${custodian.name} (${value?.length})`, `/holder/${holder.pk}/accounts/custodian/${custodian.pk}`);
			}
		}
		menu.push(custodians_entry);

		// Menu - other entries
		// const all_savings_accounts_for_holder = all_accounts_for_holder.filter((value: Account) => {
		// 	return main_context.account_types.find(acc_type => acc_type.pk === value.account_type_pk)!.category == AccountCategory.Savings
		// });
		let all_savings_accounts_for_holder = await get_all_savings_accounts_for_holder(holder);
		menu.push(new SidebarEntry(`Savings (${all_savings_accounts_for_holder.length})`, CashSolid, `/holder/${holder.pk}/accounts/savings`));

		let all_investment_accounts_for_holder = await get_all_investment_accounts_for_holder(holder);
		menu.push(new SidebarEntry(`Investment (${all_investment_accounts_for_holder.length})`, ChartMixedDollarSolid, `/holder/${holder.pk}/accounts/investment`));

		let all_retirement_accounts_for_holder = await get_all_retirement_accounts_for_holder(holder);
		menu.push(new SidebarEntry(`Retirement (${all_retirement_accounts_for_holder.length})`, LockSolid, `/holder/${holder.pk}/accounts/retirement`));


		return {
			holder,
			all_accounts_for_holder,
			all_retirement_accounts_for_holder,
			all_investment_accounts_for_holder,
			all_savings_accounts_for_holder,
			accounts_by_custodian,
			menu,
		};
	}
	catch(e) {
		error(404, `Cannot get URL: ${e}`);
	}
}
