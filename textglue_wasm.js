(function() {
    const __exports = {};
    let wasm;

    let cachedTextDecoder = new TextDecoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    function getStringFromWasm(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    __exports.__wbg_alert_0a363abae8b47797 = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);
        alert(varg0);
    };
    /**
    * @returns {void}
    */
    __exports.greet = function() {
        return wasm.greet();
    };

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

    let stack_pointer = 32;

    function addBorrowedObject(obj) {
        if (stack_pointer == 1) throw new Error('out of js stack');
        heap[--stack_pointer] = obj;
        return stack_pointer;
    }

    let cachedGlobalArgumentPtr = null;
    function globalArgumentPtr() {
        if (cachedGlobalArgumentPtr === null) {
            cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
        }
        return cachedGlobalArgumentPtr;
    }

    let cachegetUint32Memory = null;
    function getUint32Memory() {
        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
        }
        return cachegetUint32Memory;
    }
    /**
    * @param {any} new_db
    * @returns {string}
    */
    __exports.set_database = function(new_db) {
        const retptr = globalArgumentPtr();
        try {
            wasm.set_database(retptr, addBorrowedObject(new_db));
            const mem = getUint32Memory();
            const rustptr = mem[retptr / 4];
            const rustlen = mem[retptr / 4 + 1];

            const realRet = getStringFromWasm(rustptr, rustlen).slice();
            wasm.__wbindgen_free(rustptr, rustlen * 1);
            return realRet;


        } finally {
            heap[stack_pointer++] = undefined;

        }

    };

    let cachedTextEncoder = new TextEncoder('utf-8');

    let WASM_VECTOR_LEN = 0;

    let passStringToWasm;
    if (typeof cachedTextEncoder.encodeInto === 'function') {
        passStringToWasm = function(arg) {

            let size = arg.length;
            let ptr = wasm.__wbindgen_malloc(size);
            let writeOffset = 0;
            while (true) {
                const view = getUint8Memory().subarray(ptr + writeOffset, ptr + size);
                const { read, written } = cachedTextEncoder.encodeInto(arg, view);
                arg = arg.substring(read);
                writeOffset += written;
                if (arg.length === 0) {
                    break;
                }
                ptr = wasm.__wbindgen_realloc(ptr, size, size * 2);
                size *= 2;
            }
            WASM_VECTOR_LEN = writeOffset;
            return ptr;
        };
    } else {
        passStringToWasm = function(arg) {

            const buf = cachedTextEncoder.encode(arg);
            const ptr = wasm.__wbindgen_malloc(buf.length);
            getUint8Memory().set(buf, ptr);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        };
    }
    /**
    * @param {string} json
    * @returns {string}
    */
    __exports.set_database_json = function(json) {
        const ptr0 = passStringToWasm(json);
        const len0 = WASM_VECTOR_LEN;
        const retptr = globalArgumentPtr();
        try {
            wasm.set_database_json(retptr, ptr0, len0);
            const mem = getUint32Memory();
            const rustptr = mem[retptr / 4];
            const rustlen = mem[retptr / 4 + 1];

            const realRet = getStringFromWasm(rustptr, rustlen).slice();
            wasm.__wbindgen_free(rustptr, rustlen * 1);
            return realRet;


        } finally {
            wasm.__wbindgen_free(ptr0, len0 * 1);

        }

    };

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

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
/**
* @returns {any}
*/
__exports.get_database = function() {
    return takeObject(wasm.get_database());
};

