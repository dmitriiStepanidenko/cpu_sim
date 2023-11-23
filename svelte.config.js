import { preprocessMeltUI } from '@melt-ui/pp';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';
import preprocess from 'svelte-preprocess'
import importCSSPreprocess from './importCSSPreprocessor.js'

/** @type {import('@sveltejs/kit').Config}*/
const config = {
  preprocess: [
    // @ts-ignore
    importCSSPreprocess(),
    preprocess(),
    vitePreprocess(),
    preprocessMeltUI()
  ],
  kit: {
    adapter: adapter(),
    paths: {
      base: process.env.NODE_ENV === 'production' ? '/sveltekit-github-pages' : '',
    }
  }
};
export default config;
