if(!self.define){let e,s={};const i=(i,a)=>(i=new URL(i+".js",a).href,s[i]||new Promise((s=>{if("document"in self){const e=document.createElement("script");e.src=i,e.onload=s,document.head.appendChild(e)}else e=i,importScripts(i),s()})).then((()=>{let e=s[i];if(!e)throw new Error(`Module ${i} didn’t register its module`);return e})));self.define=(a,c)=>{const r=e||("document"in self?document.currentScript.src:"")||location.href;if(s[r])return;let b={};const d=e=>i(e,r),n={module:{uri:r},exports:b,require:d};s[r]=Promise.all(a.map((e=>n[e]||d(e)))).then((e=>(c(...e),b)))}}define(["./workbox-27b29e6f"],(function(e){"use strict";self.addEventListener("message",(e=>{e.data&&"SKIP_WAITING"===e.data.type&&self.skipWaiting()})),e.precacheAndRoute([{url:"_app/immutable/assets/_layout.8a0b468c.css",revision:"22646732c73333bc1a17b5681b719151"},{url:"_app/immutable/assets/_page.cf622d33.css",revision:"5527cebd1569512e817ea9e4e9e94b2f"},{url:"_app/immutable/assets/0.f43da789.css",revision:"ab1e500b655ea6c7a21caa6ec5b1ca4e"},{url:"_app/immutable/assets/2.8f3e9f72.css",revision:"0b411777172b3356d8f50451a040a1bd"},{url:"_app/immutable/assets/android-chrome-512x512.38fd8103.webp",revision:"c92e49d7de837f0bcf8cb63e92f29760"},{url:"_app/immutable/assets/android-chrome-512x512.4b88dcbd.png",revision:"a73224743827890cee2021e7217afb07"},{url:"_app/immutable/assets/android-chrome-512x512.a8179dc5.webp",revision:"c12ab535a30cb615dd552d9f10309c59"},{url:"_app/immutable/assets/android-chrome-512x512.d38fa855.png",revision:"c83585c9eeb2e98587a9d887bfe7b776"},{url:"_app/immutable/chunks/index.107d6097.js",revision:"b6169646beb40cf9442358e1604e9a72"},{url:"_app/immutable/chunks/index.506c2f87.js",revision:"d0bf73dffd807b01cf66aac4e56e2006"},{url:"_app/immutable/chunks/paths.ba1dc80e.js",revision:"3e1b4c88e93f87394e5ed3474b2564f0"},{url:"_app/immutable/chunks/scheduler.3db8c9a2.js",revision:"31278bc1bb9795e5a135709df81905fc"},{url:"_app/immutable/chunks/singletons.86fd61de.js",revision:"38297464fe00ee97c05c5cbc5951928c"},{url:"_app/immutable/chunks/Toaster.34c19dde.js",revision:"fbe3492fb8d4db493a813667906ec50e"},{url:"_app/immutable/entry/app.e6132cbd.js",revision:"01eda8f090bbd7113b0ec0650454ad84"},{url:"_app/immutable/entry/start.d15c093b.js",revision:"f0487c9363cdbc40b75158ced0fce1fd"},{url:"_app/immutable/nodes/0.f2427df1.js",revision:"258582e584e19704e65b82d58f241c69"},{url:"_app/immutable/nodes/1.03d205f6.js",revision:"9e1fc3d928d9b52858abff1175f216e8"},{url:"_app/immutable/nodes/2.542310bf.js",revision:"c3547e00e53ddb633373595142fe8a90"},{url:"android-chrome-192x192.png",revision:"40f0e16d8300cce2fe9677ccb8363122"},{url:"android-chrome-512x512.png",revision:"46877f2ba7a4f2e1ed7e86f768751a61"},{url:"apple-touch-icon.png",revision:"1d76718b243b1fabb7a551cf47b62618"},{url:"favicon-16x16.png",revision:"123a80e6412d44aba92481d6ecc50d2e"},{url:"favicon-32x32.png",revision:"3244284e52f7cbb3178f7595455498fc"},{url:"favicon.ico",revision:"c5bf481d7841d2c56884bede020f6cd2"},{url:"registerSW.js",revision:"402b66900e731ca748771b6fc5e7a068"},{url:"/sveltekit-github-pages/",revision:"191aee8ef4647dde91761a6a1a19523d"},{url:"manifest.webmanifest",revision:"a8dfc004a013f12f1399a42718d254e4"}],{}),e.cleanupOutdatedCaches(),e.registerRoute(new e.NavigationRoute(e.createHandlerBoundToURL("/sveltekit-github-pages/")))}));
