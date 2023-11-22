import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { enhancedImages } from '@sveltejs/enhanced-img';

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    enhancedImages(),
    sveltekit()]
});


