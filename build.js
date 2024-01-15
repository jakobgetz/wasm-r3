import fs from 'fs'
import path from 'path'

const dirname = path.dirname(import.meta.url.replace(/^file:/, ''))

// Create the wasabi runtime
const runtimeWS = fs.createWriteStream(path.join(dirname, './dist/wasabi.js'))
runtimeWS.write(`var wasabiBinary = '`)
runtimeWS.write(fs.readFileSync(path.join(dirname, 'wasabi/crates/wasabi_js/pkg/wasabi_js_bg.wasm')).toString('base64'))
runtimeWS.write(`'\n`)
let wasabiJsApi = fs.readFileSync(path.join(dirname, 'wasabi/crates/wasabi_js/pkg/wasabi_js.js'), 'utf-8')
wasabiJsApi = wasabiJsApi.replace(/export function/g, 'function')
wasabiJsApi = wasabiJsApi.replace(/let /g, 'var ')
wasabiJsApi = wasabiJsApi.replace(/const /g, 'var ')
wasabiJsApi = wasabiJsApi.replace(/export { initSync }/g, '')
wasabiJsApi = wasabiJsApi.replace(/export default __wbg_init;/g, '')
wasabiJsApi = wasabiJsApi.replace(/input = new URL\('wasabi_js_bg\.wasm', import\.meta\.url\);/g, '')
runtimeWS.write(wasabiJsApi)
runtimeWS.write('\n')
runtimeWS.close()

const runtimeT = fs.createWriteStream(path.join(dirname, './dist/tracer.js'))
runtimeT.write(`var tracerBinary = '`)
runtimeT.write(fs.readFileSync(path.join(dirname, 'crates/tracer/pkg/tracer_bg.wasm')).toString('base64'))
runtimeT.write(`'\n`)
let tracerJsApi = fs.readFileSync(path.join(dirname, 'crates/tracer/pkg/tracer.js'), 'utf-8')
tracerJsApi = tracerJsApi.replace(/export function/g, 'function')
tracerJsApi = tracerJsApi.replace(/let /g, 'var ')
tracerJsApi = tracerJsApi.replace(/const /g, 'var ')
tracerJsApi = tracerJsApi.replace(/export { initSync }/g, '')
tracerJsApi = tracerJsApi.replace(/export default __wbg_init;/g, '')
tracerJsApi = tracerJsApi.replace(/input = new URL\('tracer_bg\.wasm', import\.meta\.url\);/g, '')
runtimeT.write(tracerJsApi)
runtimeT.write('\n')
runtimeT.close()

const runtimeNode = fs.createWriteStream(path.join(dirname, './dist/node-runtime.js'))
runtimeNode.write(`var tracerBinary = '`)
runtimeNode.write(fs.readFileSync(path.join(dirname, 'crates/tracer/pkg/tracer_bg.wasm')).toString('base64'))
runtimeNode.write(`'\n`)
runtimeNode.write(tracerJsApi)
runtimeNode.write('\n')
runtimeNode.write(fs.readFileSync('./src/node-runtime.js', 'utf-8'))
runtimeNode.close()

// fs.mkdirSync('dist/tracer');
fs.copyFileSync('crates/tracer/pkg/tracer.js', 'dist/tracer/tracer.js')
fs.copyFileSync('crates/tracer/pkg/tracer_bg.wasm', 'dist/tracer/tracer_bg.wasm')