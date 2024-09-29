import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';
import { invoke } from "@tauri-apps/api/core";

export const load: PageLoad = async ({ params }) => {
    const account = await invoke("detail_command", {pk: Number(params.pk)});
	return { account };
};
