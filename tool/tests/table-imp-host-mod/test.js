import fs from 'fs'
import path from 'path'
const wasmPath = path.join(path.dirname(import.meta.url).replace(/^file:/, ''), 'index.wasm')
const wasmBinary = fs.readFileSync(wasmPath)

let instance
const table = new WebAssembly.Table({ initial: 2, maximum: 2, element: 'anyfunc' })
let imports = {
    env: {
        changeTable: () => (
            table.set(0, instance.exports.foo)
        ),
        a: () => 1,
        b: () => 2,
        table,
    }
}
let wasm = await WebAssembly.instantiate(wasmBinary, imports)
instance = wasm.instance
instance.exports.main()