import fs from 'fs';

async function run() {
    let wasm = fs.readFileSync('test.wasm')
    const { module, instance } = await WebAssembly.instantiate(wasm, {});
    new Uint8Array(instance.exports.memory.buffer)[4] = 1;
    let result = instance.exports.entry();
    console.log("result", result)
}
run()