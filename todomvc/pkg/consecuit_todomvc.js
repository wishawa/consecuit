
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_16(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29aa12f8c78a1bc6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_19(arg0, arg1) {
    wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3a80db5b252c5980(arg0, arg1);
}

function __wbg_adapter_22(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29aa12f8c78a1bc6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_25(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29aa12f8c78a1bc6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_28(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h29aa12f8c78a1bc6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_31(arg0, arg1) {
    wasm.__wbindgen_export_5(arg0, arg1);
}

/**
*/
export function run() {
    wasm.run();
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export_7(addHeapObject(e));
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('consecuit_todomvc_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_export_6(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Window_c4b70662a0d2c5ec = function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_document_1c64944725c0d81d = function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_location_f98ad02632f88c43 = function(arg0) {
        var ret = getObject(arg0).location;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setonbeforeunload_c7e117f6dbbc0554 = function(arg0, arg1) {
        getObject(arg0).onbeforeunload = getObject(arg1);
    };
    imports.wbg.__wbg_setonhashchange_9059b2faa2000d1f = function(arg0, arg1) {
        getObject(arg0).onhashchange = getObject(arg1);
    };
    imports.wbg.__wbg_localStorage_6775414303ab5085 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).localStorage;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setTimeout_faf087ef7e084f8c = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).setTimeout(getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_body_78ae4fd43b446013 = function(arg0) {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createDocumentFragment_a0f37fe9b567c97f = function(arg0) {
        var ret = getObject(arg0).createDocumentFragment();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_86c152812a141a62 = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTextNode_365db3bc3d0523ab = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_HtmlHeadingElement_bb7449e35aeaa1fa = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLHeadingElement;
        return ret;
    };
    imports.wbg.__wbg_setalign_109a3544ab772990 = function(arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_instanceof_HtmlDivElement_abfbf03c1045fc4b = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLDivElement;
        return ret;
    };
    imports.wbg.__wbg_setalign_87fcf1cd2f43478a = function(arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_childNodes_6db3cd681ef9d224 = function(arg0) {
        var ret = getObject(arg0).childNodes;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setnodeValue_702374ad3d0ec3df = function(arg0, arg1, arg2) {
        getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settextContent_799ebbf96e16265d = function(arg0, arg1, arg2) {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_appendChild_d318db34c4559916 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_removeChild_d3ca7b53e537867e = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).removeChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setProperty_1460c660bc329763 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_hash_0fff5255cf3c317c = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg1).hash;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_getItem_77fb9d4666f3b93a = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg1).getItem(getStringFromWasm0(arg2, arg3));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_setItem_b0c4561489dffecd = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlButtonElement_54060a3d8d49c8a6 = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLButtonElement;
        return ret;
    };
    imports.wbg.__wbg_setautofocus_579a5b5f2224d0eb = function(arg0, arg1) {
        getObject(arg0).autofocus = arg1 !== 0;
    };
    imports.wbg.__wbg_setdisabled_d52451f415793557 = function(arg0, arg1) {
        getObject(arg0).disabled = arg1 !== 0;
    };
    imports.wbg.__wbg_setformAction_36e12794f6021e2b = function(arg0, arg1, arg2) {
        getObject(arg0).formAction = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformEnctype_ab7b88b7e4f0e268 = function(arg0, arg1, arg2) {
        getObject(arg0).formEnctype = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformMethod_b1c7edfd97b45913 = function(arg0, arg1, arg2) {
        getObject(arg0).formMethod = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformNoValidate_c9c234f15fe5e0b9 = function(arg0, arg1) {
        getObject(arg0).formNoValidate = arg1 !== 0;
    };
    imports.wbg.__wbg_setformTarget_f005d4b64b541183 = function(arg0, arg1, arg2) {
        getObject(arg0).formTarget = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setname_46541685faad9c65 = function(arg0, arg1, arg2) {
        getObject(arg0).name = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settype_bd9da7e07b7cb217 = function(arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setvalue_0baa255f4dbc02bd = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setid_681bb5a14c3d5850 = function(arg0, arg1, arg2) {
        getObject(arg0).id = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setclassName_7e8ab705edf23973 = function(arg0, arg1, arg2) {
        getObject(arg0).className = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setscrollLeft_20ed260b104ef478 = function(arg0, arg1) {
        getObject(arg0).scrollLeft = arg1;
    };
    imports.wbg.__wbg_setinnerHTML_e5b817d6227a431c = function(arg0, arg1, arg2) {
        getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setouterHTML_f2960a0d81fc138d = function(arg0, arg1, arg2) {
        getObject(arg0).outerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setslot_44e0027dd1e4c8bd = function(arg0, arg1, arg2) {
        getObject(arg0).slot = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_removeAttribute_eea03ed128669b8f = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlLabelElement_6555d0897d5aef8a = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLLabelElement;
        return ret;
    };
    imports.wbg.__wbg_sethtmlFor_9576fcd4abb5a60e = function(arg0, arg1, arg2) {
        getObject(arg0).htmlFor = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_warn_5ec7c7c02d0b3841 = function(arg0) {
        console.warn(getObject(arg0));
    };
    imports.wbg.__wbg_instanceof_HtmlElement_df66c8b4a687aa43 = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLElement;
        return ret;
    };
    imports.wbg.__wbg_settitle_9ea3306420a0510b = function(arg0, arg1, arg2) {
        getObject(arg0).title = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setscrollHeight_3224a5ba9bef2079 = function(arg0, arg1) {
        getObject(arg0).scrollHeight = arg1;
    };
    imports.wbg.__wbg_setscrollTop_98a983617301edc6 = function(arg0, arg1) {
        getObject(arg0).scrollTop = arg1;
    };
    imports.wbg.__wbg_setlang_c3b081f8951b48b1 = function(arg0, arg1, arg2) {
        getObject(arg0).lang = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdir_e1987656242e93de = function(arg0, arg1, arg2) {
        getObject(arg0).dir = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setinnerText_4f4ec715a9a131a0 = function(arg0, arg1, arg2) {
        getObject(arg0).innerText = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethidden_8e35dd2030c5f20a = function(arg0, arg1) {
        getObject(arg0).hidden = arg1 !== 0;
    };
    imports.wbg.__wbg_settabIndex_5ad6742502462dcf = function(arg0, arg1) {
        getObject(arg0).tabIndex = arg1;
    };
    imports.wbg.__wbg_setaccessKey_c5f935ac768c2712 = function(arg0, arg1, arg2) {
        getObject(arg0).accessKey = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdraggable_5b28e9ac3a56893e = function(arg0, arg1) {
        getObject(arg0).draggable = arg1 !== 0;
    };
    imports.wbg.__wbg_setcontentEditable_60068de0adf97700 = function(arg0, arg1, arg2) {
        getObject(arg0).contentEditable = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setspellcheck_74f8b005cfe19c52 = function(arg0, arg1) {
        getObject(arg0).spellcheck = arg1 !== 0;
    };
    imports.wbg.__wbg_style_c88e323890d3a091 = function(arg0) {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setoncopy_857fff20ecee9da4 = function(arg0, arg1) {
        getObject(arg0).oncopy = getObject(arg1);
    };
    imports.wbg.__wbg_setoncut_aed0c4e97c1a3b3b = function(arg0, arg1) {
        getObject(arg0).oncut = getObject(arg1);
    };
    imports.wbg.__wbg_setonpaste_fbc9773e2b742775 = function(arg0, arg1) {
        getObject(arg0).onpaste = getObject(arg1);
    };
    imports.wbg.__wbg_setonabort_e274576160f368f7 = function(arg0, arg1) {
        getObject(arg0).onabort = getObject(arg1);
    };
    imports.wbg.__wbg_setonblur_72ea38b1ce86b9db = function(arg0, arg1) {
        getObject(arg0).onblur = getObject(arg1);
    };
    imports.wbg.__wbg_setonfocus_bf07f03c69ef8482 = function(arg0, arg1) {
        getObject(arg0).onfocus = getObject(arg1);
    };
    imports.wbg.__wbg_setonauxclick_19a6bab1eaa7b876 = function(arg0, arg1) {
        getObject(arg0).onauxclick = getObject(arg1);
    };
    imports.wbg.__wbg_setoncanplay_ff41af18eee3fed3 = function(arg0, arg1) {
        getObject(arg0).oncanplay = getObject(arg1);
    };
    imports.wbg.__wbg_setoncanplaythrough_af9215b4878031cc = function(arg0, arg1) {
        getObject(arg0).oncanplaythrough = getObject(arg1);
    };
    imports.wbg.__wbg_setonchange_5811995dcc8d0ca3 = function(arg0, arg1) {
        getObject(arg0).onchange = getObject(arg1);
    };
    imports.wbg.__wbg_setonclick_8da32c8c00a7359b = function(arg0, arg1) {
        getObject(arg0).onclick = getObject(arg1);
    };
    imports.wbg.__wbg_setonclose_6ad668cd460b3604 = function(arg0, arg1) {
        getObject(arg0).onclose = getObject(arg1);
    };
    imports.wbg.__wbg_setoncontextmenu_e5ab31bbc65148d3 = function(arg0, arg1) {
        getObject(arg0).oncontextmenu = getObject(arg1);
    };
    imports.wbg.__wbg_setondblclick_3cf7b33cbded6d24 = function(arg0, arg1) {
        getObject(arg0).ondblclick = getObject(arg1);
    };
    imports.wbg.__wbg_setondrag_7c9c84a4da3005d1 = function(arg0, arg1) {
        getObject(arg0).ondrag = getObject(arg1);
    };
    imports.wbg.__wbg_setondragend_c53d3cdd8ec2a953 = function(arg0, arg1) {
        getObject(arg0).ondragend = getObject(arg1);
    };
    imports.wbg.__wbg_setondragenter_5b2c16f061cedc67 = function(arg0, arg1) {
        getObject(arg0).ondragenter = getObject(arg1);
    };
    imports.wbg.__wbg_setondragexit_7426674c2bf1f9b1 = function(arg0, arg1) {
        getObject(arg0).ondragexit = getObject(arg1);
    };
    imports.wbg.__wbg_setondragleave_3571ebcdee4a0ef4 = function(arg0, arg1) {
        getObject(arg0).ondragleave = getObject(arg1);
    };
    imports.wbg.__wbg_setondragover_5eac803fea8e70fd = function(arg0, arg1) {
        getObject(arg0).ondragover = getObject(arg1);
    };
    imports.wbg.__wbg_setondragstart_a5c9525c1e8c6fad = function(arg0, arg1) {
        getObject(arg0).ondragstart = getObject(arg1);
    };
    imports.wbg.__wbg_setondrop_3160d6a97c5494ba = function(arg0, arg1) {
        getObject(arg0).ondrop = getObject(arg1);
    };
    imports.wbg.__wbg_setondurationchange_0e20f3760198e5d0 = function(arg0, arg1) {
        getObject(arg0).ondurationchange = getObject(arg1);
    };
    imports.wbg.__wbg_setonemptied_e70205a74cc09b85 = function(arg0, arg1) {
        getObject(arg0).onemptied = getObject(arg1);
    };
    imports.wbg.__wbg_setonended_6000ffd9da59aee6 = function(arg0, arg1) {
        getObject(arg0).onended = getObject(arg1);
    };
    imports.wbg.__wbg_setoninput_cc4e41f4c3a2993f = function(arg0, arg1) {
        getObject(arg0).oninput = getObject(arg1);
    };
    imports.wbg.__wbg_setoninvalid_ffce2bbed09c4a80 = function(arg0, arg1) {
        getObject(arg0).oninvalid = getObject(arg1);
    };
    imports.wbg.__wbg_setonkeydown_2dc9676ce045f972 = function(arg0, arg1) {
        getObject(arg0).onkeydown = getObject(arg1);
    };
    imports.wbg.__wbg_setonkeypress_51bbfdde076bdaa1 = function(arg0, arg1) {
        getObject(arg0).onkeypress = getObject(arg1);
    };
    imports.wbg.__wbg_setonkeyup_de3af176783cdf5c = function(arg0, arg1) {
        getObject(arg0).onkeyup = getObject(arg1);
    };
    imports.wbg.__wbg_setonload_2a9b922c65b0f6ae = function(arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    };
    imports.wbg.__wbg_setonloadeddata_e6c2be1026253c8a = function(arg0, arg1) {
        getObject(arg0).onloadeddata = getObject(arg1);
    };
    imports.wbg.__wbg_setonloadedmetadata_adab67b868255e8f = function(arg0, arg1) {
        getObject(arg0).onloadedmetadata = getObject(arg1);
    };
    imports.wbg.__wbg_setonloadend_5288f9beb57f94ff = function(arg0, arg1) {
        getObject(arg0).onloadend = getObject(arg1);
    };
    imports.wbg.__wbg_setonloadstart_a47bf43b116acd65 = function(arg0, arg1) {
        getObject(arg0).onloadstart = getObject(arg1);
    };
    imports.wbg.__wbg_setonmousedown_2a7170820737f7a7 = function(arg0, arg1) {
        getObject(arg0).onmousedown = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseenter_9bb8f5e9f4353e7c = function(arg0, arg1) {
        getObject(arg0).onmouseenter = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseleave_22a5392a2d6f6890 = function(arg0, arg1) {
        getObject(arg0).onmouseleave = getObject(arg1);
    };
    imports.wbg.__wbg_setonmousemove_87602395d265c513 = function(arg0, arg1) {
        getObject(arg0).onmousemove = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseout_3d58238d7a2b8c28 = function(arg0, arg1) {
        getObject(arg0).onmouseout = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseover_b521c1be0bd846d2 = function(arg0, arg1) {
        getObject(arg0).onmouseover = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseup_dbc6ac6770fa9d92 = function(arg0, arg1) {
        getObject(arg0).onmouseup = getObject(arg1);
    };
    imports.wbg.__wbg_setonwheel_814b03d9e862307f = function(arg0, arg1) {
        getObject(arg0).onwheel = getObject(arg1);
    };
    imports.wbg.__wbg_setonpause_1590bd9ba545869d = function(arg0, arg1) {
        getObject(arg0).onpause = getObject(arg1);
    };
    imports.wbg.__wbg_setonplay_c3a31a08486e67a8 = function(arg0, arg1) {
        getObject(arg0).onplay = getObject(arg1);
    };
    imports.wbg.__wbg_setonplaying_0da49152852b7368 = function(arg0, arg1) {
        getObject(arg0).onplaying = getObject(arg1);
    };
    imports.wbg.__wbg_setonprogress_2e325cbdec289a9b = function(arg0, arg1) {
        getObject(arg0).onprogress = getObject(arg1);
    };
    imports.wbg.__wbg_setonratechange_4e58fe4ba4a2ea36 = function(arg0, arg1) {
        getObject(arg0).onratechange = getObject(arg1);
    };
    imports.wbg.__wbg_setonreset_0ab1532a411e7e96 = function(arg0, arg1) {
        getObject(arg0).onreset = getObject(arg1);
    };
    imports.wbg.__wbg_setonresize_100718bd9b079f0c = function(arg0, arg1) {
        getObject(arg0).onresize = getObject(arg1);
    };
    imports.wbg.__wbg_setonscroll_29753bb76e4bb112 = function(arg0, arg1) {
        getObject(arg0).onscroll = getObject(arg1);
    };
    imports.wbg.__wbg_setonseeked_9418b683f1e12e92 = function(arg0, arg1) {
        getObject(arg0).onseeked = getObject(arg1);
    };
    imports.wbg.__wbg_setonseeking_829d3c748b003f39 = function(arg0, arg1) {
        getObject(arg0).onseeking = getObject(arg1);
    };
    imports.wbg.__wbg_setonselect_9c6ab6cdb780daf7 = function(arg0, arg1) {
        getObject(arg0).onselect = getObject(arg1);
    };
    imports.wbg.__wbg_setonshow_619c9ec6349b50ca = function(arg0, arg1) {
        getObject(arg0).onshow = getObject(arg1);
    };
    imports.wbg.__wbg_setonstalled_02ba023c761ead48 = function(arg0, arg1) {
        getObject(arg0).onstalled = getObject(arg1);
    };
    imports.wbg.__wbg_setonsubmit_174ccf94447bbe1b = function(arg0, arg1) {
        getObject(arg0).onsubmit = getObject(arg1);
    };
    imports.wbg.__wbg_setonsuspend_316d0b6abb1e2802 = function(arg0, arg1) {
        getObject(arg0).onsuspend = getObject(arg1);
    };
    imports.wbg.__wbg_setontimeupdate_208e3033475fc666 = function(arg0, arg1) {
        getObject(arg0).ontimeupdate = getObject(arg1);
    };
    imports.wbg.__wbg_setonvolumechange_6cae081a14c9aad6 = function(arg0, arg1) {
        getObject(arg0).onvolumechange = getObject(arg1);
    };
    imports.wbg.__wbg_setonwaiting_2a5e258d66c5d738 = function(arg0, arg1) {
        getObject(arg0).onwaiting = getObject(arg1);
    };
    imports.wbg.__wbg_setonselectstart_9df8482f89e111fa = function(arg0, arg1) {
        getObject(arg0).onselectstart = getObject(arg1);
    };
    imports.wbg.__wbg_setontoggle_c07a88f4c63a7983 = function(arg0, arg1) {
        getObject(arg0).ontoggle = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointercancel_1441a08dfe84b73c = function(arg0, arg1) {
        getObject(arg0).onpointercancel = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerdown_23cd45649fdc0514 = function(arg0, arg1) {
        getObject(arg0).onpointerdown = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerup_ae0dd6bdf3a6f5fc = function(arg0, arg1) {
        getObject(arg0).onpointerup = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointermove_afa5b7a2cd4152d4 = function(arg0, arg1) {
        getObject(arg0).onpointermove = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerout_e1e675caeb5ef089 = function(arg0, arg1) {
        getObject(arg0).onpointerout = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerover_ad07f8f2ab650735 = function(arg0, arg1) {
        getObject(arg0).onpointerover = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerenter_909852823d5611ef = function(arg0, arg1) {
        getObject(arg0).onpointerenter = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerleave_0f3a66bae3f08118 = function(arg0, arg1) {
        getObject(arg0).onpointerleave = getObject(arg1);
    };
    imports.wbg.__wbg_setongotpointercapture_b05b80206c597d93 = function(arg0, arg1) {
        getObject(arg0).ongotpointercapture = getObject(arg1);
    };
    imports.wbg.__wbg_setonlostpointercapture_25beb6505064886c = function(arg0, arg1) {
        getObject(arg0).onlostpointercapture = getObject(arg1);
    };
    imports.wbg.__wbg_setonanimationcancel_0f7f783ab2eea1bc = function(arg0, arg1) {
        getObject(arg0).onanimationcancel = getObject(arg1);
    };
    imports.wbg.__wbg_setonanimationend_31a03f77f5d213a9 = function(arg0, arg1) {
        getObject(arg0).onanimationend = getObject(arg1);
    };
    imports.wbg.__wbg_setonanimationiteration_95814721e26e8fcf = function(arg0, arg1) {
        getObject(arg0).onanimationiteration = getObject(arg1);
    };
    imports.wbg.__wbg_setonanimationstart_25c469e1a0e400fd = function(arg0, arg1) {
        getObject(arg0).onanimationstart = getObject(arg1);
    };
    imports.wbg.__wbg_setontransitioncancel_76be70389c4a3cdc = function(arg0, arg1) {
        getObject(arg0).ontransitioncancel = getObject(arg1);
    };
    imports.wbg.__wbg_setontransitionend_050243f552eed8d3 = function(arg0, arg1) {
        getObject(arg0).ontransitionend = getObject(arg1);
    };
    imports.wbg.__wbg_setontransitionrun_35514eae62344a3e = function(arg0, arg1) {
        getObject(arg0).ontransitionrun = getObject(arg1);
    };
    imports.wbg.__wbg_setontransitionstart_8d02a3d27fa09a24 = function(arg0, arg1) {
        getObject(arg0).ontransitionstart = getObject(arg1);
    };
    imports.wbg.__wbg_setonwebkitanimationend_449d7a11d63d6d78 = function(arg0, arg1) {
        getObject(arg0).onwebkitanimationend = getObject(arg1);
    };
    imports.wbg.__wbg_setonwebkitanimationiteration_8372fc66cc5cd9b5 = function(arg0, arg1) {
        getObject(arg0).onwebkitanimationiteration = getObject(arg1);
    };
    imports.wbg.__wbg_setonwebkitanimationstart_6b2810d493416463 = function(arg0, arg1) {
        getObject(arg0).onwebkitanimationstart = getObject(arg1);
    };
    imports.wbg.__wbg_setonwebkittransitionend_d020133a6f19d79f = function(arg0, arg1) {
        getObject(arg0).onwebkittransitionend = getObject(arg1);
    };
    imports.wbg.__wbg_setonerror_f3fa875044b1d766 = function(arg0, arg1) {
        getObject(arg0).onerror = getObject(arg1);
    };
    imports.wbg.__wbg_setontouchstart_1eecb59b784ee163 = function(arg0, arg1) {
        getObject(arg0).ontouchstart = getObject(arg1);
    };
    imports.wbg.__wbg_setontouchend_470bb243ddc5de41 = function(arg0, arg1) {
        getObject(arg0).ontouchend = getObject(arg1);
    };
    imports.wbg.__wbg_setontouchmove_95368bfbed9e7c96 = function(arg0, arg1) {
        getObject(arg0).ontouchmove = getObject(arg1);
    };
    imports.wbg.__wbg_setontouchcancel_a25cc98d4aba2c55 = function(arg0, arg1) {
        getObject(arg0).ontouchcancel = getObject(arg1);
    };
    imports.wbg.__wbg_focus_00530e359f44fc6e = function() { return handleError(function (arg0) {
        getObject(arg0).focus();
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlAnchorElement_8e5b1b24ff4f5208 = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLAnchorElement;
        return ret;
    };
    imports.wbg.__wbg_settarget_3defe7b07a33381f = function(arg0, arg1, arg2) {
        getObject(arg0).target = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdownload_524e363cfe2b556e = function(arg0, arg1, arg2) {
        getObject(arg0).download = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setping_426101db384fae40 = function(arg0, arg1, arg2) {
        getObject(arg0).ping = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setrel_a78a889e7ba3f712 = function(arg0, arg1, arg2) {
        getObject(arg0).rel = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setreferrerPolicy_3afac071f5ae4c88 = function(arg0, arg1, arg2) {
        getObject(arg0).referrerPolicy = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethreflang_ae6cf92dd822d35f = function(arg0, arg1, arg2) {
        getObject(arg0).hreflang = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settype_e2f553bb4f6dca52 = function(arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setcoords_47075354bf685633 = function(arg0, arg1, arg2) {
        getObject(arg0).coords = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setcharset_0dbf8f97496dd56f = function(arg0, arg1, arg2) {
        getObject(arg0).charset = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setname_0cbe908a6431c92b = function(arg0, arg1, arg2) {
        getObject(arg0).name = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setrev_2cb15d558c2214f4 = function(arg0, arg1, arg2) {
        getObject(arg0).rev = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setshape_e4c9c512ead13d2d = function(arg0, arg1, arg2) {
        getObject(arg0).shape = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethref_712353d0c3a9500a = function(arg0, arg1, arg2) {
        getObject(arg0).href = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setprotocol_e38a476749f7eec6 = function(arg0, arg1, arg2) {
        getObject(arg0).protocol = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setusername_4ffa4aa3c67f20ef = function(arg0, arg1, arg2) {
        getObject(arg0).username = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setpassword_2670c2bf3779876e = function(arg0, arg1, arg2) {
        getObject(arg0).password = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethost_57b9b6b5c7f15ed6 = function(arg0, arg1, arg2) {
        getObject(arg0).host = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethostname_e9ee2f6849c0e0a8 = function(arg0, arg1, arg2) {
        getObject(arg0).hostname = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setport_d9d8b46597d4cd62 = function(arg0, arg1, arg2) {
        getObject(arg0).port = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setpathname_13aee7e0d87a61f9 = function(arg0, arg1, arg2) {
        getObject(arg0).pathname = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setsearch_10822656398806a1 = function(arg0, arg1, arg2) {
        getObject(arg0).search = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethash_5f361c77b5923a6d = function(arg0, arg1, arg2) {
        getObject(arg0).hash = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_instanceof_HtmlInputElement_8cafe5f30dfdb6bc = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLInputElement;
        return ret;
    };
    imports.wbg.__wbg_setaccept_e53de14605da7924 = function(arg0, arg1, arg2) {
        getObject(arg0).accept = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setalt_92ca35252c15369d = function(arg0, arg1, arg2) {
        getObject(arg0).alt = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setautocomplete_3f7d3951409c57a6 = function(arg0, arg1, arg2) {
        getObject(arg0).autocomplete = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setautofocus_5d3aec51de5021e2 = function(arg0, arg1) {
        getObject(arg0).autofocus = arg1 !== 0;
    };
    imports.wbg.__wbg_setdefaultChecked_ffb49d02352dd480 = function(arg0, arg1) {
        getObject(arg0).defaultChecked = arg1 !== 0;
    };
    imports.wbg.__wbg_setchecked_206243371da58f6a = function(arg0, arg1) {
        getObject(arg0).checked = arg1 !== 0;
    };
    imports.wbg.__wbg_setdisabled_5483f0367d980507 = function(arg0, arg1) {
        getObject(arg0).disabled = arg1 !== 0;
    };
    imports.wbg.__wbg_setfiles_5615d8483434f216 = function(arg0, arg1) {
        getObject(arg0).files = getObject(arg1);
    };
    imports.wbg.__wbg_setformAction_90eac97fea49f1f8 = function(arg0, arg1, arg2) {
        getObject(arg0).formAction = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformEnctype_e918de0dc0f85681 = function(arg0, arg1, arg2) {
        getObject(arg0).formEnctype = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformMethod_cd67bc038f43fc65 = function(arg0, arg1, arg2) {
        getObject(arg0).formMethod = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformNoValidate_7ef18da2018feced = function(arg0, arg1) {
        getObject(arg0).formNoValidate = arg1 !== 0;
    };
    imports.wbg.__wbg_setformTarget_d938032b8fa9d989 = function(arg0, arg1, arg2) {
        getObject(arg0).formTarget = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setheight_c1697c0f4fb1be43 = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_setindeterminate_b1720934d7ce3765 = function(arg0, arg1) {
        getObject(arg0).indeterminate = arg1 !== 0;
    };
    imports.wbg.__wbg_setinputMode_f3a1e367f5b08ef8 = function(arg0, arg1, arg2) {
        getObject(arg0).inputMode = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setmax_1da568ca02ce3a91 = function(arg0, arg1, arg2) {
        getObject(arg0).max = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setmaxLength_69e5c3efbb631260 = function(arg0, arg1) {
        getObject(arg0).maxLength = arg1;
    };
    imports.wbg.__wbg_setmin_a7c2b73c5b69d808 = function(arg0, arg1, arg2) {
        getObject(arg0).min = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setminLength_d8fd47b8c9dedc6c = function(arg0, arg1) {
        getObject(arg0).minLength = arg1;
    };
    imports.wbg.__wbg_setmultiple_00b587f9cdafb61a = function(arg0, arg1) {
        getObject(arg0).multiple = arg1 !== 0;
    };
    imports.wbg.__wbg_setname_39e246ad262091cc = function(arg0, arg1, arg2) {
        getObject(arg0).name = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setpattern_19be7273d8a5d605 = function(arg0, arg1, arg2) {
        getObject(arg0).pattern = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setplaceholder_53e28d33645ccd24 = function(arg0, arg1, arg2) {
        getObject(arg0).placeholder = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setreadOnly_b77648c82907d199 = function(arg0, arg1) {
        getObject(arg0).readOnly = arg1 !== 0;
    };
    imports.wbg.__wbg_setrequired_99f3dba5082c08f3 = function(arg0, arg1) {
        getObject(arg0).required = arg1 !== 0;
    };
    imports.wbg.__wbg_setsize_9ec16303ce038acb = function(arg0, arg1) {
        getObject(arg0).size = arg1 >>> 0;
    };
    imports.wbg.__wbg_setsrc_daddd9fabc40781b = function(arg0, arg1, arg2) {
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setstep_8d775e97b173a2c4 = function(arg0, arg1, arg2) {
        getObject(arg0).step = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settype_6a7d0ca3b1b6d0c2 = function(arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdefaultValue_2709f202e4385329 = function(arg0, arg1, arg2) {
        getObject(arg0).defaultValue = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_value_0627d4b1c27534e6 = function(arg0, arg1) {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setvalue_2459f62386b6967f = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setvalueAsNumber_e37e7c1dc8c59617 = function(arg0, arg1) {
        getObject(arg0).valueAsNumber = arg1;
    };
    imports.wbg.__wbg_setwidth_1db606cdeab47202 = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_setalign_cd821ec10b01d276 = function(arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setuseMap_f84a5b3fe06b770d = function(arg0, arg1, arg2) {
        getObject(arg0).useMap = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setwebkitdirectory_74aa21411ad02ee4 = function(arg0, arg1) {
        getObject(arg0).webkitdirectory = arg1 !== 0;
    };
    imports.wbg.__wbg_instanceof_HtmlSpanElement_14de941550b82a8d = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLSpanElement;
        return ret;
    };
    imports.wbg.__wbg_keyCode_490ed69472addfdc = function(arg0) {
        var ret = getObject(arg0).keyCode;
        return ret;
    };
    imports.wbg.__wbg_length_62e6735d81b8b0f1 = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_item_19347f9da4575496 = function(arg0, arg1) {
        var ret = getObject(arg0).item(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_be86524d73f67598 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_888d259a5fefc347 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_is_0f5efc7977a2c50b = function(arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_self_c6fbdfc2918d5e58 = function() { return handleError(function () {
        var ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_baec038b5ab35c54 = function() { return handleError(function () {
        var ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_3f735a5746d41fbd = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_1bc0b39582740e95 = function() { return handleError(function () {
        var ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_export_0, wasm.__wbindgen_export_1);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_closure_wrapper521 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 121, __wbg_adapter_16);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper523 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 121, __wbg_adapter_19);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper525 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 121, __wbg_adapter_22);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper527 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 121, __wbg_adapter_25);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper529 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 121, __wbg_adapter_28);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1149 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 435, __wbg_adapter_31);
        return addHeapObject(ret);
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

