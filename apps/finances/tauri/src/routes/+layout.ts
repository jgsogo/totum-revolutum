// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

import { error } from '@sveltejs/kit';
import { get_all_holders, get_base_media_url, get_base_static_url, get_base_url } from '$lib/commands';
import { Holder } from '$lib/models/Holder';

/** @type {import('./$types').LayoutLoad} */
export async function load({ depends }) {
    depends('invalidate:refresh');

	try {
		let all_holders: Holder[] = await get_all_holders();
		let base_media_url = await get_base_media_url();
		let base_static_url = await get_base_static_url();
		let base_url = await get_base_url();
		return {
			all_holders,
			base_url,
			base_media_url,
			base_static_url,
		};
	}
	catch(e) {
		error(500, `${e}`);
	}
}
