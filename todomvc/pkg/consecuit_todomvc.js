
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
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1d2595e8f6581061(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_19(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1d2595e8f6581061(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_22(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1d2595e8f6581061(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_25(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1d2595e8f6581061(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_28(arg0, arg1) {
    wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc9622def23bab499(arg0, arg1);
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
        wasm.__wbindgen_exn_store(addHeapObject(e));
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
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Window_11e25482011fc506 = function(arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_document_5aff8cd83ef968f5 = function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_location_05eee59b82ccc208 = function(arg0) {
        var ret = getObject(arg0).location;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setonbeforeunload_0e422cfa029af939 = function(arg0, arg1) {
        getObject(arg0).onbeforeunload = getObject(arg1);
    };
    imports.wbg.__wbg_setonhashchange_4872fff2a0da1490 = function(arg0, arg1) {
        getObject(arg0).onhashchange = getObject(arg1);
    };
    imports.wbg.__wbg_localStorage_b787eb9a4418c2b1 = function() { return handleError(function (arg0) {
        var ret = getObject(arg0).localStorage;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setTimeout_57a30fa8c22d1825 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).setTimeout(getObject(arg1));
        return ret;
    }, arguments) };
    imports.wbg.__wbg_body_525168d9e773c3f8 = function(arg0) {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_createDocumentFragment_2fd3aeae2a71f3d4 = function(arg0) {
        var ret = getObject(arg0).createDocumentFragment();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createElement_ac65a6ce60c4812c = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTextNode_442392ad92e75695 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_HtmlHeadingElement_a09b4abfd4de32fc = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLHeadingElement;
        return ret;
    };
    imports.wbg.__wbg_setalign_e580ee82fe649c17 = function(arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_instanceof_HtmlDivElement_819bb57c54982a2f = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLDivElement;
        return ret;
    };
    imports.wbg.__wbg_setalign_f15e6ef8543290ff = function(arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_hash_1496bf468ca4142a = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg1).hash;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_childNodes_936ae129d4f52c02 = function(arg0) {
        var ret = getObject(arg0).childNodes;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setnodeValue_ef88f21c4f2101f5 = function(arg0, arg1, arg2) {
        getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settextContent_2e55253528a044b7 = function(arg0, arg1, arg2) {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_appendChild_6ed236bb79c198df = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_removeChild_f633f19eb895b696 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).removeChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getItem_1c87352132d0b415 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var ret = getObject(arg1).getItem(getStringFromWasm0(arg2, arg3));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    }, arguments) };
    imports.wbg.__wbg_setItem_103b1e46491c9b0e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_setProperty_dccccce3a52c26db = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_setid_cea8de140a58c4f1 = function(arg0, arg1, arg2) {
        getObject(arg0).id = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setclassName_09e9074a6eb1e2cb = function(arg0, arg1, arg2) {
        getObject(arg0).className = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setscrollLeft_545b4a0b773b482f = function(arg0, arg1) {
        getObject(arg0).scrollLeft = arg1;
    };
    imports.wbg.__wbg_setinnerHTML_bd5b74e3148c235e = function(arg0, arg1, arg2) {
        getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setouterHTML_14aff931432eb7f6 = function(arg0, arg1, arg2) {
        getObject(arg0).outerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setslot_f9b01f5da375ec97 = function(arg0, arg1, arg2) {
        getObject(arg0).slot = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_removeAttribute_16e5bf3866aa53e8 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlButtonElement_da6b54269a93893e = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLButtonElement;
        return ret;
    };
    imports.wbg.__wbg_setautofocus_3d30c2e0c5083fd5 = function(arg0, arg1) {
        getObject(arg0).autofocus = arg1 !== 0;
    };
    imports.wbg.__wbg_setdisabled_85de00a9f90d420a = function(arg0, arg1) {
        getObject(arg0).disabled = arg1 !== 0;
    };
    imports.wbg.__wbg_setformAction_5d222cd66fd98ad7 = function(arg0, arg1, arg2) {
        getObject(arg0).formAction = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformEnctype_be3c032b744f17fb = function(arg0, arg1, arg2) {
        getObject(arg0).formEnctype = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformMethod_a0584439c0fe140d = function(arg0, arg1, arg2) {
        getObject(arg0).formMethod = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformNoValidate_6e8e8398032b4d7b = function(arg0, arg1) {
        getObject(arg0).formNoValidate = arg1 !== 0;
    };
    imports.wbg.__wbg_setformTarget_7c442e5e6daad141 = function(arg0, arg1, arg2) {
        getObject(arg0).formTarget = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setname_793a083781e0647b = function(arg0, arg1, arg2) {
        getObject(arg0).name = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settype_985173fd488717c8 = function(arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setvalue_7c5bc0bbf8b74601 = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_warn_b39e749f1dc02058 = function(arg0) {
        console.warn(getObject(arg0));
    };
    imports.wbg.__wbg_instanceof_HtmlElement_835072e813858ac0 = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLElement;
        return ret;
    };
    imports.wbg.__wbg_settitle_2359723f49b6cd01 = function(arg0, arg1, arg2) {
        getObject(arg0).title = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setscrollHeight_81a5f723b9eb351c = function(arg0, arg1) {
        getObject(arg0).scrollHeight = arg1;
    };
    imports.wbg.__wbg_setscrollTop_e47b2652ad013eb8 = function(arg0, arg1) {
        getObject(arg0).scrollTop = arg1;
    };
    imports.wbg.__wbg_setlang_5e037ec7873ed2fb = function(arg0, arg1, arg2) {
        getObject(arg0).lang = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdir_1ad9439bfc9e257d = function(arg0, arg1, arg2) {
        getObject(arg0).dir = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setinnerText_4204a2dcac11f07d = function(arg0, arg1, arg2) {
        getObject(arg0).innerText = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethidden_4e6127e185ecf2df = function(arg0, arg1) {
        getObject(arg0).hidden = arg1 !== 0;
    };
    imports.wbg.__wbg_settabIndex_98adf685073385b6 = function(arg0, arg1) {
        getObject(arg0).tabIndex = arg1;
    };
    imports.wbg.__wbg_setaccessKey_f5cefeeb012b5499 = function(arg0, arg1, arg2) {
        getObject(arg0).accessKey = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdraggable_6b4338cbb3fb2139 = function(arg0, arg1) {
        getObject(arg0).draggable = arg1 !== 0;
    };
    imports.wbg.__wbg_setcontentEditable_838f9bd2143f4160 = function(arg0, arg1, arg2) {
        getObject(arg0).contentEditable = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setspellcheck_72058f164c9dfee6 = function(arg0, arg1) {
        getObject(arg0).spellcheck = arg1 !== 0;
    };
    imports.wbg.__wbg_style_25309daade79abb3 = function(arg0) {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setoncopy_ce2115e8cbed652e = function(arg0, arg1) {
        getObject(arg0).oncopy = getObject(arg1);
    };
    imports.wbg.__wbg_setoncut_1807e4fafedaadea = function(arg0, arg1) {
        getObject(arg0).oncut = getObject(arg1);
    };
    imports.wbg.__wbg_setonpaste_69c65ac336f0aebf = function(arg0, arg1) {
        getObject(arg0).onpaste = getObject(arg1);
    };
    imports.wbg.__wbg_setonabort_322419dcc4e69f0f = function(arg0, arg1) {
        getObject(arg0).onabort = getObject(arg1);
    };
    imports.wbg.__wbg_setonblur_0ca27a76f64aa92c = function(arg0, arg1) {
        getObject(arg0).onblur = getObject(arg1);
    };
    imports.wbg.__wbg_setonfocus_2dc94aaa2254cd7f = function(arg0, arg1) {
        getObject(arg0).onfocus = getObject(arg1);
    };
    imports.wbg.__wbg_setonauxclick_ac5a1220ba4bdf66 = function(arg0, arg1) {
        getObject(arg0).onauxclick = getObject(arg1);
    };
    imports.wbg.__wbg_setoncanplay_e0056a79223b2e12 = function(arg0, arg1) {
        getObject(arg0).oncanplay = getObject(arg1);
    };
    imports.wbg.__wbg_setoncanplaythrough_bc5cc1f2cf14c65b = function(arg0, arg1) {
        getObject(arg0).oncanplaythrough = getObject(arg1);
    };
    imports.wbg.__wbg_setonchange_9d18f3906782a690 = function(arg0, arg1) {
        getObject(arg0).onchange = getObject(arg1);
    };
    imports.wbg.__wbg_setonclick_020a4ab155fe4406 = function(arg0, arg1) {
        getObject(arg0).onclick = getObject(arg1);
    };
    imports.wbg.__wbg_setonclose_75c4cfc8f5a4125d = function(arg0, arg1) {
        getObject(arg0).onclose = getObject(arg1);
    };
    imports.wbg.__wbg_setoncontextmenu_da7201816c727fe5 = function(arg0, arg1) {
        getObject(arg0).oncontextmenu = getObject(arg1);
    };
    imports.wbg.__wbg_setondblclick_f4e5045af3a9ffd4 = function(arg0, arg1) {
        getObject(arg0).ondblclick = getObject(arg1);
    };
    imports.wbg.__wbg_setondrag_83309f012a39d01c = function(arg0, arg1) {
        getObject(arg0).ondrag = getObject(arg1);
    };
    imports.wbg.__wbg_setondragend_dbbf4d46eb3c3a31 = function(arg0, arg1) {
        getObject(arg0).ondragend = getObject(arg1);
    };
    imports.wbg.__wbg_setondragenter_7577ecd0b2045036 = function(arg0, arg1) {
        getObject(arg0).ondragenter = getObject(arg1);
    };
    imports.wbg.__wbg_setondragexit_d5a16662cdb98f33 = function(arg0, arg1) {
        getObject(arg0).ondragexit = getObject(arg1);
    };
    imports.wbg.__wbg_setondragleave_940ec21230fad1a5 = function(arg0, arg1) {
        getObject(arg0).ondragleave = getObject(arg1);
    };
    imports.wbg.__wbg_setondragover_c8679d2ed39c87c8 = function(arg0, arg1) {
        getObject(arg0).ondragover = getObject(arg1);
    };
    imports.wbg.__wbg_setondragstart_0ff9444e17defbb4 = function(arg0, arg1) {
        getObject(arg0).ondragstart = getObject(arg1);
    };
    imports.wbg.__wbg_setondrop_da1a165f51096b19 = function(arg0, arg1) {
        getObject(arg0).ondrop = getObject(arg1);
    };
    imports.wbg.__wbg_setondurationchange_d072fa60de0d5018 = function(arg0, arg1) {
        getObject(arg0).ondurationchange = getObject(arg1);
    };
    imports.wbg.__wbg_setonemptied_fe53c947d9c1bf2d = function(arg0, arg1) {
        getObject(arg0).onemptied = getObject(arg1);
    };
    imports.wbg.__wbg_setonended_50ecad6bcdc3d5cb = function(arg0, arg1) {
        getObject(arg0).onended = getObject(arg1);
    };
    imports.wbg.__wbg_setoninput_95d472f00ada53c5 = function(arg0, arg1) {
        getObject(arg0).oninput = getObject(arg1);
    };
    imports.wbg.__wbg_setoninvalid_9f83dd37f8b9cd77 = function(arg0, arg1) {
        getObject(arg0).oninvalid = getObject(arg1);
    };
    imports.wbg.__wbg_setonkeydown_c790097203253e54 = function(arg0, arg1) {
        getObject(arg0).onkeydown = getObject(arg1);
    };
    imports.wbg.__wbg_setonkeypress_45dc4598fd1dcdc5 = function(arg0, arg1) {
        getObject(arg0).onkeypress = getObject(arg1);
    };
    imports.wbg.__wbg_setonkeyup_53caa01260b4cb4a = function(arg0, arg1) {
        getObject(arg0).onkeyup = getObject(arg1);
    };
    imports.wbg.__wbg_setonload_42a438d19db596f4 = function(arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    };
    imports.wbg.__wbg_setonloadeddata_54e1f809a2ae6cab = function(arg0, arg1) {
        getObject(arg0).onloadeddata = getObject(arg1);
    };
    imports.wbg.__wbg_setonloadedmetadata_df994e7bd3cba1af = function(arg0, arg1) {
        getObject(arg0).onloadedmetadata = getObject(arg1);
    };
    imports.wbg.__wbg_setonloadend_b9c14e889673ca99 = function(arg0, arg1) {
        getObject(arg0).onloadend = getObject(arg1);
    };
    imports.wbg.__wbg_setonloadstart_0832f3f97c32e56a = function(arg0, arg1) {
        getObject(arg0).onloadstart = getObject(arg1);
    };
    imports.wbg.__wbg_setonmousedown_63121051fd919136 = function(arg0, arg1) {
        getObject(arg0).onmousedown = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseenter_feb991d3b04c9f87 = function(arg0, arg1) {
        getObject(arg0).onmouseenter = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseleave_02f0b84f7cf77454 = function(arg0, arg1) {
        getObject(arg0).onmouseleave = getObject(arg1);
    };
    imports.wbg.__wbg_setonmousemove_f734d5bad3680555 = function(arg0, arg1) {
        getObject(arg0).onmousemove = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseout_75ebd17b54191df6 = function(arg0, arg1) {
        getObject(arg0).onmouseout = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseover_3ad5de144204d75f = function(arg0, arg1) {
        getObject(arg0).onmouseover = getObject(arg1);
    };
    imports.wbg.__wbg_setonmouseup_4084e9f945950922 = function(arg0, arg1) {
        getObject(arg0).onmouseup = getObject(arg1);
    };
    imports.wbg.__wbg_setonwheel_efbfb6d62cd53a36 = function(arg0, arg1) {
        getObject(arg0).onwheel = getObject(arg1);
    };
    imports.wbg.__wbg_setonpause_8bdf71065045faa8 = function(arg0, arg1) {
        getObject(arg0).onpause = getObject(arg1);
    };
    imports.wbg.__wbg_setonplay_d980eb079068502c = function(arg0, arg1) {
        getObject(arg0).onplay = getObject(arg1);
    };
    imports.wbg.__wbg_setonplaying_a02212e28e6f125d = function(arg0, arg1) {
        getObject(arg0).onplaying = getObject(arg1);
    };
    imports.wbg.__wbg_setonprogress_6ff420088a0e3da3 = function(arg0, arg1) {
        getObject(arg0).onprogress = getObject(arg1);
    };
    imports.wbg.__wbg_setonratechange_a0b6cc86792ebefa = function(arg0, arg1) {
        getObject(arg0).onratechange = getObject(arg1);
    };
    imports.wbg.__wbg_setonreset_7da0cf853a364abb = function(arg0, arg1) {
        getObject(arg0).onreset = getObject(arg1);
    };
    imports.wbg.__wbg_setonresize_ed3837a2a338ca22 = function(arg0, arg1) {
        getObject(arg0).onresize = getObject(arg1);
    };
    imports.wbg.__wbg_setonscroll_307414ac617372cd = function(arg0, arg1) {
        getObject(arg0).onscroll = getObject(arg1);
    };
    imports.wbg.__wbg_setonseeked_54b276cb8e6d6557 = function(arg0, arg1) {
        getObject(arg0).onseeked = getObject(arg1);
    };
    imports.wbg.__wbg_setonseeking_b76447172ad10b8d = function(arg0, arg1) {
        getObject(arg0).onseeking = getObject(arg1);
    };
    imports.wbg.__wbg_setonselect_fc40e7fc13b9b7df = function(arg0, arg1) {
        getObject(arg0).onselect = getObject(arg1);
    };
    imports.wbg.__wbg_setonshow_5597b327fabce48e = function(arg0, arg1) {
        getObject(arg0).onshow = getObject(arg1);
    };
    imports.wbg.__wbg_setonstalled_1cd0798f654e9bea = function(arg0, arg1) {
        getObject(arg0).onstalled = getObject(arg1);
    };
    imports.wbg.__wbg_setonsubmit_373de0193099432d = function(arg0, arg1) {
        getObject(arg0).onsubmit = getObject(arg1);
    };
    imports.wbg.__wbg_setonsuspend_19414436698873ff = function(arg0, arg1) {
        getObject(arg0).onsuspend = getObject(arg1);
    };
    imports.wbg.__wbg_setontimeupdate_6bb4db01d58704c8 = function(arg0, arg1) {
        getObject(arg0).ontimeupdate = getObject(arg1);
    };
    imports.wbg.__wbg_setonvolumechange_05fc6667a4914c72 = function(arg0, arg1) {
        getObject(arg0).onvolumechange = getObject(arg1);
    };
    imports.wbg.__wbg_setonwaiting_f087995ef368c0af = function(arg0, arg1) {
        getObject(arg0).onwaiting = getObject(arg1);
    };
    imports.wbg.__wbg_setonselectstart_4efd3bfc7379b932 = function(arg0, arg1) {
        getObject(arg0).onselectstart = getObject(arg1);
    };
    imports.wbg.__wbg_setontoggle_d4bdf01975f5148c = function(arg0, arg1) {
        getObject(arg0).ontoggle = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointercancel_404b98c995e1dd4d = function(arg0, arg1) {
        getObject(arg0).onpointercancel = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerdown_ea6ae5e53c1f1b3d = function(arg0, arg1) {
        getObject(arg0).onpointerdown = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerup_113333746ea65f5b = function(arg0, arg1) {
        getObject(arg0).onpointerup = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointermove_85fdfa51628224a2 = function(arg0, arg1) {
        getObject(arg0).onpointermove = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerout_c3aa25229ffb23a4 = function(arg0, arg1) {
        getObject(arg0).onpointerout = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerover_438faede1911b572 = function(arg0, arg1) {
        getObject(arg0).onpointerover = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerenter_6219b6179a05bec0 = function(arg0, arg1) {
        getObject(arg0).onpointerenter = getObject(arg1);
    };
    imports.wbg.__wbg_setonpointerleave_5e5e6f976b697a21 = function(arg0, arg1) {
        getObject(arg0).onpointerleave = getObject(arg1);
    };
    imports.wbg.__wbg_setongotpointercapture_727fca1dc3a5fede = function(arg0, arg1) {
        getObject(arg0).ongotpointercapture = getObject(arg1);
    };
    imports.wbg.__wbg_setonlostpointercapture_665dd4a21d72e259 = function(arg0, arg1) {
        getObject(arg0).onlostpointercapture = getObject(arg1);
    };
    imports.wbg.__wbg_setonanimationcancel_5d92cb546013b624 = function(arg0, arg1) {
        getObject(arg0).onanimationcancel = getObject(arg1);
    };
    imports.wbg.__wbg_setonanimationend_90ddc312a33f74cb = function(arg0, arg1) {
        getObject(arg0).onanimationend = getObject(arg1);
    };
    imports.wbg.__wbg_setonanimationiteration_8f75eb350a350f1c = function(arg0, arg1) {
        getObject(arg0).onanimationiteration = getObject(arg1);
    };
    imports.wbg.__wbg_setonanimationstart_ad27d0df0544d3ce = function(arg0, arg1) {
        getObject(arg0).onanimationstart = getObject(arg1);
    };
    imports.wbg.__wbg_setontransitioncancel_b6dc8018c196a513 = function(arg0, arg1) {
        getObject(arg0).ontransitioncancel = getObject(arg1);
    };
    imports.wbg.__wbg_setontransitionend_9a1adaca071f66f2 = function(arg0, arg1) {
        getObject(arg0).ontransitionend = getObject(arg1);
    };
    imports.wbg.__wbg_setontransitionrun_82e48ec32fe9f187 = function(arg0, arg1) {
        getObject(arg0).ontransitionrun = getObject(arg1);
    };
    imports.wbg.__wbg_setontransitionstart_beb0f874eb78ce66 = function(arg0, arg1) {
        getObject(arg0).ontransitionstart = getObject(arg1);
    };
    imports.wbg.__wbg_setonwebkitanimationend_9b7d6527c49efa9f = function(arg0, arg1) {
        getObject(arg0).onwebkitanimationend = getObject(arg1);
    };
    imports.wbg.__wbg_setonwebkitanimationiteration_0c0c66e3dddf7296 = function(arg0, arg1) {
        getObject(arg0).onwebkitanimationiteration = getObject(arg1);
    };
    imports.wbg.__wbg_setonwebkitanimationstart_8bb5f687e6338bd2 = function(arg0, arg1) {
        getObject(arg0).onwebkitanimationstart = getObject(arg1);
    };
    imports.wbg.__wbg_setonwebkittransitionend_43779adc18dca07a = function(arg0, arg1) {
        getObject(arg0).onwebkittransitionend = getObject(arg1);
    };
    imports.wbg.__wbg_setonerror_61618db3d13ead14 = function(arg0, arg1) {
        getObject(arg0).onerror = getObject(arg1);
    };
    imports.wbg.__wbg_setontouchstart_bb83c005752b2a5f = function(arg0, arg1) {
        getObject(arg0).ontouchstart = getObject(arg1);
    };
    imports.wbg.__wbg_setontouchend_a5d45b44d40c1f6a = function(arg0, arg1) {
        getObject(arg0).ontouchend = getObject(arg1);
    };
    imports.wbg.__wbg_setontouchmove_c16f6766566c609e = function(arg0, arg1) {
        getObject(arg0).ontouchmove = getObject(arg1);
    };
    imports.wbg.__wbg_setontouchcancel_6addcf924da2d51c = function(arg0, arg1) {
        getObject(arg0).ontouchcancel = getObject(arg1);
    };
    imports.wbg.__wbg_focus_2fac919cca20d33b = function() { return handleError(function (arg0) {
        getObject(arg0).focus();
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlInputElement_df6fbc606ba24e20 = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLInputElement;
        return ret;
    };
    imports.wbg.__wbg_setaccept_7cf030ecd4c69313 = function(arg0, arg1, arg2) {
        getObject(arg0).accept = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setalt_27d0c19a96bc1b64 = function(arg0, arg1, arg2) {
        getObject(arg0).alt = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setautocomplete_6aad3132b61ab62f = function(arg0, arg1, arg2) {
        getObject(arg0).autocomplete = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setautofocus_700ebffe64c7c2a3 = function(arg0, arg1) {
        getObject(arg0).autofocus = arg1 !== 0;
    };
    imports.wbg.__wbg_setdefaultChecked_a0627d72402a75fb = function(arg0, arg1) {
        getObject(arg0).defaultChecked = arg1 !== 0;
    };
    imports.wbg.__wbg_setchecked_dc7daac77dc0e73e = function(arg0, arg1) {
        getObject(arg0).checked = arg1 !== 0;
    };
    imports.wbg.__wbg_setdisabled_1f38a7ba42cfe924 = function(arg0, arg1) {
        getObject(arg0).disabled = arg1 !== 0;
    };
    imports.wbg.__wbg_setfiles_e045c6ca7db73f9a = function(arg0, arg1) {
        getObject(arg0).files = getObject(arg1);
    };
    imports.wbg.__wbg_setformAction_63906e26003af1bd = function(arg0, arg1, arg2) {
        getObject(arg0).formAction = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformEnctype_d8f7dc02b74178f7 = function(arg0, arg1, arg2) {
        getObject(arg0).formEnctype = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformMethod_f1fb4aa5e49858f8 = function(arg0, arg1, arg2) {
        getObject(arg0).formMethod = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setformNoValidate_3ef1f3ea2d3dfb00 = function(arg0, arg1) {
        getObject(arg0).formNoValidate = arg1 !== 0;
    };
    imports.wbg.__wbg_setformTarget_2590db13f278d70e = function(arg0, arg1, arg2) {
        getObject(arg0).formTarget = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setheight_52afe6338676aa47 = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_setindeterminate_3e3b14f33a894970 = function(arg0, arg1) {
        getObject(arg0).indeterminate = arg1 !== 0;
    };
    imports.wbg.__wbg_setinputMode_fc7a0c7c50846bb9 = function(arg0, arg1, arg2) {
        getObject(arg0).inputMode = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setmax_d1e3c7235a14b517 = function(arg0, arg1, arg2) {
        getObject(arg0).max = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setmaxLength_e80397cb9ea81c9b = function(arg0, arg1) {
        getObject(arg0).maxLength = arg1;
    };
    imports.wbg.__wbg_setmin_462666736f23ed1f = function(arg0, arg1, arg2) {
        getObject(arg0).min = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setminLength_0bc20af9862365d0 = function(arg0, arg1) {
        getObject(arg0).minLength = arg1;
    };
    imports.wbg.__wbg_setmultiple_96bb3556c7b59a3a = function(arg0, arg1) {
        getObject(arg0).multiple = arg1 !== 0;
    };
    imports.wbg.__wbg_setname_ba8d8e0de86024ff = function(arg0, arg1, arg2) {
        getObject(arg0).name = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setpattern_1f81e4f4fe6314c0 = function(arg0, arg1, arg2) {
        getObject(arg0).pattern = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setplaceholder_147cc90b66c5bc49 = function(arg0, arg1, arg2) {
        getObject(arg0).placeholder = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setreadOnly_32fdbe1ee2751bef = function(arg0, arg1) {
        getObject(arg0).readOnly = arg1 !== 0;
    };
    imports.wbg.__wbg_setrequired_596985f627f22c34 = function(arg0, arg1) {
        getObject(arg0).required = arg1 !== 0;
    };
    imports.wbg.__wbg_setsize_b6692780c79d2467 = function(arg0, arg1) {
        getObject(arg0).size = arg1 >>> 0;
    };
    imports.wbg.__wbg_setsrc_570ff241d8e79d34 = function(arg0, arg1, arg2) {
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setstep_32670596f76d86c9 = function(arg0, arg1, arg2) {
        getObject(arg0).step = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settype_bd7de9ca642dc3b2 = function(arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdefaultValue_ebcb8c28d9d5901a = function(arg0, arg1, arg2) {
        getObject(arg0).defaultValue = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_value_f4c762446c572119 = function(arg0, arg1) {
        var ret = getObject(arg1).value;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_setvalue_65a652cfd99c8a4a = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setvalueAsNumber_84bb389cbb16676a = function(arg0, arg1) {
        getObject(arg0).valueAsNumber = arg1;
    };
    imports.wbg.__wbg_setwidth_7b1072d77abbc441 = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_setalign_8c91004a795d3db1 = function(arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setuseMap_42ce152ff9178949 = function(arg0, arg1, arg2) {
        getObject(arg0).useMap = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setwebkitdirectory_1c1eb8e41ce6e22d = function(arg0, arg1) {
        getObject(arg0).webkitdirectory = arg1 !== 0;
    };
    imports.wbg.__wbg_instanceof_HtmlLabelElement_ee5ce1f35fb00156 = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLLabelElement;
        return ret;
    };
    imports.wbg.__wbg_sethtmlFor_ec05d208238426fc = function(arg0, arg1, arg2) {
        getObject(arg0).htmlFor = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_instanceof_HtmlAnchorElement_fb8c991ea5f60f22 = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLAnchorElement;
        return ret;
    };
    imports.wbg.__wbg_settarget_33febf3355b89a48 = function(arg0, arg1, arg2) {
        getObject(arg0).target = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setdownload_821e219b873e279b = function(arg0, arg1, arg2) {
        getObject(arg0).download = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setping_465940a72b81538e = function(arg0, arg1, arg2) {
        getObject(arg0).ping = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setrel_48725176210a9cc2 = function(arg0, arg1, arg2) {
        getObject(arg0).rel = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setreferrerPolicy_ad838f109959cf05 = function(arg0, arg1, arg2) {
        getObject(arg0).referrerPolicy = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethreflang_ffbf030914398087 = function(arg0, arg1, arg2) {
        getObject(arg0).hreflang = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settype_72662d77935d04ad = function(arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setcoords_f49574c0384261fd = function(arg0, arg1, arg2) {
        getObject(arg0).coords = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setcharset_7c22db6f1f2d28e3 = function(arg0, arg1, arg2) {
        getObject(arg0).charset = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setname_f450ebfabe6b7575 = function(arg0, arg1, arg2) {
        getObject(arg0).name = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setrev_782baf98ac49ac23 = function(arg0, arg1, arg2) {
        getObject(arg0).rev = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setshape_dd568de71d9deca6 = function(arg0, arg1, arg2) {
        getObject(arg0).shape = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethref_1056ddb16724e3a3 = function(arg0, arg1, arg2) {
        getObject(arg0).href = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setprotocol_68490fc1d5de1464 = function(arg0, arg1, arg2) {
        getObject(arg0).protocol = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setusername_6522e40b9d25d443 = function(arg0, arg1, arg2) {
        getObject(arg0).username = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setpassword_fd07040146fef812 = function(arg0, arg1, arg2) {
        getObject(arg0).password = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethost_8da5e325478c741d = function(arg0, arg1, arg2) {
        getObject(arg0).host = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethostname_55e195d2f719e606 = function(arg0, arg1, arg2) {
        getObject(arg0).hostname = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setport_2c9046beac2e0646 = function(arg0, arg1, arg2) {
        getObject(arg0).port = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setpathname_a59c0342c23911ee = function(arg0, arg1, arg2) {
        getObject(arg0).pathname = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setsearch_429d07898e70cae4 = function(arg0, arg1, arg2) {
        getObject(arg0).search = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_sethash_8d1af1e09feaee6f = function(arg0, arg1, arg2) {
        getObject(arg0).hash = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_keyCode_218ac9c01e06b3d5 = function(arg0) {
        var ret = getObject(arg0).keyCode;
        return ret;
    };
    imports.wbg.__wbg_length_5dff05fb57a644be = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_item_5385cba946ede1be = function(arg0, arg1) {
        var ret = getObject(arg0).item(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_HtmlSpanElement_08d45b315b2f7f1b = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLSpanElement;
        return ret;
    };
    imports.wbg.__wbg_call_ba36642bd901572b = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newnoargs_9fdd8f3961dd1bee = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_is_a973b4c0e9019083 = function(arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_self_bb69a836a72ec6e9 = function() { return handleError(function () {
        var ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_3304fc4b414c9693 = function() { return handleError(function () {
        var ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_e0d21cabc6630763 = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_8463719227271676 = function() { return handleError(function () {
        var ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_closure_wrapper651 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 136, __wbg_adapter_16);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper653 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 136, __wbg_adapter_19);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper655 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 136, __wbg_adapter_22);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper657 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 136, __wbg_adapter_25);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper876 = function(arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 200, __wbg_adapter_28);
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

