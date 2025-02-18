// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

import { error } from '@sveltejs/kit';
import { get_app_state, get_main_context } from '$lib/commands';
import type { AppState, MainContext } from '../../models/src-js/index.js';

/** @type {import('./$types').LayoutLoad} */
export async function load({ }) {
	try {
		let main_context: MainContext = await get_main_context();
		let app_state: AppState = await get_app_state();
		return {
			app_state,
			main_context,
		};
	}
	catch (e) {
		error(500, `${e}`);
	}
}