/**
* @returns {string}
*/
__exports.get_database_json = function() {
    const retptr = globalArgumentPtr();
    wasm.get_database_json(retptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

};

/**
* @returns {string}
*/
__exports.get_database_pretty_json = function() {
    const retptr = globalArgumentPtr();
    wasm.get_database_pretty_json(retptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

};

/**
* @param {string} id
* @returns {string}
*/
__exports.get_snippet = function(id) {
    const ptr0 = passStringToWasm(id);
    const len0 = WASM_VECTOR_LEN;
    const retptr = globalArgumentPtr();
    try {
        wasm.get_snippet(retptr, ptr0, len0);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];
        if (rustptr === 0) return;
        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;


    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

/**
* @param {string} id
* @returns {boolean}
*/
__exports.contains_snippet = function(id) {
    const ptr0 = passStringToWasm(id);
    const len0 = WASM_VECTOR_LEN;
    try {
        return (wasm.contains_snippet(ptr0, len0)) !== 0;

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

/**
* @param {string} id
* @param {string} text
* @returns {void}
*/
__exports.set_snippet = function(id, text) {
    const ptr0 = passStringToWasm(id);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm(text);
    const len1 = WASM_VECTOR_LEN;
    try {
        return wasm.set_snippet(ptr0, len0, ptr1, len1);

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
        wasm.__wbindgen_free(ptr1, len1 * 1);

    }

};

/**
* @param {string} id
* @returns {any}
*/
__exports.snippet_ids = function(id) {
    const ptr0 = passStringToWasm(id);
    const len0 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.snippet_ids(ptr0, len0));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

/**
* @returns {any}
*/
__exports.get_metadata = function() {
    return takeObject(wasm.get_metadata());
};

/**
* @param {string} id
* @param {any} metadata
* @returns {string}
*/
__exports.set_metadata = function(id, metadata) {
    const ptr0 = passStringToWasm(id);
    const len0 = WASM_VECTOR_LEN;
    const retptr = globalArgumentPtr();
    try {
        wasm.set_metadata(retptr, ptr0, len0, addBorrowedObject(metadata));
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;


    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
        heap[stack_pointer++] = undefined;

    }

};

/**
* @returns {any}
*/
__exports.get_documents = function() {
    return takeObject(wasm.get_documents());
};

/**
* @param {string} name
* @returns {any}
*/
__exports.new_document_autoid = function(name) {
    const ptr0 = passStringToWasm(name);
    const len0 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.new_document_autoid(ptr0, len0));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

/**
* @param {string} name
* @returns {any}
*/
__exports.get_document = function(name) {
    const ptr0 = passStringToWasm(name);
    const len0 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.get_document(ptr0, len0));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

/**
* @param {string} id
* @returns {string}
*/
__exports.new_snippet = function(id) {
    const ptr0 = passStringToWasm(id);
    const len0 = WASM_VECTOR_LEN;
    const retptr = globalArgumentPtr();
    try {
        wasm.new_snippet(retptr, ptr0, len0);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;


    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

/**
* @param {string} document
* @returns {any}
*/
__exports.add_chapter_autoname = function(document) {
    const ptr0 = passStringToWasm(document);
    const len0 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.add_chapter_autoname(ptr0, len0));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

/**
* @param {string} document
* @param {number} i
* @returns {any}
*/
__exports.get_chapter = function(document, i) {
    const ptr0 = passStringToWasm(document);
    const len0 = WASM_VECTOR_LEN;
    try {
        return takeObject(wasm.get_chapter(ptr0, len0, i));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
* @param {string} document
* @param {number} i
* @param {any} chapter
* @returns {string}
*/
__exports.set_chapter = function(document, i, chapter) {
    const ptr0 = passStringToWasm(document);
    const len0 = WASM_VECTOR_LEN;
    const retptr = globalArgumentPtr();
    try {
        wasm.set_chapter(retptr, ptr0, len0, i, addHeapObject(chapter));
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;


    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

};

/**
* @param {string} document
* @param {number} i
* @param {string} id_prefix
* @param {string} id_postfix
* @returns {string}
*/
__exports.get_chapter_text = function(document, i, id_prefix, id_postfix) {
    const ptr0 = passStringToWasm(document);
    const len0 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm(id_prefix);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm(id_postfix);
    const len3 = WASM_VECTOR_LEN;
    const retptr = globalArgumentPtr();
    try {
        wasm.get_chapter_text(retptr, ptr0, len0, i, ptr2, len2, ptr3, len3);
        const mem = getUint32Memory();
        const rustptr = mem[retptr / 4];
        const rustlen = mem[retptr / 4 + 1];

        const realRet = getStringFromWasm(rustptr, rustlen).slice();
        wasm.__wbindgen_free(rustptr, rustlen * 1);
        return realRet;


    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
        wasm.__wbindgen_free(ptr2, len2 * 1);
        wasm.__wbindgen_free(ptr3, len3 * 1);

    }

};

/**
* @param {string} document
* @param {number} i
* @param {string} id_prefix
* @param {string} id_postfix
* @param {string} text
* @returns {void}
*/
__exports.set_chapter_text = function(document, i, id_prefix, id_postfix, text) {
    const ptr0 = passStringToWasm(document);
    const len0 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm(id_prefix);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm(id_postfix);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passStringToWasm(text);
    const len4 = WASM_VECTOR_LEN;
    try {
        return wasm.set_chapter_text(ptr0, len0, i, ptr2, len2, ptr3, len3, ptr4, len4);

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);
        wasm.__wbindgen_free(ptr2, len2 * 1);
        wasm.__wbindgen_free(ptr3, len3 * 1);
        wasm.__wbindgen_free(ptr4, len4 * 1);

    }

};

__exports.__widl_f_log_1_ = function(arg0) {
    console.log(getObject(arg0));
};

__exports.__wbindgen_string_new = function(p, l) { return addHeapObject(getStringFromWasm(p, l)); };

__exports.__wbindgen_json_parse = function(ptr, len) { return addHeapObject(JSON.parse(getStringFromWasm(ptr, len))); };

__exports.__wbindgen_json_serialize = function(idx, ptrptr) {
    const ptr = passStringToWasm(JSON.stringify(getObject(idx)));
    getUint32Memory()[ptrptr / 4] = ptr;
    return WASM_VECTOR_LEN;
};

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

__exports.__wbindgen_object_drop_ref = function(i) { dropObject(i); };

function init(module_or_path, maybe_memory) {
    let result;
    const imports = { './textglue_wasm': __exports };
    if (module_or_path instanceof URL || typeof module_or_path === 'string' || module_or_path instanceof Request) {

        const response = fetch(module_or_path);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module_or_path, imports)
        .then(instance => {
            return { instance, module: module_or_path };
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

self.wasm_bindgen = Object.assign(init, __exports);

})();
