// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

import { error } from '@sveltejs/kit';
import { get_all_holders, get_app_config } from '$lib/commands';
import { Holder } from '$lib/models/Holder';
import type { AppConfig } from '../../models/protos/app_config_pb.js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ depends }) {
    depends('invalidate:refresh');

	try {
		let app_config: AppConfig = await get_app_config();
		let all_holders: Holder[] = await get_all_holders();
		return {
			app_config,
			all_holders,
		};
	}
	catch(e) {
		error(500, `${e}`);
	}
}
