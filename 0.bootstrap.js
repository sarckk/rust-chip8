(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/librchip.js":
/*!**************************!*\
  !*** ../pkg/librchip.js ***!
  \**************************/
/*! exports provided: WasmVM, __wbindgen_object_drop_ref, __wbg_randomFillSync_378e02b85af41ab6, __wbg_getRandomValues_99bbe8a65f4aef87, __wbg_process_5729605ce9d34ea8, __wbindgen_is_object, __wbg_versions_531e16e1a776ee97, __wbg_node_18b58a160b60d170, __wbindgen_is_string, __wbg_static_accessor_NODE_MODULE_bdc5ca9096c68aeb, __wbg_require_edfaedd93e302925, __wbg_crypto_2bc4d5b05161de5b, __wbg_msCrypto_d003eebe62c636a9, __wbg_newnoargs_f579424187aa1717, __wbg_call_89558c3e96703ca1, __wbindgen_object_clone_ref, __wbg_self_e23d74ae45fb17d1, __wbg_window_b4be7f48b24ac56e, __wbg_globalThis_d61b1f48a57191ae, __wbg_global_e7669da72fd7f239, __wbindgen_is_undefined, __wbg_buffer_5e74a88a1424a2e0, __wbg_newwithbyteoffsetandlength_278ec7532799393a, __wbg_new_e3b800e570795b3c, __wbg_set_5b8081e9d002f0df, __wbg_length_30803400a8f15c59, __wbg_newwithlength_5f4ce114a24dfe1e, __wbg_subarray_a68f835ca2af506f, __wbg_new_693216e109162396, __wbg_stack_0ddaca5d1abfb52f, __wbg_error_09919627ac0992f5, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./librchip_bg.wasm */ \"../pkg/librchip_bg.wasm\");\n/* harmony import */ var _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./librchip_bg.js */ \"../pkg/librchip_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"WasmVM\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"WasmVM\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_378e02b85af41ab6\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_randomFillSync_378e02b85af41ab6\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_99bbe8a65f4aef87\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_99bbe8a65f4aef87\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_process_5729605ce9d34ea8\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_process_5729605ce9d34ea8\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_object\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_object\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_versions_531e16e1a776ee97\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_versions_531e16e1a776ee97\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_node_18b58a160b60d170\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_node_18b58a160b60d170\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_string\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_string\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_NODE_MODULE_bdc5ca9096c68aeb\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_static_accessor_NODE_MODULE_bdc5ca9096c68aeb\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_edfaedd93e302925\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_require_edfaedd93e302925\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_2bc4d5b05161de5b\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_crypto_2bc4d5b05161de5b\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_d003eebe62c636a9\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_msCrypto_d003eebe62c636a9\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_f579424187aa1717\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newnoargs_f579424187aa1717\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_89558c3e96703ca1\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_call_89558c3e96703ca1\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_clone_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_e23d74ae45fb17d1\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_e23d74ae45fb17d1\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_b4be7f48b24ac56e\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_window_b4be7f48b24ac56e\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_d61b1f48a57191ae\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_globalThis_d61b1f48a57191ae\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_e7669da72fd7f239\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_global_e7669da72fd7f239\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_5e74a88a1424a2e0\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_buffer_5e74a88a1424a2e0\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithbyteoffsetandlength_278ec7532799393a\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newwithbyteoffsetandlength_278ec7532799393a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_e3b800e570795b3c\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_e3b800e570795b3c\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_5b8081e9d002f0df\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_set_5b8081e9d002f0df\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_30803400a8f15c59\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_length_30803400a8f15c59\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithlength_5f4ce114a24dfe1e\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newwithlength_5f4ce114a24dfe1e\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_subarray_a68f835ca2af506f\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_subarray_a68f835ca2af506f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_693216e109162396\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_693216e109162396\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_0ddaca5d1abfb52f\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_0ddaca5d1abfb52f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_09919627ac0992f5\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_09919627ac0992f5\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return _librchip_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_memory\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/librchip.js?");

/***/ }),

