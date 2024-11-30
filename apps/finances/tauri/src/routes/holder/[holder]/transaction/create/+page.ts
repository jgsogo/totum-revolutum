// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
import { error } from '@sveltejs/kit';
import { holders, get_base_media_url, get_base_static_url } from '$lib/commands';
import { Holder } from '$lib/models/Holder';

/** @type {import('./$types').LayoutLoad} */
export async function load({ url }) {
    let from = url.searchParams.get('from');
    let to = url.searchParams.get('to');
    return { from,to };
}
