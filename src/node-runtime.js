
// in this runtime available available is:
// - setup(wasabiBinary) you need to call this to make the next function available
// - instrument_wasm() a function that takes a wasm buffer and instruments it with wasabi.
// - the setupAnalysis function returns a new Analysis instance when given the Wasabi object
export function setup(filePath) {
    let i = 0
    // const p_timeToFirstInstantiate = performanceEvent('time until instantiation of first wasm module in this context')
    const printWelcome = function () {
        // console.log('---------------------------------------------')
        // console.log('             Instrumenting Wasm              ')
        // console.log('---------------------------------------------')
        // console.log('WebAssembly module instantiated.             ')
    }
    let instance
    const r3 = {
        check_mem: () => {
            const trace = instance.exports.trace.buffer.slice(0, instance.exports.trace_byte_length.value)
            fs.writeFileSync(filePath, Buffer.from(trace))
        }
    }

    const binaryString = atob(tracerBinary);
    const uint8Array = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
        uint8Array[i] = binaryString.charCodeAt(i);
    }
    const buffer = uint8Array.buffer;
    initSync(buffer)
    let original_instantiate = WebAssembly.instantiate
    WebAssembly.instantiate = async function (buffer, importObject) {
        importObject.r3 = r3
        buffer = (buffer.byte) ? buffer.byte : buffer
        const this_i = i
        i += 1
        printWelcome()
        const instrumented = instrument_wasm_js(new Uint8Array(buffer));
        buffer = new Uint8Array(instrumented)
        let result = await original_instantiate(buffer, importObject)
        WebAssembly.instantiate = original_instantiate
        instance = result.instance
        return result
    };


    return r3.check_mem

}