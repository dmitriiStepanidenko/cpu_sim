if(!self.define){let e,a={};const c=(c,s)=>(c=new URL(c+".js",s).href,a[c]||new Promise((a=>{if("document"in self){const e=document.createElement("script");e.src=c,e.onload=a,document.head.appendChild(e)}else e=c,importScripts(c),a()})).then((()=>{let e=a[c];if(!e)throw new Error(`Module ${c} didn’t register its module`);return e})));self.define=(s,i)=>{const b=e||("document"in self?document.currentScript.src:"")||location.href;if(a[b])return;let r={};const d=e=>c(e,b),n={module:{uri:b},exports:r,require:d};a[b]=Promise.all(s.map((e=>n[e]||d(e)))).then((e=>(i(...e),r)))}}define(["./workbox-27b29e6f"],(function(e){"use strict";self.addEventListener("message",(e=>{e.data&&"SKIP_WAITING"===e.data.type&&self.skipWaiting()})),e.precacheAndRoute([{url:"_app/immutable/assets/_layout.fa897784.css",revision:"71693c7903e5b98a5d7dfcb4f0d8c33f"},{url:"_app/immutable/assets/_page.bc6ec126.css",revision:"f7e7098dc358300650c6cef863e31ba9"},{url:"_app/immutable/assets/0.d487229c.css",revision:"a3ccfcd1c7dccaa5f737c2a8a7822d29"},{url:"_app/immutable/assets/2.3c587ba2.css",revision:"25d1cb89f9300ee24c953cbc38708f69"},{url:"_app/immutable/assets/android-chrome-512x512.38fd8103.webp",revision:"c92e49d7de837f0bcf8cb63e92f29760"},{url:"_app/immutable/assets/android-chrome-512x512.4b88dcbd.png",revision:"a73224743827890cee2021e7217afb07"},{url:"_app/immutable/assets/android-chrome-512x512.a8179dc5.webp",revision:"c12ab535a30cb615dd552d9f10309c59"},{url:"_app/immutable/assets/android-chrome-512x512.d38fa855.png",revision:"c83585c9eeb2e98587a9d887bfe7b776"},{url:"_app/immutable/chunks/index.a1997104.js",revision:"b229ce183cfef997cc98273a59c7a66e"},{url:"_app/immutable/chunks/index.e8359eb4.js",revision:"151f7c9491b8eb03eb2a1bf2856abfb2"},{url:"_app/immutable/chunks/paths.17b5e1cd.js",revision:"577dcbd6cf26a44f16e273e984ce4ac2"},{url:"_app/immutable/chunks/scheduler.1c3949fc.js",revision:"e69fc9c549cd9e9a425aec2aaa46d3a2"},{url:"_app/immutable/chunks/singletons.c3fa8353.js",revision:"7f9723cef4a0adb4e3b7e187a09fdd25"},{url:"_app/immutable/chunks/Toaster.3eab06c8.js",revision:"db8f8def285664f1b3efd62da18412da"},{url:"_app/immutable/entry/app.a06e2f15.js",revision:"86fb870d716e8f38c9b75d4d46a6ac1f"},{url:"_app/immutable/entry/start.fec88ab5.js",revision:"2d46b5c03281bba89b66b59bc4cb4554"},{url:"_app/immutable/nodes/0.0c423fdf.js",revision:"a1a5ba3c8ce7b6925a9ef7efca40e03b"},{url:"_app/immutable/nodes/1.5bfacbc0.js",revision:"4d8f570e83110a7bc6c7601ac63fbc94"},{url:"_app/immutable/nodes/2.ad808660.js",revision:"3b9a9a89ba4c89414ad8c31d8ba11228"},{url:"android-chrome-192x192.png",revision:"40f0e16d8300cce2fe9677ccb8363122"},{url:"android-chrome-512x512.png",revision:"46877f2ba7a4f2e1ed7e86f768751a61"},{url:"apple-touch-icon.png",revision:"1d76718b243b1fabb7a551cf47b62618"},{url:"favicon-16x16.png",revision:"123a80e6412d44aba92481d6ecc50d2e"},{url:"favicon-32x32.png",revision:"3244284e52f7cbb3178f7595455498fc"},{url:"favicon.ico",revision:"c5bf481d7841d2c56884bede020f6cd2"},{url:"registerSW.js",revision:"402b66900e731ca748771b6fc5e7a068"},{url:"/sveltekit-github-pages/",revision:"7bba7f04c2b68085ce1cc40af5bd930c"},{url:"manifest.webmanifest",revision:"a8dfc004a013f12f1399a42718d254e4"}],{}),e.cleanupOutdatedCaches(),e.registerRoute(new e.NavigationRoute(e.createHandlerBoundToURL("/sveltekit-github-pages/")))}));