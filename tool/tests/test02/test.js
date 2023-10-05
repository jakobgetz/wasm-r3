import fs from 'fs'
import path from 'path'
const wasmBinary = fs.readFileSync(path.join(import.meta.dir, 'index.wasm'))

let instance
let imports = {
    env: {
        changeMem: (address, value) => (
            new Uint8Array(instance.exports.memory.buffer)[address] = value
        )
    }
}
let wasm = await WebAssembly.instantiate(wasmBinary, imports)
instance = wasm.instance
instance.exports.main()