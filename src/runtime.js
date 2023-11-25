
// in this runtime available available is:
// - setup(wasabiBinary) you need to call this to make the next function available
// - instrument_wasm() a function that takes a wasm buffer and instruments it with wasabi.

// function traceToStream(traceStream) {
//     function map(list) {
//         list.join(',')
//     }
//     return {
//         start(location) { },
//         nop(location) { },
//         unreachable(location) { },
//         if_(location, condition) { },
//         br(location, target) { },
//         br_if(location, conditionalTarget, condition) { },
//         br_table(location, table, defaultTarget, tableIdx) { },
//         begin(location, type) { },
//         end(location, type, beginLocation, ifLocation) { },
//         drop(location, value) { },
//         select(location, cond, first, second) { },
//         return_(location, values) { },
//         const_(location, op, value) { },
//         unary(location, op, input, result) { },
//         binary(location, op, first, second, result) { },
//         memory_size(location, memoryIdx, currentSizePages) { },
//         local(location, op, localIndex, value) { },
//         memory_fill(location, index, value, length) { },
//         memory_copy(location, destination, source, length) { },
//         memory_init(location, destination, source, length) { },
//         table_size(location, currentSizeEntries) { },
//         table_copy(location, destination, source, length) { },
//         table_init(location, destination, source, length) { },
//         table_grow(location, n, val, previusElement) { },
//         table_fill(location, index, value, length) { },

//         begin_function: (location, args) => {
//             traceStream.write(`C;${location.func};${map(args)}`)
//             // traceStream.write('C'funcIdx: location.func, args)
//         },

//         store: (location, op, target, memarg, value) => {
//             traceStream.write(`S;${target.memIdx};${target.addr + memarg.offset};${op};${value}`)
//             // this.trace.push(this.stringifyEvent({ type: 'S', memIdx: target.memIdx, addr: target.addr + memarg.offset, op, value }))
//         },

//         memory_grow: (location, memIdx, byPages, previousSizePages) => {
//             traceStream.write(`MG;${target.memIdx};${byPages}`)
//             // this.trace.push(this.stringifyEvent({ type: 'MG', memIdx, byPages }))
//         },

//         load: (location, op, target, memarg, value) => {
//             traceStream.write(`L;${target.memIdx};${target.addr + memarg.offset};${op};${value}`)
//             // this.trace.push(this.stringifyEvent({ type: 'L', memIdx: target.memIdx, addr: target.addr + memarg.offset, op, value }))
//         },

//         global: (location, op, idx, value) => {
//             if (op === 'global.set') {
//                 traceStream.write(`GS;${idx};${value}`)
//                 // this.trace.push(this.stringifyEvent({ type: 'GS', globalIdx: idx, value }))
//             } else {
//                 traceStream.write(`GG;${idx};${value}`)
//                 // this.trace.push(this.stringifyEvent({ type: 'GG', globalIdx: idx, value }))
//             }
//         },

//         call_pre: (location, op, funcIdx, args, tableTarget) => {
//             traceStream.write(`C;${funcIdx};${map(args)}`)
//             // this.trace.push(this.stringifyEvent({ type: 'C', funcIdx, args }))
//             if (op === 'call_indirect') {
//                 traceStream.write(`TG;${tableTarget.tableIdx};${tableTarget.elemIdx};${funcIdx}`)
//                 // this.trace.push(this.stringifyEvent({ type: 'TG', tableIdx: tableTarget.tableIdx, idx: tableTarget.elemIdx, funcIdx }))
//             }
//         },

//         call_post: (location, results) => {
//             traceStream.write(`R;${map(results)}`)
//             // this.trace.push(this.stringifyEvent({ type: 'R', funcIdx: 0, results }))
//         },

//         table_set: (location, target, value) => {
//             traceStream.write(`TS;${tableTarget.tableIdx};${tableTarget.elemIdx};${funcIdx};${value}`)
//             // this.trace.push(this.stringifyEvent({ type: 'TS', tableIdx: target.tableIdx, idx: target.elemIdx, funcIdx: value }))
//         },

