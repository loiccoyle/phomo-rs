(async()=>{var F="/assets/phomo_wasm_bg-Bpz6lO0D.wasm",R=async(e={},t)=>{let n;if(t.startsWith("data:")){const _=t.replace(/^data:.*?base64,/,"");let o;if(typeof Buffer=="function"&&typeof Buffer.from=="function")o=Buffer.from(_,"base64");else if(typeof atob=="function"){const c=atob(_);o=new Uint8Array(c.length);for(let a=0;a<c.length;a++)o[a]=c.charCodeAt(a)}else throw new Error("Cannot decode base64-encoded data URL");n=await WebAssembly.instantiate(o,e)}else{const _=await fetch(t),o=_.headers.get("Content-Type")||"";if("instantiateStreaming"in WebAssembly&&o.startsWith("application/wasm"))n=await WebAssembly.instantiateStreaming(_,e);else{const c=await _.arrayBuffer();n=await WebAssembly.instantiate(c,e)}}return n.instance.exports};let r;function $(e){r=e}const v=new Array(128).fill(void 0);v.push(void 0,null,!0,!1);function i(e){return v[e]}let k=v.length;function l(e){k===v.length&&v.push(v.length+1);const t=k;return k=v[t],v[t]=e,t}function O(e,t){try{return e.apply(this,t)}catch(n){r.__wbindgen_export_0(l(n))}}const V=typeof TextDecoder>"u"?(0,module.require)("util").TextDecoder:TextDecoder;let W=new V("utf-8",{ignoreBOM:!0,fatal:!0});W.decode();let M=null;function B(){return(M===null||M.byteLength===0)&&(M=new Uint8Array(r.memory.buffer)),M}function x(e,t){return e=e>>>0,W.decode(B().subarray(e,e+t))}function C(e){e<132||(v[e]=k,k=e)}function w(e){const t=i(e);return C(e),t}let m=0;const H=typeof TextEncoder>"u"?(0,module.require)("util").TextEncoder:TextEncoder;let U=new H("utf-8");const J=typeof U.encodeInto=="function"?function(e,t){return U.encodeInto(e,t)}:function(e,t){const n=U.encode(e);return t.set(n),{read:e.length,written:n.length}};function I(e,t,n){if(n===void 0){const u=U.encode(e),b=t(u.length,1)>>>0;return B().subarray(b,b+u.length).set(u),m=u.length,b}let _=e.length,o=t(_,1)>>>0;const c=B();let a=0;for(;a<_;a++){const u=e.charCodeAt(a);if(u>127)break;c[o+a]=u}if(a!==_){a!==0&&(e=e.slice(a)),o=n(o,_,_=a+e.length*3,1)>>>0;const u=B().subarray(o+a,o+_),b=J(e,u);a+=b.written,o=n(o,_,a,1)>>>0}return m=a,o}let A=null;function s(){return(A===null||A.buffer.detached===!0||A.buffer.detached===void 0&&A.buffer!==r.memory.buffer)&&(A=new DataView(r.memory.buffer)),A}function T(e){return e==null}function q(e){const t=typeof e;if(t=="number"||t=="boolean"||e==null)return`${e}`;if(t=="string")return`"${e}"`;if(t=="symbol"){const o=e.description;return o==null?"Symbol":`Symbol(${o})`}if(t=="function"){const o=e.name;return typeof o=="string"&&o.length>0?`Function(${o})`:"Function"}if(Array.isArray(e)){const o=e.length;let c="[";o>0&&(c+=q(e[0]));for(let a=1;a<o;a++)c+=", "+q(e[a]);return c+="]",c}const n=/\[object ([^\]]+)\]/.exec(toString.call(e));let _;if(n&&n.length>1)_=n[1];else return toString.call(e);if(_=="Object")try{return"Object("+JSON.stringify(e)+")"}catch{return"Object"}return e instanceof Error?`${e.name}: ${e.message}
${e.stack}`:_}function P(e,t){const n=t(e.length*1,1)>>>0;return B().set(e,n/1),m=e.length,n}let j=null;function K(){return(j===null||j.byteLength===0)&&(j=new Uint32Array(r.memory.buffer)),j}function Q(e,t){const n=t(e.length*4,4)>>>0;return K().set(e,n/4),m=e.length,n}function X(e,t){e=e>>>0;const n=s(),_=[];for(let o=e;o<e+4*t;o+=4)_.push(w(n.getUint32(o,!0)));return _}const z=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(e=>r.__wbg_mosaic_free(e>>>0,1));class Y{__destroy_into_raw(){const t=this.__wbg_ptr;return this.__wbg_ptr=0,z.unregister(this),t}free(){const t=this.__destroy_into_raw();r.__wbg_mosaic_free(t,0)}constructor(t,n,_,o,c,a,u){try{const h=r.__wbindgen_add_to_stack_pointer(-16),G=P(t,r.__wbindgen_export_2),D=m;var b=T(u)?0:Q(u,r.__wbindgen_export_2),g=m;r.mosaic_new(h,G,D,l(n),_,o,c,T(a)?2:a,b,g);var d=s().getInt32(h+4*0,!0),y=s().getInt32(h+4*1,!0),p=s().getInt32(h+4*2,!0);if(p)throw w(y);return this.__wbg_ptr=d>>>0,z.register(this,this.__wbg_ptr,this),this}finally{r.__wbindgen_add_to_stack_pointer(16)}}equalize(){r.mosaic_equalize(this.__wbg_ptr)}transferMasterToTiles(){r.mosaic_transferMasterToTiles(this.__wbg_ptr)}transferTilesToMaster(){try{const _=r.__wbindgen_add_to_stack_pointer(-16);r.mosaic_transferTilesToMaster(_,this.__wbg_ptr);var t=s().getInt32(_+4*0,!0),n=s().getInt32(_+4*1,!0);if(n)throw w(t)}finally{r.__wbindgen_add_to_stack_pointer(16)}}build(t){let n,_;try{const d=r.__wbindgen_add_to_stack_pointer(-16),y=I(t,r.__wbindgen_export_2,r.__wbindgen_export_3),p=m;r.mosaic_build(d,this.__wbg_ptr,y,p);var o=s().getInt32(d+4*0,!0),c=s().getInt32(d+4*1,!0),a=s().getInt32(d+4*2,!0),u=s().getInt32(d+4*3,!0),b=o,g=c;if(u)throw b=0,g=0,w(a);return n=b,_=g,x(b,g)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_export_1(n,_,1)}}buildGreedy(t){let n,_;try{const d=r.__wbindgen_add_to_stack_pointer(-16),y=I(t,r.__wbindgen_export_2,r.__wbindgen_export_3),p=m;r.mosaic_buildGreedy(d,this.__wbg_ptr,y,p);var o=s().getInt32(d+4*0,!0),c=s().getInt32(d+4*1,!0),a=s().getInt32(d+4*2,!0),u=s().getInt32(d+4*3,!0),b=o,g=c;if(u)throw b=0,g=0,w(a);return n=b,_=g,x(b,g)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_export_1(n,_,1)}}buildAuction(t){let n,_;try{const d=r.__wbindgen_add_to_stack_pointer(-16),y=I(t,r.__wbindgen_export_2,r.__wbindgen_export_3),p=m;r.mosaic_buildAuction(d,this.__wbg_ptr,y,p);var o=s().getInt32(d+4*0,!0),c=s().getInt32(d+4*1,!0),a=s().getInt32(d+4*2,!0),u=s().getInt32(d+4*3,!0),b=o,g=c;if(u)throw b=0,g=0,w(a);return n=b,_=g,x(b,g)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_export_1(n,_,1)}}getTiles(){try{const a=r.__wbindgen_add_to_stack_pointer(-16);r.mosaic_getTiles(a,this.__wbg_ptr);var t=s().getInt32(a+4*0,!0),n=s().getInt32(a+4*1,!0),_=s().getInt32(a+4*2,!0),o=s().getInt32(a+4*3,!0);if(o)throw w(_);var c=X(t,n).slice();return r.__wbindgen_export_1(t,n*4,4),c}finally{r.__wbindgen_add_to_stack_pointer(16)}}getMaster(){let t,n;try{const g=r.__wbindgen_add_to_stack_pointer(-16);r.mosaic_getMaster(g,this.__wbg_ptr);var _=s().getInt32(g+4*0,!0),o=s().getInt32(g+4*1,!0),c=s().getInt32(g+4*2,!0),a=s().getInt32(g+4*3,!0),u=_,b=o;if(a)throw u=0,b=0,w(c);return t=u,n=b,x(u,b)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_export_1(t,n,1)}}buildBlueprint(t){try{const c=r.__wbindgen_add_to_stack_pointer(-16),a=I(t,r.__wbindgen_export_2,r.__wbindgen_export_3),u=m;r.mosaic_buildBlueprint(c,this.__wbg_ptr,a,u);var n=s().getInt32(c+4*0,!0),_=s().getInt32(c+4*1,!0),o=s().getInt32(c+4*2,!0);if(o)throw w(_);return w(n)}finally{r.__wbindgen_add_to_stack_pointer(16)}}buildBlueprintGreedy(t){try{const c=r.__wbindgen_add_to_stack_pointer(-16),a=I(t,r.__wbindgen_export_2,r.__wbindgen_export_3),u=m;r.mosaic_buildBlueprintGreedy(c,this.__wbg_ptr,a,u);var n=s().getInt32(c+4*0,!0),_=s().getInt32(c+4*1,!0),o=s().getInt32(c+4*2,!0);if(o)throw w(_);return w(n)}finally{r.__wbindgen_add_to_stack_pointer(16)}}buildBlueprintAuction(t){try{const c=r.__wbindgen_add_to_stack_pointer(-16),a=I(t,r.__wbindgen_export_2,r.__wbindgen_export_3),u=m;r.mosaic_buildBlueprintAuction(c,this.__wbg_ptr,a,u);var n=s().getInt32(c+4*0,!0),_=s().getInt32(c+4*1,!0),o=s().getInt32(c+4*2,!0);if(o)throw w(_);return w(n)}finally{r.__wbindgen_add_to_stack_pointer(16)}}renderBlueprint(t){let n,_;try{const d=r.__wbindgen_add_to_stack_pointer(-16);r.mosaic_renderBlueprint(d,this.__wbg_ptr,l(t));var o=s().getInt32(d+4*0,!0),c=s().getInt32(d+4*1,!0),a=s().getInt32(d+4*2,!0),u=s().getInt32(d+4*3,!0),b=o,g=c;if(u)throw b=0,g=0,w(a);return n=b,_=g,x(b,g)}finally{r.__wbindgen_add_to_stack_pointer(16),r.__wbindgen_export_1(n,_,1)}}}function Z(e){const t=i(e).buffer;return l(t)}function ee(){return O(function(e,t){const n=i(e).call(i(t));return l(n)},arguments)}function te(e,t,n,_){console.debug(i(e),i(t),i(n),i(_))}function ne(e){return i(e).done}function _e(e){console.error(i(e))}function re(e,t){let n,_;try{n=e,_=t,console.error(x(e,t))}finally{r.__wbindgen_export_1(n,_,1)}}function ie(e,t,n,_){console.error(i(e),i(t),i(n),i(_))}function oe(){return O(function(e,t){const n=Reflect.get(i(e),i(t));return l(n)},arguments)}function se(e,t){const n=i(e)[t>>>0];return l(n)}function ae(e,t){const n=i(e)[i(t)];return l(n)}function ce(e,t,n,_){console.info(i(e),i(t),i(n),i(_))}function ue(e){let t;try{t=i(e) instanceof ArrayBuffer}catch{t=!1}return t}function de(e){let t;try{t=i(e) instanceof Uint8Array}catch{t=!1}return t}function be(e){return Array.isArray(i(e))}function ge(e){return Number.isSafeInteger(i(e))}function fe(){return l(Symbol.iterator)}function le(e){return i(e).length}function we(e){return i(e).length}function pe(e,t,n,_){console.log(i(e),i(t),i(n),i(_))}function me(){const e=new Object;return l(e)}function ye(){const e=new Array;return l(e)}function he(){const e=new Error;return l(e)}function Ie(e){const t=new Uint8Array(i(e));return l(t)}function ve(e){const t=i(e).next;return l(t)}function xe(){return O(function(e){const t=i(e).next();return l(t)},arguments)}function Te(e,t,n){i(e)[t>>>0]=w(n)}function Ae(e,t,n){i(e)[w(t)]=w(n)}function ke(e,t,n){i(e).set(i(t),n>>>0)}function Be(e,t){const n=i(t).stack,_=I(n,r.__wbindgen_export_2,r.__wbindgen_export_3),o=m;s().setInt32(e+4*1,o,!0),s().setInt32(e+4*0,_,!0)}function Me(e){const t=i(e).value;return l(t)}function je(e,t,n,_){console.warn(i(e),i(t),i(n),i(_))}function Ue(e){return+i(e)}function Se(e){const t=BigInt.asUintN(64,e);return l(t)}function Ee(e,t){const n=i(t),_=typeof n=="bigint"?n:void 0;s().setBigInt64(e+8*1,T(_)?BigInt(0):_,!0),s().setInt32(e+4*0,!T(_),!0)}function Ge(e){const t=i(e);return typeof t=="boolean"?t?1:0:2}function qe(e,t){const n=q(i(t)),_=I(n,r.__wbindgen_export_2,r.__wbindgen_export_3),o=m;s().setInt32(e+4*1,o,!0),s().setInt32(e+4*0,_,!0)}function Oe(e,t){const n=new Error(x(e,t));return l(n)}function ze(e,t){return i(e) in i(t)}function We(e){return typeof i(e)=="bigint"}function Le(e){return typeof i(e)=="function"}function Ne(e){const t=i(e);return typeof t=="object"&&t!==null}function De(e){return i(e)===void 0}function Fe(e,t){return i(e)===i(t)}function Re(e,t){return i(e)==i(t)}function $e(){const e=r.memory;return l(e)}function Ve(e,t){const n=i(t),_=typeof n=="number"?n:void 0;s().setFloat64(e+8*1,T(_)?0:_,!0),s().setInt32(e+4*0,!T(_),!0)}function Ce(e){return l(e)}function He(e){const t=i(e);return l(t)}function Je(e){w(e)}function Pe(e,t){const n=i(t),_=typeof n=="string"?n:void 0;var o=T(_)?0:I(_,r.__wbindgen_export_2,r.__wbindgen_export_3),c=m;s().setInt32(e+4*1,c,!0),s().setInt32(e+4*0,o,!0)}function Ke(e,t){const n=x(e,t);return l(n)}function Qe(e,t){throw new Error(x(e,t))}URL=globalThis.URL;const f=await R({"./phomo_wasm_bg.js":{__wbindgen_object_drop_ref:Je,__wbg_new_405e22f390576ce2:me,__wbg_new_78feb108b6472713:ye,__wbindgen_number_new:Ce,__wbindgen_bigint_from_u64:Se,__wbg_set_3f1d0b984ed272ed:Ae,__wbg_set_37837023f3d740e8:Te,__wbindgen_error_new:Oe,__wbindgen_is_object:Ne,__wbg_getwithrefkey_1dc361bd10053bfe:ae,__wbindgen_is_undefined:De,__wbindgen_in:ze,__wbindgen_is_bigint:We,__wbindgen_bigint_get_as_i64:Ee,__wbindgen_jsval_eq:Fe,__wbg_isSafeInteger_343e2beeeece1bb0:ge,__wbindgen_as_number:Ue,__wbindgen_string_new:Ke,__wbg_error_524f506f44df1645:_e,__wbg_length_e2d2a49132c1b256:we,__wbg_get_b9b93047fe3cf45b:se,__wbg_new_a12002a7f91c75be:Ie,__wbg_isArray_a1eab7e0d067391b:be,__wbg_iterator_9a24c88df860dc65:fe,__wbg_get_67b2ba62fc30de12:oe,__wbindgen_is_function:Le,__wbg_call_672a4d21634d4a24:ee,__wbg_next_25feadfc0913fea9:ve,__wbg_next_6574e1a8a62d1055:xe,__wbg_done_769e5ede4b31c67b:ne,__wbg_value_cd1ffa7b1ab794f1:Me,__wbg_new_8a6f238a6ece86ea:he,__wbg_stack_0ed75d68575b0f3c:Be,__wbg_error_7534b8e9a36f1ab4:re,__wbg_length_a446193dc22c12f8:le,__wbindgen_memory:$e,__wbg_buffer_609cc3eee51ed158:Z,__wbg_set_65595bdd868b3009:ke,__wbindgen_jsval_loose_eq:Re,__wbindgen_boolean_get:Ge,__wbindgen_number_get:Ve,__wbindgen_string_get:Pe,__wbg_instanceof_Uint8Array_17156bcf118086a9:de,__wbg_instanceof_ArrayBuffer_e14585432e3737fc:ue,__wbindgen_object_clone_ref:He,__wbindgen_throw:Qe,__wbindgen_debug_string:qe,__wbg_error_80de38b3f7cc3c3c:ie,__wbg_warn_aaf1f4664a035bd6:je,__wbg_info_033d8b8a0838f1d3:ce,__wbg_log_cad59bb680daec67:pe,__wbg_debug_e17b51583ca6a632:te}},F),Xe=f.memory,Ye=f.init_panic_hook,Ze=f.overlayGrid,et=f.__wbg_mosaic_free,tt=f.mosaic_new,nt=f.mosaic_equalize,_t=f.mosaic_transferMasterToTiles,rt=f.mosaic_transferTilesToMaster,it=f.mosaic_build,ot=f.mosaic_buildGreedy,st=f.mosaic_buildAuction,at=f.mosaic_getTiles,ct=f.mosaic_getMaster,ut=f.mosaic_buildBlueprint,dt=f.mosaic_buildBlueprintGreedy,bt=f.mosaic_buildBlueprintAuction,gt=f.mosaic_renderBlueprint,ft=f.__wbindgen_export_0,lt=f.__wbindgen_export_1,wt=f.__wbindgen_export_2,pt=f.__wbindgen_export_3,mt=f.__wbindgen_add_to_stack_pointer,L=f.__wbindgen_start;var yt=Object.freeze({__proto__:null,__wbg_mosaic_free:et,__wbindgen_add_to_stack_pointer:mt,__wbindgen_export_0:ft,__wbindgen_export_1:lt,__wbindgen_export_2:wt,__wbindgen_export_3:pt,__wbindgen_start:L,init_panic_hook:Ye,memory:Xe,mosaic_build:it,mosaic_buildAuction:st,mosaic_buildBlueprint:ut,mosaic_buildBlueprintAuction:bt,mosaic_buildBlueprintGreedy:dt,mosaic_buildGreedy:ot,mosaic_equalize:nt,mosaic_getMaster:ct,mosaic_getTiles:at,mosaic_new:tt,mosaic_renderBlueprint:gt,mosaic_transferMasterToTiles:_t,mosaic_transferTilesToMaster:rt,overlayGrid:Ze});$(yt);L();const N=async e=>{const _=await (await (await fetch(e)).blob()).arrayBuffer();return new Uint8Array(_)};var S=(e=>(e.None="none",e.MasterToTile="masterToTile",e.TileToMaster="tileToMaster",e.Equalize="equalize",e))(S||{}),E=(e=>(e.Optimal="optimal",e.Greedy="greedy",e.Auction="auction",e))(E||{});const ht=async e=>Promise.all(e.map(t=>N(t)));self.onmessage=async e=>{console.log(e);const{masterImageUrl:t,tileImagesUrls:n,gridWidth:_,gridHeight:o,tileSizingMethod:c,tileRepeats:a,tileAssignmentMethod:u,colorMatchingMethod:b,mosaicImageSize:g}=e.data;console.log(e.data);try{const d=await N(t),y=await ht(n),p=new Y(d,y,_,o,a,c,g?Uint32Array.from(g):void 0);switch(b){case S.MasterToTile:p.transferMasterToTiles();break;case S.TileToMaster:p.transferTilesToMaster();break;case S.Equalize:p.equalize();break}let h;switch(u){case E.Optimal:h=p.buildBlueprint("NormL1");break;case E.Greedy:h=p.buildBlueprintGreedy("NormL1");break;case E.Auction:h=p.buildBlueprintAuction("NormL1");break}const G=p.renderBlueprint(h);self.postMessage({mosaicTiles:p.getTiles(),mosaicBlueprint:h,mosaicImage:G})}catch(d){let y="An unknown error occurred.";d instanceof Error?y=d.message:typeof d=="string"&&(y=d),self.postMessage({error:y})}}})();