/***/ "../pkg/librchip_bg.js":
/*!*****************************!*\
  !*** ../pkg/librchip_bg.js ***!
  \*****************************/
/*! exports provided: WasmVM, __wbindgen_object_drop_ref, __wbg_randomFillSync_378e02b85af41ab6, __wbg_getRandomValues_99bbe8a65f4aef87, __wbg_process_5729605ce9d34ea8, __wbindgen_is_object, __wbg_versions_531e16e1a776ee97, __wbg_node_18b58a160b60d170, __wbindgen_is_string, __wbg_static_accessor_NODE_MODULE_bdc5ca9096c68aeb, __wbg_require_edfaedd93e302925, __wbg_crypto_2bc4d5b05161de5b, __wbg_msCrypto_d003eebe62c636a9, __wbg_newnoargs_f579424187aa1717, __wbg_call_89558c3e96703ca1, __wbindgen_object_clone_ref, __wbg_self_e23d74ae45fb17d1, __wbg_window_b4be7f48b24ac56e, __wbg_globalThis_d61b1f48a57191ae, __wbg_global_e7669da72fd7f239, __wbindgen_is_undefined, __wbg_buffer_5e74a88a1424a2e0, __wbg_newwithbyteoffsetandlength_278ec7532799393a, __wbg_new_e3b800e570795b3c, __wbg_set_5b8081e9d002f0df, __wbg_length_30803400a8f15c59, __wbg_newwithlength_5f4ce114a24dfe1e, __wbg_subarray_a68f835ca2af506f, __wbg_new_693216e109162396, __wbg_stack_0ddaca5d1abfb52f, __wbg_error_09919627ac0992f5, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module, global) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"WasmVM\", function() { return WasmVM; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_378e02b85af41ab6\", function() { return __wbg_randomFillSync_378e02b85af41ab6; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_99bbe8a65f4aef87\", function() { return __wbg_getRandomValues_99bbe8a65f4aef87; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_process_5729605ce9d34ea8\", function() { return __wbg_process_5729605ce9d34ea8; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_object\", function() { return __wbindgen_is_object; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_versions_531e16e1a776ee97\", function() { return __wbg_versions_531e16e1a776ee97; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_node_18b58a160b60d170\", function() { return __wbg_node_18b58a160b60d170; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_string\", function() { return __wbindgen_is_string; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_NODE_MODULE_bdc5ca9096c68aeb\", function() { return __wbg_static_accessor_NODE_MODULE_bdc5ca9096c68aeb; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_edfaedd93e302925\", function() { return __wbg_require_edfaedd93e302925; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_2bc4d5b05161de5b\", function() { return __wbg_crypto_2bc4d5b05161de5b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_d003eebe62c636a9\", function() { return __wbg_msCrypto_d003eebe62c636a9; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_f579424187aa1717\", function() { return __wbg_newnoargs_f579424187aa1717; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_89558c3e96703ca1\", function() { return __wbg_call_89558c3e96703ca1; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return __wbindgen_object_clone_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_e23d74ae45fb17d1\", function() { return __wbg_self_e23d74ae45fb17d1; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_b4be7f48b24ac56e\", function() { return __wbg_window_b4be7f48b24ac56e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_d61b1f48a57191ae\", function() { return __wbg_globalThis_d61b1f48a57191ae; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_e7669da72fd7f239\", function() { return __wbg_global_e7669da72fd7f239; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_5e74a88a1424a2e0\", function() { return __wbg_buffer_5e74a88a1424a2e0; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithbyteoffsetandlength_278ec7532799393a\", function() { return __wbg_newwithbyteoffsetandlength_278ec7532799393a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_e3b800e570795b3c\", function() { return __wbg_new_e3b800e570795b3c; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_5b8081e9d002f0df\", function() { return __wbg_set_5b8081e9d002f0df; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_30803400a8f15c59\", function() { return __wbg_length_30803400a8f15c59; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithlength_5f4ce114a24dfe1e\", function() { return __wbg_newwithlength_5f4ce114a24dfe1e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_subarray_a68f835ca2af506f\", function() { return __wbg_subarray_a68f835ca2af506f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_693216e109162396\", function() { return __wbg_new_693216e109162396; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_0ddaca5d1abfb52f\", function() { return __wbg_stack_0ddaca5d1abfb52f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_09919627ac0992f5\", function() { return __wbg_error_09919627ac0992f5; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return __wbindgen_memory; });\n/* harmony import */ var _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./librchip_bg.wasm */ \"../pkg/librchip_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nfunction passArray8ToWasm0(arg, malloc) {\n    const ptr = malloc(arg.length * 1);\n    getUint8Memory0().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nfunction handleError(f, args) {\n    try {\n        return f.apply(this, args);\n    } catch (e) {\n        _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n    }\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n/**\n*/\nclass WasmVM {\n\n    static __wrap(ptr) {\n        const obj = Object.create(WasmVM.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_wasmvm_free\"](ptr);\n    }\n    /**\n    */\n    constructor() {\n        var ret = _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"wasmvm_new\"]();\n        return WasmVM.__wrap(ret);\n    }\n    /**\n    * @returns {boolean}\n    */\n    should_redraw() {\n        var ret = _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"wasmvm_should_redraw\"](this.ptr);\n        return ret !== 0;\n    }\n    /**\n    * @param {string} key\n    * @param {boolean} pressed\n    */\n    set_key(key, pressed) {\n        var ptr0 = passStringToWasm0(key, _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"wasmvm_set_key\"](this.ptr, ptr0, len0, pressed);\n    }\n    /**\n    * @param {Uint8Array} buf\n    */\n    load_program(buf) {\n        var ptr0 = passArray8ToWasm0(buf, _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"wasmvm_load_program\"](this.ptr, ptr0, len0);\n    }\n    /**\n    */\n    emulate_cycle() {\n        _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"wasmvm_emulate_cycle\"](this.ptr);\n    }\n    /**\n    * @returns {boolean}\n    */\n    decrement_timers() {\n        var ret = _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"wasmvm_decrement_timers\"](this.ptr);\n        return ret !== 0;\n    }\n    /**\n    * @returns {Uint8Array}\n    */\n    get_display() {\n        var ret = _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"wasmvm_get_display\"](this.ptr);\n        return takeObject(ret);\n    }\n}\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbg_randomFillSync_378e02b85af41ab6() { return handleError(function (arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));\n}, arguments) };\n\nfunction __wbg_getRandomValues_99bbe8a65f4aef87() { return handleError(function (arg0, arg1) {\n    getObject(arg0).getRandomValues(getObject(arg1));\n}, arguments) };\n\nfunction __wbg_process_5729605ce9d34ea8(arg0) {\n    var ret = getObject(arg0).process;\n    return addHeapObject(ret);\n};\n\nfunction __wbindgen_is_object(arg0) {\n    const val = getObject(arg0);\n    var ret = typeof(val) === 'object' && val !== null;\n    return ret;\n};\n\nfunction __wbg_versions_531e16e1a776ee97(arg0) {\n    var ret = getObject(arg0).versions;\n    return addHeapObject(ret);\n};\n\nfunction __wbg_node_18b58a160b60d170(arg0) {\n    var ret = getObject(arg0).node;\n    return addHeapObject(ret);\n};\n\nfunction __wbindgen_is_string(arg0) {\n    var ret = typeof(getObject(arg0)) === 'string';\n    return ret;\n};\n\nfunction __wbg_static_accessor_NODE_MODULE_bdc5ca9096c68aeb() {\n    var ret = module;\n    return addHeapObject(ret);\n};\n\nfunction __wbg_require_edfaedd93e302925() { return handleError(function (arg0, arg1, arg2) {\n    var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_crypto_2bc4d5b05161de5b(arg0) {\n    var ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nfunction __wbg_msCrypto_d003eebe62c636a9(arg0) {\n    var ret = getObject(arg0).msCrypto;\n    return addHeapObject(ret);\n};\n\nfunction __wbg_newnoargs_f579424187aa1717(arg0, arg1) {\n    var ret = new Function(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nfunction __wbg_call_89558c3e96703ca1() { return handleError(function (arg0, arg1) {\n    var ret = getObject(arg0).call(getObject(arg1));\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbindgen_object_clone_ref(arg0) {\n    var ret = getObject(arg0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_self_e23d74ae45fb17d1() { return handleError(function () {\n    var ret = self.self;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_window_b4be7f48b24ac56e() { return handleError(function () {\n    var ret = window.window;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_globalThis_d61b1f48a57191ae() { return handleError(function () {\n    var ret = globalThis.globalThis;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_global_e7669da72fd7f239() { return handleError(function () {\n    var ret = global.global;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbindgen_is_undefined(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nfunction __wbg_buffer_5e74a88a1424a2e0(arg0) {\n    var ret = getObject(arg0).buffer;\n    return addHeapObject(ret);\n};\n\nfunction __wbg_newwithbyteoffsetandlength_278ec7532799393a(arg0, arg1, arg2) {\n    var ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_new_e3b800e570795b3c(arg0) {\n    var ret = new Uint8Array(getObject(arg0));\n    return addHeapObject(ret);\n};\n\nfunction __wbg_set_5b8081e9d002f0df(arg0, arg1, arg2) {\n    getObject(arg0).set(getObject(arg1), arg2 >>> 0);\n};\n\nfunction __wbg_length_30803400a8f15c59(arg0) {\n    var ret = getObject(arg0).length;\n    return ret;\n};\n\nfunction __wbg_newwithlength_5f4ce114a24dfe1e(arg0) {\n    var ret = new Uint8Array(arg0 >>> 0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_subarray_a68f835ca2af506f(arg0, arg1, arg2) {\n    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_new_693216e109162396() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_error_09919627ac0992f5(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbindgen_memory() {\n    var ret = _librchip_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"];\n    return addHeapObject(ret);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../web/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module), __webpack_require__(/*! ./../web/node_modules/webpack/buildin/global.js */ \"./node_modules/webpack/buildin/global.js\")))\n\n//# sourceURL=webpack:///../pkg/librchip_bg.js?");