//         table_get: (location, target, value) => {
//             traceStream.write(`TG;${tableTarget.tableIdx};${tableTarget.elemIdx};${funcIdx}`)
//             // this.trace.push(this.stringifyEvent({ type: 'TG', tableIdx: target.tableIdx, idx: target.elemIdx, funcIdx: value }))
//         }
//     }
// }
function traceToStream(traceStream) {
    function map(list) {
        list.join(',')
    }

    let trace = ''
    function makeRequest(event) {
        trace += event + '\n'
        if (trace.length >= 1000000) {
            fetch(`/r3-trace`, {
                method: 'POST',
                body: trace
            })
            trace = ''
        }
    }

    // const compressedStream = new CompressionStream('gzip')
    // const reader = compressedStream.readable.getReader();
    // const writer = compressedStream.writable.getWriter()
    // const encoder = new TextEncoder()


    // async function addToChunk() {
    //     let chunks = []
    //     chunks.push(await reader.read().value)
    //     if (chunks.length >= 1000) {
    //         fetch(`/r3-trace`, {
    //             method: 'POST',
    //             body: new Blob(chunks, { type: 'application/gzip' })
    //         })
    //         chunks = []
    //     }
    // }

    return {
        start(location) { },
        nop(location) { },
        unreachable(location) { },
        if_(location, condition) { },
        br(location, target) { },
        br_if(location, conditionalTarget, condition) { },
        br_table(location, table, defaultTarget, tableIdx) { },
        begin(location, type) { },
        end(location, type, beginLocation, ifLocation) { },
        drop(location, value) { },
        select(location, cond, first, second) { },
        return_(location, values) { },
        const_(location, op, value) { },
        unary(location, op, input, result) { },
        binary(location, op, first, second, result) { },
        memory_size(location, memoryIdx, currentSizePages) { },
        local(location, op, localIndex, value) { },
        memory_fill(location, index, value, length) { },
        memory_copy(location, destination, source, length) { },
        memory_init(location, destination, source, length) { },
        table_size(location, currentSizeEntries) { },
        table_copy(location, destination, source, length) { },
        table_init(location, destination, source, length) { },
        table_grow(location, n, val, previusElement) { },
        table_fill(location, index, value, length) { },

        begin_function: (location, args) => {
            makeRequest(`C;${location.func};${map(args)}`)
            // addToChunk()
            // traceStream.write('C'funcIdx: location.func, args)
        },

        store: (location, op, target, memarg, value) => {
            makeRequest(`S;${target.memIdx};${target.addr + memarg.offset};${op};${value}`)
            // this.trace.push(this.stringifyEvent({ type: 'S', memIdx: target.memIdx, addr: target.addr + memarg.offset, op, value }))
        },

        memory_grow: (location, memIdx, byPages, previousSizePages) => {
            makeRequest(`MG;${target.memIdx};${byPages}`)
            // this.trace.push(this.stringifyEvent({ type: 'MG', memIdx, byPages }))
        },

        load: (location, op, target, memarg, value) => {
            makeRequest(`L;${target.memIdx};${target.addr + memarg.offset};${op};${value}`)
            // this.trace.push(this.stringifyEvent({ type: 'L', memIdx: target.memIdx, addr: target.addr + memarg.offset, op, value }))
        },

        global: (location, op, idx, value) => {
            if (op === 'global.set') {
                makeRequest(`GS;${idx};${value}`)
                // this.trace.push(this.stringifyEvent({ type: 'GS', globalIdx: idx, value }))
            } else {
                makeRequest(`GG;${idx};${value}`)
                // this.trace.push(this.stringifyEvent({ type: 'GG', globalIdx: idx, value }))
            }
        },

        call_pre: (location, op, funcIdx, args, tableTarget) => {
            makeRequest(`C;${funcIdx};${map(args)}`)
            // this.trace.push(this.stringifyEvent({ type: 'C', funcIdx, args }))
            if (op === 'call_indirect') {
                makeRequest(`TG;${tableTarget.tableIdx};${tableTarget.elemIdx};${funcIdx}`)
                // this.trace.push(this.stringifyEvent({ type: 'TG', tableIdx: tableTarget.tableIdx, idx: tableTarget.elemIdx, funcIdx }))
            }
        },

        call_post: (location, results) => {
            makeRequest(`R;${map(results)}`)
            // this.trace.push(this.stringifyEvent({ type: 'R', funcIdx: 0, results }))
        },

        table_set: (location, target, value) => {
            makeRequest(`TS;${tableTarget.tableIdx};${tableTarget.elemIdx};${funcIdx};${value}`)
            // this.trace.push(this.stringifyEvent({ type: 'TS', tableIdx: target.tableIdx, idx: target.elemIdx, funcIdx: value }))
        },

        table_get: (location, target, value) => {
            makeRequest(`TG;${tableTarget.tableIdx};${tableTarget.elemIdx};${funcIdx}`)
            // this.trace.push(this.stringifyEvent({ type: 'TG', tableIdx: target.tableIdx, idx: target.elemIdx, funcIdx: value }))
        }
    }
}

