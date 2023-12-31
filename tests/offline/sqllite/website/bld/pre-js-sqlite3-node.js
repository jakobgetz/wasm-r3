/**
   BEGIN FILE: api/pre-js.js

   This file is intended to be prepended to the sqlite3.js build using
   Emscripten's --pre-js=THIS_FILE flag (or equivalent).
*/

// See notes in extern-post-js.js
const sqlite3InitModuleState = globalThis.sqlite3InitModuleState
      || Object.assign(Object.create(null),{
        debugModule: ()=>{}
      });
delete globalThis.sqlite3InitModuleState;
sqlite3InitModuleState.debugModule('globalThis.location =',globalThis.location);


/**
   Bug warning: a custom Module.instantiateWasm() does not work
   in WASMFS builds:

   https://github.com/emscripten-core/emscripten/issues/17951

   In such builds we must disable this.
*/
const xNameOfInstantiateWasm = false
      ? 'instantiateWasm'
      : 'emscripten-bug-17951';
Module[xNameOfInstantiateWasm] = function callee(imports,onSuccess){
  imports.env.foo = function(){};
  const uri = Module.locateFile(
    callee.uri, (
      ('undefined'===typeof scriptDirectory/*var defined by Emscripten glue*/)
        ? "" : scriptDirectory)
  );
  sqlite3InitModuleState.debugModule(
    "instantiateWasm() uri =", uri
  );
  const wfetch = ()=>fetch(uri, {credentials: 'same-origin'});
  const loadWasm = WebAssembly.instantiateStreaming
        ? async ()=>{
          return WebAssembly.instantiateStreaming(wfetch(), imports)
            .then((arg)=>onSuccess(arg.instance, arg.module));
        }
        : async ()=>{ // Safari < v15
          return wfetch()
            .then(response => response.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports))
            .then((arg)=>onSuccess(arg.instance, arg.module));
        };
  loadWasm();
  return {};
};
/*
  It is literally impossible to reliably get the name of _this_ script
  at runtime, so impossible to derive X.wasm from script name
  X.js. Thus we need, at build-time, to redefine
  Module[xNameOfInstantiateWasm].uri by appending it to a build-specific
  copy of this file with the name of the wasm file. This is apparently
  why Emscripten hard-codes the name of the wasm file into their glue
  scripts.
*/
Module[xNameOfInstantiateWasm].uri = 'sqlite3.wasm';
/* END FILE: api/pre-js.js, noting that the build process may add a
   line after this one to change the above .uri to a build-specific
   one. */
