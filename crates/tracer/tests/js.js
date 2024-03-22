import fs from "fs/promises"

async function run() {
    let wasm = await WebAssembly.instantiate(await fs.readFile("data.wasm"));
    let x = wasm.instance.exports.main()
    console.log(x);
}
run()