function setup() {
    if (self.monkeypatched === true) {
        return
    }
    self.monkeypatched = true
    self.originalWasmBuffer = []
    self.readable = []
    self.writable = []
    wasabis = []
    let i = 0
    const printWelcome = function () {
        console.log('---------------------------------------------')
        console.log('          Wasabi analysis attached           ')
        console.log('---------------------------------------------')
        console.log('WebAssembly module instantiated.             ')
    }

    const importObjectWithHooks = function (importObject, i) {
        let importObjectWithHooks = importObject || {};
        importObjectWithHooks.__wasabi_hooks = wasabis[i].module.lowlevelHooks;
        return importObjectWithHooks;
    };

    const wireInstanceExports = function (instance, i) {
        wasabis[i].module.exports = instance.exports;
        wasabis[i].module.tables = [];
        wasabis[i].module.memories = [];
        wasabis[i].module.globals = [];
        for (let exp in instance.exports) {
            if (wasabis[i].module.info.tableExportNames.includes(exp)) {
                wasabis[i].module.tables.push(instance.exports[exp]);
            }
            if (wasabis[i].module.info.memoryExportNames.includes(exp)) {
                wasabis[i].module.memories.push(instance.exports[exp])
            }
            if (wasabis[i].module.info.globalExportNames.includes(exp)) {
                wasabis[i].module.globals.push(instance.exports[exp]);
            }
        }
    };

    const binaryString = atob(wasabiBinary);
    const uint8Array = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
        uint8Array[i] = binaryString.charCodeAt(i);
    }
    const buffer = uint8Array.buffer;

    initSync(buffer)
    let original_instantiate = WebAssembly.instantiate
    WebAssembly.instantiate = function (buffer, importObject) {
        const this_i = i
        i += 1
        console.log('WebAssembly.instantiate')
        printWelcome()
        self.originalWasmBuffer.push(Array.from(new Uint8Array(buffer)))
        const { instrumented, js } = instrument_wasm({ original: new Uint8Array(buffer) });
        wasabis.push(eval(js + '\nWasabi'))
        buffer = new Uint8Array(instrumented)
        importObject = importObjectWithHooks(importObject, this_i)
        const { readable, writable } = new TransformStream()
        wasabis[0].analysis = traceToStream(writable.getWriter())
        // fetch('/r3-trace', {
        //     method: 'POST',
        //     body: readable,
        //     duplex: 'half'
        // })
        let result
        result = original_instantiate(buffer, importObject)
        result.then(({ module, instance }) => {
            wireInstanceExports(instance, this_i)
        })
        self.wasabis = wasabis
        return result
    };
    // replace instantiateStreaming
    WebAssembly.instantiateStreaming = async function (source, obj) {
        console.log('WebAssembly.instantiateStreaming')
        let response = await source;
        let body = await response.arrayBuffer();
        return WebAssembly.instantiate(body, obj);
    }
}
setup()