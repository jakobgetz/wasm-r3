import fs from 'fs'
import path from 'path'
import cp from 'child_process'

const dirname = path.dirname(import.meta.url.replace(/^file:/, ''))

// Create the r3 browser runtime
const runtimeWS = fs.createWriteStream(path.join(dirname, './dist/wasabi.js'))
runtimeWS.write(`var wasabiBinary = '`)
runtimeWS.write(fs.readFileSync(path.join(dirname, 'r3-tracer/pkg/r3_tracer_bg.wasm')).toString('base64'))
runtimeWS.write(`'\n`)
let wasabiJsApi = fs.readFileSync(path.join(dirname, 'r3-tracer/pkg/r3_tracer.js'), 'utf-8')
wasabiJsApi = wasabiJsApi.replace(/export function/g, 'function')
wasabiJsApi = wasabiJsApi.replace(/let /g, 'var ')
wasabiJsApi = wasabiJsApi.replace(/const /g, 'var ')
wasabiJsApi = wasabiJsApi.replace(/export { initSync }/g, '')
wasabiJsApi = wasabiJsApi.replace(/export default __wbg_init;/g, '')
wasabiJsApi = wasabiJsApi.replace(/input = new URL\('r3_tracer_bg\.wasm', import\.meta\.url\);/g, '')
runtimeWS.write(wasabiJsApi)
runtimeWS.write('\n')
runtimeWS.close()