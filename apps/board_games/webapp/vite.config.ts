import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, searchForWorkspaceRoot } from 'vite';


export default defineConfig({
	plugins: [tailwindcss(), sveltekit()],
	server: {
		fs: {
			allow: [
				searchForWorkspaceRoot(process.cwd()),
				"/private/var/tmp/_bazel_jgsogo/", // TODO: Bazel deploys the node_modules inside this tmp folders
			]
		},
	},
});
