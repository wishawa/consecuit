
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

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

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error('expected a boolean argument');
    }
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

    if (typeof(arg) !== 'string') throw new Error('expected a string argument');

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
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
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

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error('expected a number argument');
}
function __wbg_adapter_18(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h75adec3e30ed1d55(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_21(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h858525fa6d89d56e(arg0, arg1);
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
        input = new URL('minimal_counter_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Window_11e25482011fc506 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof Window;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_document_5aff8cd83ef968f5 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setTimeout_57a30fa8c22d1825 = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).setTimeout(getObject(arg1));
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_body_525168d9e773c3f8 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).body;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createElement_ac65a6ce60c4812c = function() { return handleError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createTextNode_442392ad92e75695 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setProperty_dccccce3a52c26db = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlTableElement_78f21b7382f6a1cc = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof HTMLTableElement;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setcaption_1074df2cb3792974 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).caption = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_settHead_8162a2cd8819f5ab = function() { return logError(function (arg0, arg1) {
        getObject(arg0).tHead = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_settFoot_4d228b53a7eb10d0 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).tFoot = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setalign_46d379e27d99e3bb = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setborder_0bece12f5b07376b = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).border = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setframe_c1e872167fbcb35a = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).frame = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setrules_58012cdd147c2360 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).rules = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setsummary_9cedac5721ce72b2 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).summary = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setwidth_3528df7f0c363d2d = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).width = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setbgColor_935d3a5c52193306 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bgColor = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setcellPadding_d711bea8b3ab09f3 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).cellPadding = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setcellSpacing_942c0fb990754007 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).cellSpacing = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setnodeValue_ef88f21c4f2101f5 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).nodeValue = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_settextContent_2e55253528a044b7 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_appendChild_6ed236bb79c198df = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlButtonElement_da6b54269a93893e = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof HTMLButtonElement;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setautofocus_3d30c2e0c5083fd5 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).autofocus = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setdisabled_85de00a9f90d420a = function() { return logError(function (arg0, arg1) {
        getObject(arg0).disabled = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setformAction_5d222cd66fd98ad7 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).formAction = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setformEnctype_be3c032b744f17fb = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).formEnctype = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setformMethod_a0584439c0fe140d = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).formMethod = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setformNoValidate_6e8e8398032b4d7b = function() { return logError(function (arg0, arg1) {
        getObject(arg0).formNoValidate = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setformTarget_7c442e5e6daad141 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).formTarget = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setname_793a083781e0647b = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).name = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_settype_985173fd488717c8 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).type = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setvalue_7c5bc0bbf8b74601 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setid_cea8de140a58c4f1 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).id = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setclassName_09e9074a6eb1e2cb = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).className = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setscrollLeft_545b4a0b773b482f = function() { return logError(function (arg0, arg1) {
        getObject(arg0).scrollLeft = arg1;
    }, arguments) };
    imports.wbg.__wbg_setinnerHTML_bd5b74e3148c235e = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setouterHTML_14aff931432eb7f6 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).outerHTML = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setslot_f9b01f5da375ec97 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).slot = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_removeAttribute_16e5bf3866aa53e8 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
    }, arguments) };
    imports.wbg.__wbg_warn_b39e749f1dc02058 = function() { return logError(function (arg0) {
        console.warn(getObject(arg0));
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlTableCellElement_1404a743237d79f3 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof HTMLTableCellElement;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setcolSpan_75299093a4242556 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).colSpan = arg1 >>> 0;
    }, arguments) };
    imports.wbg.__wbg_setrowSpan_42a887e1be21e94e = function() { return logError(function (arg0, arg1) {
        getObject(arg0).rowSpan = arg1 >>> 0;
    }, arguments) };
    imports.wbg.__wbg_setheaders_80c6793d565bf06b = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).headers = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setalign_225359812242ab9e = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setaxis_e800b7b9ec685882 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).axis = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setheight_7f879068f8f3c38b = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).height = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setwidth_27e28ea85ac9552e = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).width = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setch_c1ba338fbaaf4490 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).ch = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setchOff_76416554316d8c25 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).chOff = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setnoWrap_4fe1ac7dccd48b91 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).noWrap = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setvAlign_f0cfbea47d67f42d = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).vAlign = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setbgColor_1e02cde6ef6256fa = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bgColor = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlElement_835072e813858ac0 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof HTMLElement;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_settitle_2359723f49b6cd01 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).title = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setscrollHeight_81a5f723b9eb351c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).scrollHeight = arg1;
    }, arguments) };
    imports.wbg.__wbg_setscrollTop_e47b2652ad013eb8 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).scrollTop = arg1;
    }, arguments) };
    imports.wbg.__wbg_setlang_5e037ec7873ed2fb = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).lang = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setdir_1ad9439bfc9e257d = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).dir = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setinnerText_4204a2dcac11f07d = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).innerText = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_sethidden_4e6127e185ecf2df = function() { return logError(function (arg0, arg1) {
        getObject(arg0).hidden = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_settabIndex_98adf685073385b6 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).tabIndex = arg1;
    }, arguments) };
    imports.wbg.__wbg_setaccessKey_f5cefeeb012b5499 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).accessKey = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setdraggable_6b4338cbb3fb2139 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).draggable = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_setcontentEditable_838f9bd2143f4160 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).contentEditable = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setspellcheck_72058f164c9dfee6 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).spellcheck = arg1 !== 0;
    }, arguments) };
    imports.wbg.__wbg_style_25309daade79abb3 = function() { return logError(function (arg0) {
        var ret = getObject(arg0).style;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setoncopy_ce2115e8cbed652e = function() { return logError(function (arg0, arg1) {
        getObject(arg0).oncopy = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setoncut_1807e4fafedaadea = function() { return logError(function (arg0, arg1) {
        getObject(arg0).oncut = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpaste_69c65ac336f0aebf = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpaste = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonabort_322419dcc4e69f0f = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onabort = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonblur_0ca27a76f64aa92c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onblur = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonfocus_2dc94aaa2254cd7f = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onfocus = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonauxclick_ac5a1220ba4bdf66 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onauxclick = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setoncanplay_e0056a79223b2e12 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).oncanplay = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setoncanplaythrough_bc5cc1f2cf14c65b = function() { return logError(function (arg0, arg1) {
        getObject(arg0).oncanplaythrough = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonchange_9d18f3906782a690 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onchange = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonclick_020a4ab155fe4406 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onclick = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonclose_75c4cfc8f5a4125d = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onclose = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setoncontextmenu_da7201816c727fe5 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).oncontextmenu = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondblclick_f4e5045af3a9ffd4 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondblclick = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondrag_83309f012a39d01c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondrag = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondragend_dbbf4d46eb3c3a31 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondragend = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondragenter_7577ecd0b2045036 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondragenter = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondragexit_d5a16662cdb98f33 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondragexit = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondragleave_940ec21230fad1a5 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondragleave = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondragover_c8679d2ed39c87c8 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondragover = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondragstart_0ff9444e17defbb4 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondragstart = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondrop_da1a165f51096b19 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondrop = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setondurationchange_d072fa60de0d5018 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ondurationchange = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonemptied_fe53c947d9c1bf2d = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onemptied = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonended_50ecad6bcdc3d5cb = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onended = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setoninput_95d472f00ada53c5 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).oninput = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setoninvalid_9f83dd37f8b9cd77 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).oninvalid = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonkeydown_c790097203253e54 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onkeydown = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonkeypress_45dc4598fd1dcdc5 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onkeypress = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonkeyup_53caa01260b4cb4a = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onkeyup = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonload_42a438d19db596f4 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonloadeddata_54e1f809a2ae6cab = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onloadeddata = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonloadedmetadata_df994e7bd3cba1af = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onloadedmetadata = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonloadend_b9c14e889673ca99 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onloadend = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonloadstart_0832f3f97c32e56a = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onloadstart = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonmousedown_63121051fd919136 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onmousedown = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonmouseenter_feb991d3b04c9f87 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onmouseenter = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonmouseleave_02f0b84f7cf77454 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onmouseleave = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonmousemove_f734d5bad3680555 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onmousemove = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonmouseout_75ebd17b54191df6 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onmouseout = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonmouseover_3ad5de144204d75f = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onmouseover = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonmouseup_4084e9f945950922 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onmouseup = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonwheel_efbfb6d62cd53a36 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onwheel = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpause_8bdf71065045faa8 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpause = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonplay_d980eb079068502c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onplay = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonplaying_a02212e28e6f125d = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onplaying = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonprogress_6ff420088a0e3da3 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onprogress = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonratechange_a0b6cc86792ebefa = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onratechange = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonreset_7da0cf853a364abb = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onreset = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonresize_ed3837a2a338ca22 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onresize = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonscroll_307414ac617372cd = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onscroll = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonseeked_54b276cb8e6d6557 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onseeked = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonseeking_b76447172ad10b8d = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onseeking = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonselect_fc40e7fc13b9b7df = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onselect = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonshow_5597b327fabce48e = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onshow = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonstalled_1cd0798f654e9bea = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onstalled = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonsubmit_373de0193099432d = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onsubmit = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonsuspend_19414436698873ff = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onsuspend = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontimeupdate_6bb4db01d58704c8 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontimeupdate = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonvolumechange_05fc6667a4914c72 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onvolumechange = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonwaiting_f087995ef368c0af = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onwaiting = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonselectstart_4efd3bfc7379b932 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onselectstart = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontoggle_d4bdf01975f5148c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontoggle = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpointercancel_404b98c995e1dd4d = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpointercancel = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpointerdown_ea6ae5e53c1f1b3d = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpointerdown = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpointerup_113333746ea65f5b = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpointerup = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpointermove_85fdfa51628224a2 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpointermove = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpointerout_c3aa25229ffb23a4 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpointerout = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpointerover_438faede1911b572 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpointerover = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpointerenter_6219b6179a05bec0 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpointerenter = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonpointerleave_5e5e6f976b697a21 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onpointerleave = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setongotpointercapture_727fca1dc3a5fede = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ongotpointercapture = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonlostpointercapture_665dd4a21d72e259 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onlostpointercapture = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonanimationcancel_5d92cb546013b624 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onanimationcancel = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonanimationend_90ddc312a33f74cb = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onanimationend = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonanimationiteration_8f75eb350a350f1c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onanimationiteration = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonanimationstart_ad27d0df0544d3ce = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onanimationstart = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontransitioncancel_b6dc8018c196a513 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontransitioncancel = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontransitionend_9a1adaca071f66f2 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontransitionend = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontransitionrun_82e48ec32fe9f187 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontransitionrun = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontransitionstart_beb0f874eb78ce66 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontransitionstart = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonwebkitanimationend_9b7d6527c49efa9f = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onwebkitanimationend = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonwebkitanimationiteration_0c0c66e3dddf7296 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onwebkitanimationiteration = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonwebkitanimationstart_8bb5f687e6338bd2 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onwebkitanimationstart = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonwebkittransitionend_43779adc18dca07a = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onwebkittransitionend = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setonerror_61618db3d13ead14 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onerror = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontouchstart_bb83c005752b2a5f = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontouchstart = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontouchend_a5d45b44d40c1f6a = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontouchend = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontouchmove_c16f6766566c609e = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontouchmove = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setontouchcancel_6addcf924da2d51c = function() { return logError(function (arg0, arg1) {
        getObject(arg0).ontouchcancel = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlTableRowElement_f6d6b604d8f4e818 = function() { return logError(function (arg0) {
        var ret = getObject(arg0) instanceof HTMLTableRowElement;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_setalign_5329443e0ede1e86 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).align = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setch_d165e586bb907fe2 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).ch = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setchOff_5088570aa2579801 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).chOff = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setvAlign_147156ed26ab2778 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).vAlign = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_setbgColor_410b5b8a45d180d9 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).bgColor = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_newnoargs_9fdd8f3961dd1bee = function() { return logError(function (arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_ba36642bd901572b = function() { return handleError(function (arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_is_a973b4c0e9019083 = function() { return logError(function (arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_globalThis_e0d21cabc6630763 = function() { return handleError(function () {
        var ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_bb69a836a72ec6e9 = function() { return handleError(function () {
        var ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_3304fc4b414c9693 = function() { return handleError(function () {
        var ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_8463719227271676 = function() { return handleError(function () {
        var ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
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
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };
    imports.wbg.__wbindgen_closure_wrapper618 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 80, __wbg_adapter_18);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper3225 = function() { return logError(function (arg0, arg1, arg2) {
        var ret = makeClosure(arg0, arg1, 115, __wbg_adapter_21);
        return addHeapObject(ret);
    }, arguments) };

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

