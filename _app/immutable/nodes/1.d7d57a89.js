import{s as $,f as b,l as _,a as S,g as d,h as f,m as g,d as p,c as x,i as l,u as h,n as v,v as E,E as q}from"../chunks/scheduler.3db8c9a2.js";import{S as y,i as C}from"../chunks/index.107d6097.js";import{d as H}from"../chunks/singletons.1dde8e1a.js";const P=()=>{const s=H;return{page:{subscribe:s.page.subscribe},navigating:{subscribe:s.navigating.subscribe},updated:s.updated}},j={subscribe(s){return P().page.subscribe(s)}};function k(s){let t,r=s[0].status+"",o,n,i,c=s[0].error?.message+"",u;return{c(){t=b("h1"),o=_(r),n=S(),i=b("p"),u=_(c)},l(e){t=d(e,"H1",{});var a=f(t);o=g(a,r),a.forEach(p),n=x(e),i=d(e,"P",{});var m=f(i);u=g(m,c),m.forEach(p)},m(e,a){l(e,t,a),h(t,o),l(e,n,a),l(e,i,a),h(i,u)},p(e,[a]){a&1&&r!==(r=e[0].status+"")&&v(o,r),a&1&&c!==(c=e[0].error?.message+"")&&v(u,c)},i:E,o:E,d(e){e&&(p(t),p(n),p(i))}}}function w(s,t,r){let o;return q(s,j,n=>r(0,o=n)),[o]}let D=class extends y{constructor(t){super(),C(this,t,w,k,$,{})}};export{D as component};
