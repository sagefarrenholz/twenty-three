"use strict";(self.webpackChunkmy_webpack_project=self.webpackChunkmy_webpack_project||[]).push([[605],{605:(n,e,t)=>{t.a(n,(async(n,_)=>{try{t.r(e),t.d(e,{__wbg_addEventListener_e167f012cbedfa4e:()=>c.O,__wbg_clearRect_384c24b287b30369:()=>c.k,__wbg_code_01dc6af887ca9ecb:()=>c.pU,__wbg_fillRect_a5a5da573f0412b5:()=>c.Sw,__wbg_fillText_77c07f67110c7270:()=>c.bR,__wbg_getContext_69ec873410cbba3c:()=>c.Ck,__wbg_height_6fb32e51e54037ae:()=>c.aJ,__wbg_instanceof_CanvasRenderingContext2d_a0c4f0da6392b8ca:()=>c.GP,__wbg_instanceof_KeyboardEvent_cb701ff8e9ff53cb:()=>c.HG,__wbg_log_b103404cc5920657:()=>c.T6,__wbg_now_b7a162010a9e75b4:()=>c.cB,__wbg_set_wasm:()=>c.lI,__wbg_setfillStyle_98060f7b257936ba:()=>c.Qg,__wbg_setfont_931e1f36bec6a342:()=>c.CI,__wbg_width_53a5bd0268e99485:()=>c.F2,__wbindgen_cb_drop:()=>c.LC,__wbindgen_closure_wrapper33:()=>c.XU,__wbindgen_debug_string:()=>c.rl,__wbindgen_error_new:()=>c.Rj,__wbindgen_object_drop_ref:()=>c.bk,__wbindgen_string_new:()=>c.yc,__wbindgen_throw:()=>c.Qn,load_handlers:()=>c.Vt,main_js:()=>c.GT,render:()=>c.XX,render_editor:()=>c.$O});var r=t(650),c=t(903),o=n([r]);r=(o.then?(await o)():o)[0],(0,c.lI)(r),r.__wbindgen_start(),_()}catch(n){_(n)}}))},903:(n,e,t)=>{let _;function r(n){_=n}t.d(e,{$O:()=>v,CI:()=>D,Ck:()=>L,F2:()=>F,GP:()=>X,GT:()=>R,HG:()=>H,LC:()=>S,O:()=>$,Qg:()=>A,Qn:()=>q,Rj:()=>E,Sw:()=>B,T6:()=>U,Vt:()=>I,XU:()=>z,XX:()=>x,aJ:()=>G,bR:()=>J,bk:()=>T,cB:()=>P,k:()=>Q,lI:()=>r,pU:()=>K,rl:()=>V,yc:()=>O});const c=new Array(128).fill(void 0);function o(n){return c[n]}c.push(void 0,null,!0,!1);let i=c.length;function a(n){const e=o(n);return function(n){n<132||(c[n]=i,i=n)}(n),e}let b=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});b.decode();let f=null;function d(){return null!==f&&0!==f.byteLength||(f=new Uint8Array(_.memory.buffer)),f}function u(n,e){return n>>>=0,b.decode(d().subarray(n,n+e))}function l(n){i===c.length&&c.push(c.length+1);const e=i;return i=c[e],c[e]=n,e}function g(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let t="[";e>0&&(t+=g(n[0]));for(let _=1;_<e;_++)t+=", "+g(n[_]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(n));let _;if(!(t.length>1))return toString.call(n);if(_=t[1],"Object"==_)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:_}let s=0,w=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const y="function"==typeof w.encodeInto?function(n,e){return w.encodeInto(n,e)}:function(n,e){const t=w.encode(n);return e.set(t),{read:n.length,written:t.length}};function h(n,e,t){if(void 0===t){const t=w.encode(n),_=e(t.length,1)>>>0;return d().subarray(_,_+t.length).set(t),s=t.length,_}let _=n.length,r=e(_,1)>>>0;const c=d();let o=0;for(;o<_;o++){const e=n.charCodeAt(o);if(e>127)break;c[r+o]=e}if(o!==_){0!==o&&(n=n.slice(o)),r=t(r,_,_=o+3*n.length,1)>>>0;const e=d().subarray(r+o,r+_);o+=y(n,e).written,r=t(r,_,o,1)>>>0}return s=o,r}let p=null;function k(){return(null===p||!0===p.buffer.detached||void 0===p.buffer.detached&&p.buffer!==_.memory.buffer)&&(p=new DataView(_.memory.buffer)),p}const m="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((n=>{_.__wbindgen_export_2.get(n.dtor)(n.a,n.b)}));function C(n,e,t){_._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hfe4b83faab95dce7(n,e,l(t))}function x(n,e,t){try{const o=_.__wbindgen_add_to_stack_pointer(-16);_.render(o,l(n),e,t);var r=k().getInt32(o+0,!0),c=k().getInt32(o+4,!0);if(k().getInt32(o+8,!0))throw a(c);return 0!==r}finally{_.__wbindgen_add_to_stack_pointer(16)}}function v(n,e,t){try{const c=_.__wbindgen_add_to_stack_pointer(-16);_.render_editor(c,l(n),e,t);var r=k().getInt32(c+0,!0);if(k().getInt32(c+4,!0))throw a(r)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function I(n){try{const t=_.__wbindgen_add_to_stack_pointer(-16);_.load_handlers(t,l(n));var e=k().getInt32(t+0,!0);if(k().getInt32(t+4,!0))throw a(e)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function R(){_.main_js()}function j(n,e){try{return n.apply(this,e)}catch(n){_.__wbindgen_exn_store(l(n))}}function T(n){a(n)}function E(n,e){return l(new Error(u(n,e)))}function O(n,e){return l(u(n,e))}function S(n){const e=a(n).original;return 1==e.cnt--&&(e.a=0,!0)}function $(){return j((function(n,e,t,_){o(n).addEventListener(u(e,t),o(_))}),arguments)}function F(n){return o(n).width}function G(n){return o(n).height}function L(){return j((function(n,e,t){const _=o(n).getContext(u(e,t));return null==_?0:l(_)}),arguments)}function U(n){console.log(o(n))}function X(n){let e;try{e=o(n)instanceof CanvasRenderingContext2D}catch(n){e=!1}return e}function A(n,e){o(n).fillStyle=o(e)}function D(n,e,t){o(n).font=u(e,t)}function Q(n,e,t,_,r){o(n).clearRect(e,t,_,r)}function B(n,e,t,_,r){o(n).fillRect(e,t,_,r)}function J(){return j((function(n,e,t,_,r,c){o(n).fillText(u(e,t),_,r,c)}),arguments)}function H(n){let e;try{e=o(n)instanceof KeyboardEvent}catch(n){e=!1}return e}function K(n,e){const t=h(o(e).code,_.__wbindgen_malloc,_.__wbindgen_realloc),r=s;k().setInt32(n+4,r,!0),k().setInt32(n+0,t,!0)}function P(){return Date.now()}function V(n,e){const t=h(g(o(e)),_.__wbindgen_malloc,_.__wbindgen_realloc),r=s;k().setInt32(n+4,r,!0),k().setInt32(n+0,t,!0)}function q(n,e){throw new Error(u(n,e))}function z(n,e,t){const r=function(n,e,t,r){const c={a:n,b:e,cnt:1,dtor:13},o=(...n)=>{c.cnt++;try{return r(c.a,c.b,...n)}finally{0==--c.cnt&&(_.__wbindgen_export_2.get(c.dtor)(c.a,c.b),c.a=0,m.unregister(c))}};return o.original=c,m.register(o,c,c),o}(n,e,0,C);return l(r)}},650:(n,e,t)=>{var _=t(903);n.exports=t.v(e,n.id,"b6c26b8476073cab4ce0",{"./index_bg.js":{__wbindgen_object_drop_ref:_.bk,__wbindgen_error_new:_.Rj,__wbindgen_string_new:_.yc,__wbindgen_cb_drop:_.LC,__wbg_addEventListener_e167f012cbedfa4e:_.O,__wbg_width_53a5bd0268e99485:_.F2,__wbg_height_6fb32e51e54037ae:_.aJ,__wbg_getContext_69ec873410cbba3c:_.Ck,__wbg_log_b103404cc5920657:_.T6,__wbg_instanceof_CanvasRenderingContext2d_a0c4f0da6392b8ca:_.GP,__wbg_setfillStyle_98060f7b257936ba:_.Qg,__wbg_setfont_931e1f36bec6a342:_.CI,__wbg_clearRect_384c24b287b30369:_.k,__wbg_fillRect_a5a5da573f0412b5:_.Sw,__wbg_fillText_77c07f67110c7270:_.bR,__wbg_instanceof_KeyboardEvent_cb701ff8e9ff53cb:_.HG,__wbg_code_01dc6af887ca9ecb:_.pU,__wbg_now_b7a162010a9e75b4:_.cB,__wbindgen_debug_string:_.rl,__wbindgen_throw:_.Qn,__wbindgen_closure_wrapper33:_.XU}})}}]);