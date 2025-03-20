import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import vitePluginWasm from 'vite-plugin-wasm';

export default defineConfig({
	plugins: [sveltekit(), vitePluginWasm()]
});
