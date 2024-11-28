// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

import { error } from '@sveltejs/kit';
import { holders, get_base_media_url } from '$lib/commands';
import { Holder } from '$lib/models/Holder';

/** @type {import('./$types').LayoutLoad} */
export async function load({ depends }) {
    depends('invalidate:refresh');

	try {
		let all_holders: Holder[] = await holders();
		let base_media_url = await get_base_media_url();
		return {
			holders: all_holders,
			base_media_url,
		};
	}
	catch(e) {
		error(500, `${e}`);
	}
}
