if(!self.define){let e,a={};const s=(s,i)=>(s=new URL(s+".js",i).href,a[s]||new Promise((a=>{if("document"in self){const e=document.createElement("script");e.src=s,e.onload=a,document.head.appendChild(e)}else e=s,importScripts(s),a()})).then((()=>{let e=a[s];if(!e)throw new Error(`Module ${s} didn’t register its module`);return e})));self.define=(i,c)=>{const r=e||("document"in self?document.currentScript.src:"")||location.href;if(a[r])return;let f={};const n=e=>s(e,r),b={module:{uri:r},exports:f,require:n};a[r]=Promise.all(i.map((e=>b[e]||n(e)))).then((e=>(c(...e),f)))}}define(["./workbox-27b29e6f"],(function(e){"use strict";self.addEventListener("message",(e=>{e.data&&"SKIP_WAITING"===e.data.type&&self.skipWaiting()})),e.precacheAndRoute([{url:"_app/immutable/assets/_layout.fa897784.css",revision:"71693c7903e5b98a5d7dfcb4f0d8c33f"},{url:"_app/immutable/assets/_page.bc6ec126.css",revision:"f7e7098dc358300650c6cef863e31ba9"},{url:"_app/immutable/assets/0.d487229c.css",revision:"a3ccfcd1c7dccaa5f737c2a8a7822d29"},{url:"_app/immutable/assets/2.3c587ba2.css",revision:"25d1cb89f9300ee24c953cbc38708f69"},{url:"_app/immutable/assets/android-chrome-512x512.38fd8103.webp",revision:"c92e49d7de837f0bcf8cb63e92f29760"},{url:"_app/immutable/assets/android-chrome-512x512.4b88dcbd.png",revision:"a73224743827890cee2021e7217afb07"},{url:"_app/immutable/assets/android-chrome-512x512.a8179dc5.webp",revision:"c12ab535a30cb615dd552d9f10309c59"},{url:"_app/immutable/assets/android-chrome-512x512.d38fa855.png",revision:"c83585c9eeb2e98587a9d887bfe7b776"},{url:"_app/immutable/chunks/index.a1997104.js",revision:"b229ce183cfef997cc98273a59c7a66e"},{url:"_app/immutable/chunks/index.e8359eb4.js",revision:"151f7c9491b8eb03eb2a1bf2856abfb2"},{url:"_app/immutable/chunks/paths.6411805e.js",revision:"a67327e1026baebd017a8ffda80e0bd7"},{url:"_app/immutable/chunks/scheduler.1c3949fc.js",revision:"e69fc9c549cd9e9a425aec2aaa46d3a2"},{url:"_app/immutable/chunks/singletons.b99f4aa3.js",revision:"7843ef8253207586d085ff72ac077307"},{url:"_app/immutable/chunks/Toaster.df927812.js",revision:"ca272fcd3693ca38bb5b27f8189f9fa8"},{url:"_app/immutable/entry/app.39c04fd0.js",revision:"b2eaa9147543d79d2a95a6605fd0a302"},{url:"_app/immutable/entry/start.321ef94c.js",revision:"b3693d24ca161d0b514312e171d7a3d0"},{url:"_app/immutable/nodes/0.a256d853.js",revision:"0c521ce667f38ac9f658a3b4f2a42c5d"},{url:"_app/immutable/nodes/1.98b828be.js",revision:"a5b5f7a5155404714b3360df84515856"},{url:"_app/immutable/nodes/2.8def8f45.js",revision:"38132343af24161bdb0388e493fe5bfc"},{url:"android-chrome-192x192.png",revision:"40f0e16d8300cce2fe9677ccb8363122"},{url:"android-chrome-512x512.png",revision:"46877f2ba7a4f2e1ed7e86f768751a61"},{url:"apple-touch-icon.png",revision:"1d76718b243b1fabb7a551cf47b62618"},{url:"favicon-16x16.png",revision:"123a80e6412d44aba92481d6ecc50d2e"},{url:"favicon-32x32.png",revision:"3244284e52f7cbb3178f7595455498fc"},{url:"favicon.ico",revision:"c5bf481d7841d2c56884bede020f6cd2"},{url:"registerSW.js",revision:"402b66900e731ca748771b6fc5e7a068"},{url:"/sveltekit-github-pages/",revision:"e21977e99e364487e57f13146333718a"},{url:"manifest.webmanifest",revision:"a8dfc004a013f12f1399a42718d254e4"}],{}),e.cleanupOutdatedCaches(),e.registerRoute(new e.NavigationRoute(e.createHandlerBoundToURL("/sveltekit-github-pages/")))}));