/***/ }),

/***/ "../pkg/librchip_bg.wasm":
/*!*******************************!*\
  !*** ../pkg/librchip_bg.wasm ***!
  \*******************************/
/*! exports provided: memory, __wbg_wasmvm_free, wasmvm_new, wasmvm_should_redraw, wasmvm_set_key, wasmvm_load_program, wasmvm_emulate_cycle, wasmvm_decrement_timers, wasmvm_get_display, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_exn_store, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./librchip_bg.js */ \"../pkg/librchip_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/librchip_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var chip8__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! chip8 */ \"../pkg/librchip.js\");\n/* harmony import */ var _roms_txt__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./roms.txt */ \"./roms.txt\");\n/* harmony import */ var alpinejs__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! alpinejs */ \"./node_modules/alpinejs/dist/module.esm.js\");\n\n\n\n\nconst roms = _roms_txt__WEBPACK_IMPORTED_MODULE_1__[\"default\"].split('\\n').filter(el => el);\nconst initRom = roms[0];\n\nconst audioCtx = new(window.AudioContext || window.webkitAudioContext)();\n\nlet currentAnimation = undefined;\n\nconst cycle_loop = (vm) => {\n    const canvas = document.getElementById('canvas');\n    const ctx = canvas.getContext('2d');\n    ctx.clearRect(0, 0, canvas.width, canvas.height);\n\n    // 600 fps\n    let redraw = false;\n\n    for(let i = 0; i < 10; i++) {\n        vm.emulate_cycle();\n        if(vm.should_redraw()) {\n            redraw = true;\n        }\n    }\n\n    // runs at 60fps\n    let beep = vm.decrement_timers();\n    if(beep) {\n        const oscillator = audioCtx.createOscillator();\n        oscillator.type = 'square'\n        oscillator.frequency.value = 356; \n        const volume = audioCtx.createGain();\n        volume.connect(audioCtx.destination);\n        oscillator.connect(volume);\n        volume.gain.value = 0.02;\n        oscillator.start();\n        oscillator.stop(audioCtx.currentTime + 0.10);\n    }\n\n    if(redraw) {\n        const display = vm.get_display();\n        display.forEach((px, i) => {\n            const row = Math.floor(i / 64);\n            const col = i % 64;\n            if(px == 1) {\n                ctx.fillRect(col*10, row*10, 10, 10);\n            } \n        });\n    }\n\n\n    currentAnimation = window.requestAnimationFrame(() => {\n        cycle_loop(vm);\n    });\n}\n\nconst load_rom = async (rom) => {\n    const res = await fetch(`./programs/${rom}.ch8`);\n    const res_instr = await fetch(`./programs/${rom}.txt`);\n    const instructions = await res_instr.text(); \n    const bytes = await res.arrayBuffer();\n    return [new Uint8Array(bytes, 0, bytes.byteLength), instructions];\n}\n\nconst run = async (rom) => {\n    if(currentAnimation) {\n        window.cancelAnimationFrame(currentAnimation);\n        currentAnimation = undefined;\n    }\n\n    const vm = new chip8__WEBPACK_IMPORTED_MODULE_0__[\"WasmVM\"]();\n    vm.load_program(rom);\n\n    document.addEventListener(\"keydown\", (e) => {\n        vm.set_key(e.code, true);\n    })\n\n    document.addEventListener(\"keyup\", (e) => {\n        vm.set_key(e.code, false);\n    })\n\n    cycle_loop(vm);\n}\n\ndocument.addEventListener(\"alpine:init\", async () => {\n    const canvas = document.getElementById('canvas');\n    const ctx = canvas.getContext('2d');\n    ctx.fillStyle = '#e0e0e0';\n\n    alpinejs__WEBPACK_IMPORTED_MODULE_2__[\"default\"].data(\"chip8\", () => ({\n        roms,\n        selectedRom: initRom,\n        instructions: \"\",\n\n        init() {\n            this.runRom(initRom);\n        },\n\n        async runRom(romName) {\n            const [rom, instructions] = await load_rom(romName); \n            this.instructions = instructions;\n            console.log(\"INSTRUCTIONS:\");\n            console.log(this.instructions);\n            run(rom);\n        },\n\n        uploadRom(event) {\n            if(!event.target.files.length) return;\n            this.instructions = \"\";\n\n            let file = event.target.files[0];\n            const reader = new FileReader();\n            reader.readAsArrayBuffer(file);\n            reader.onload = (e) => {\n                const rom = new Uint8Array(reader.result);\n                run(rom);\n            }\n        }\n    }))\n})\n\nalpinejs__WEBPACK_IMPORTED_MODULE_2__[\"default\"].start();\n\n\n\n\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./roms.txt":
/*!******************!*\
  !*** ./roms.txt ***!
  \******************/
/*! exports provided: default */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony default export */ __webpack_exports__[\"default\"] = (\"Space Invaders [David Winter]\\nTetris [Fran Dachille, 1991]\\nPong [Paul Vervalin, 1990]\\n15 Puzzle [Roger Ivie]\\nMost Dangerous Game [Peter Maruhnic]\\nRush Hour [Hap, 2006]\\nUFO [Lutz V, 1992]\\nSubmarine [Carmelo Cortez, 1978]\\nBreakout [Carmelo Cortez, 1979]\\nBiorhythm [Jef Winsor]\\nKaleidoscope [Joseph Weisbecker, 1978]\\nBrick (Brix hack, 1990)\\nHidden [David Winter, 1996]\\nLunar Lander [Udo Pernisz, 1979]\\nBlitz [David Winter]\\nDeflection [John Fort]\\nConnect 4 [David Winter]\\nAnimal Race [Brian Astle]\\nBowling [Gooitzen van der Wal]\\nCraps [Camerlo Cortez, 1978]\\n\");\n\n//# sourceURL=webpack:///./roms.txt?");

/***/ })

}]);