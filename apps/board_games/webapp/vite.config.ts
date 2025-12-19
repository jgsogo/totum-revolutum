import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, searchForWorkspaceRoot } from 'vite';
import { viteStaticCopy } from 'vite-plugin-static-copy'
import path from 'path';

export default defineConfig({
	assetsInclude: ["**/*.bin", "**/*.svg"], // This works, it copies the 'favicon.svg'
	plugins: [tailwindcss(), sveltekit(),
		// Collect files as assets (available to the client side of the application)
	viteStaticCopy({
		targets: [
			{
				src: [
					// USA
					path.resolve(__dirname, '../games/ticket_to_ride/maps/usa') + '/routes.svg',
					path.resolve(__dirname, '../games/ticket_to_ride/maps/usa') + '/background.svg',
					path.resolve(__dirname, '../games/ticket_to_ride/maps/usa') + '/data.bin',
				],
				dest: './ticket_to_ride/maps/usa',
			},
		],
	})
	],
	server: {
		fs: {
			allow: [
				searchForWorkspaceRoot(process.cwd()),
				"/private/var/tmp/_bazel_jgsogo/", // TODO: Bazel deploys the node_modules inside this tmp folders
			]
		},
	},
});
