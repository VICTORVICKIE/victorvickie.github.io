import { svelte } from '@sveltejs/vite-plugin-svelte';
import path from "path";
import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

export default defineConfig({
    plugins: [svelte(), wasmPack('./minimax')],
    build: {
        outDir: './docs'
    },
    resolve: {
        alias: {
            '$lib': path.resolve('./src/lib')
        }
    }
});
