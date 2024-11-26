import { error } from '@sveltejs/kit';
import { sidebar_menu, holder_details } from '$lib/commands';
import { Holder } from '$lib/models/Holder';

/** @type {import('./$types').LayoutLoad} */
export async function load({ params }) {
	let holder_pk = params.holder === '<unknown>' ? 1 : parseInt(params.holder, 10);

	try {
		let holder = await holder_details(holder_pk);
		let holder_menu = await sidebar_menu("/all", holder);
		return {
			holder,
			holder_menu,
		};
	}
	catch(e) {
		error(404, `Cannot get URL: ${e}`);
	}
}
