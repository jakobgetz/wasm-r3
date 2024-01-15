import fs from 'fs'
let x = fs.readFileSync('test.wasm')
WebAssembly.instantiate(x).then(() => console.log('done'))