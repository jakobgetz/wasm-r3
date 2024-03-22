export default async function test(wasmBinary) {
  let instance;
  let mem = new WebAssembly.Memory({ initial: 1 });
  let imports = {
    env: {
      foo: () => {
        new Uint8Array(mem.buffer)[0] = 1;
        instance.exports.reentry()
      },
      bar: () => {
        new Uint8Array(mem.buffer)[1] = 1;
      },
      baz: () => {},
      mem,
    },
  };
  let wasm = await WebAssembly.instantiate(wasmBinary, imports);
  instance = wasm.instance;
  instance.exports.entry();
}
