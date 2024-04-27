import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, searchForWorkspaceRoot } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		fs: {
		  // Allow serving files from one level up to the project root
		  allow: [
			// search up for workspace root
			searchForWorkspaceRoot(process.cwd()),
			// This was needed so `js_run_devserver` was able to serve svelte files
			"../../../../../../../../"
		],
		},
	  }
});
