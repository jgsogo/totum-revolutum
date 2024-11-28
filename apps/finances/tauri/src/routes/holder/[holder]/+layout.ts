import { error } from '@sveltejs/kit';
import { holder_details, all_accounts, savings_accounts, retirement_accounts, investment_accounts } from '$lib/commands';
import {SidebarEntry} from "$lib/components/SidebarMenu/SidebarEntry.js"
import {Account} from '$lib/models/Account.js'

import {
	LockSolid,
	ChartMixedDollarSolid,
	CashSolid,
	LandmarkSolid,

} from 'flowbite-svelte-icons';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
	// TODO: Choose better default, see https://github.com/jgsogo/totum-revolutum/issues/637
	let holder_pk = params.holder === '<unknown>' ? 1 : parseInt(params.holder, 10);

	try {
		let holder = await holder_details(holder_pk);

		let menu = []

		// Menu - custodians
		let all_accounts_list: Account[] = await all_accounts(holder);
		let custodians = all_accounts_list.map((it) => it.custodian);
		let custodians_entry = new SidebarEntry(`Custodians (${custodians.length})`, LandmarkSolid);
		for (let it of custodians) {
			custodians_entry.addChildren(it.name, `/holder/${holder.pk}/custodian/${it.pk}`);
		}
		menu.push(custodians_entry);

		// Menu - other entries
		let savings_accounts_list = await savings_accounts(holder);
		menu.push(new SidebarEntry(`Savings (${savings_accounts_list.length})`, CashSolid));

		let investment_accounts_list = await investment_accounts(holder);
		menu.push(new SidebarEntry(`Investment (${investment_accounts_list.length})`, ChartMixedDollarSolid));

		let retirement_accounts_list = await retirement_accounts(holder);
		menu.push(new SidebarEntry(`Retirement (${retirement_accounts_list.length})`, LockSolid));


		return {
			holder,
			all_accounts_list,
			retirement_accounts_list,
			investment_accounts_list,
			menu,
		};
	}
	catch(e) {
		error(404, `Cannot get URL: ${e}`);
	}
}
