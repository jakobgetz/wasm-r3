import fs from 'fs'
import path from 'path'
let instance
let imports = {}
imports['env'] = {}
imports['asm2wasm'] = {}
const memory = new WebAssembly.Memory({ initial: 256, maximum: 256 })
imports['env']['memory'] = memory
const DYNAMICTOP_PTR = new WebAssembly.Global({ value: 'i32', mutable: false}, 0)
imports['env']['DYNAMICTOP_PTR'] = DYNAMICTOP_PTR
const STACKTOP = new WebAssembly.Global({ value: 'i32', mutable: false}, 0)
imports['env']['STACKTOP'] = STACKTOP
const STACK_MAX = new WebAssembly.Global({ value: 'i32', mutable: false}, 0)
imports['env']['STACK_MAX'] = STACK_MAX
const memoryBase = new WebAssembly.Global({ value: 'i32', mutable: false}, 0)
imports['env']['memoryBase'] = memoryBase
const tableBase = new WebAssembly.Global({ value: 'i32', mutable: false}, 0)
imports['env']['tableBase'] = tableBase
const table = new WebAssembly.Table({ initial: 34, maximum: 34, element: 'anyfunc'})
imports['env']['table'] = table
let global_0 = 0
imports['env']['abort'] = () => {
global_0++
}
let global_1 = 0
imports['env']['enlargeMemory'] = () => {
global_1++
}
let global_2 = 0
imports['env']['getTotalMemory'] = () => {
global_2++
}
let global_3 = 0
imports['env']['abortOnCannotGrowMemory'] = () => {
global_3++
}
let global_4 = 0
imports['env']['_pthread_getspecific'] = () => {
global_4++
}
let global_5 = 0
imports['env']['___syscall54'] = () => {
global_5++
}
let global_6 = 0
imports['env']['_pthread_setspecific'] = () => {
global_6++
}
let global_7 = 0
imports['env']['___lock'] = () => {
global_7++
}
let global_8 = 0
imports['env']['_abort'] = () => {
global_8++
}
let global_9 = 0
imports['env']['___setErrNo'] = () => {
global_9++
}
let global_10 = 0
imports['env']['___syscall6'] = () => {
global_10++
}
let global_11 = 0
imports['env']['___syscall140'] = () => {
global_11++
}
let global_12 = 0
imports['env']['_pthread_once'] = () => {
global_12++
}
let global_13 = 0
imports['env']['_emscripten_memcpy_big'] = () => {
global_13++
}
let global_14 = 0
imports['env']['_pthread_key_create'] = () => {
global_14++
}
let global_15 = 0
imports['env']['___unlock'] = () => {
global_15++
}
let global_16 = 0
imports['env']['___assert_fail'] = () => {
global_16++
}
let global_17 = 0
imports['env']['___syscall146'] = () => {
global_17++
}
let global_18 = 0
imports['asm2wasm']['f64-to-int'] = () => {
global_18++
}
export function replay(wasm) {instance = wasm.instance
instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

instance.exports._fib(40)

}
export function instantiate(wasmBinary) {
return WebAssembly.instantiate(wasmBinary, imports)
}
if (process.argv[2] === 'run') {
const p = path.join(path.dirname(import.meta.url).replace(/^file:/, ''), 'index.wasm')
const wasmBinary = fs.readFileSync(p)
instantiate(wasmBinary).then((wasm) => replay(wasm))
}
