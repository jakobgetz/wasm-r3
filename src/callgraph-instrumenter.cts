//@ts-nocheck
import { Browser, Page, chromium, Worker } from 'playwright'
import readline from 'readline'
import { Trace } from '../trace.cjs'
import { Wasabi } from '../wasabi.cjs';
import { Record } from './benchmark.cjs';
import fs from 'fs'
//@ts-ignore
import { instrument_wasm } from '../wasabi/wasabi_js.js'
import setupAnalysis, { CallGraph, ToFromHost } from './callgraph.cjs';

// Extend the global self object
declare global {
    interface Window {
        callGraphs: CallGraph[],
        originalWasmBuffer: number[][],
        wasabis: Wasabi[],
        wasabiBinary: any,
        initSync: Function,
        WebAssembly: any,
        instrument_wasm: Function,
        i: number,
        monkeypatched: boolean
    }
}

function setup(callGraphConstructor: string) {
    console.log('setup')
    console.time('setup')
    const initSync = self.initSync
    const WebAssembly = self.WebAssembly
    const instrument_wasm = self.instrument_wasm
    if (self.callGraphs === undefined) {
        self.callGraphs = []
    }
    if (self.wasabis === undefined) {
        self.wasabis = []
    }
    if (self.i === undefined) {
        self.i = 0
    }
    if (self.originalWasmBuffer === undefined) {
        self.originalWasmBuffer = []
    }
    if (self.monkeypatched === true) {
        return
    }
    self.monkeypatched = true
    const printWelcome = function () {
        console.log('---------------------------------------------')
        console.log('                   Wasm-R3                   ')
        console.log('---------------------------------------------')
        console.log('WebAssembly module instantiated. Recording...')
    }

    const importObjectWithHooks = function (importObject, i) {
        let importObjectWithHooks = importObject || {};
        importObjectWithHooks.__wasabi_hooks = self.wasabis[i].module.lowlevelHooks;
        return importObjectWithHooks;
    };

    const wireInstanceExports = function (instance, i) {
        self.wasabis[i].module.exports = instance.exports;
        self.wasabis[i].module.tables = [];
        self.wasabis[i].module.memories = [];
        self.wasabis[i].module.globals = [];
        // console.log(self.wasabis)
        for (let exp in instance.exports) {
            if (self.wasabis[i].module.info.tableExportNames.includes(exp)) {
                self.wasabis[i].module.tables.push(instance.exports[exp]);
            }
            if (self.wasabis[i].module.info.memoryExportNames.includes(exp)) {
                // console.log('I am here and I add to the memory', exp)
                self.wasabis[i].module.memories.push(instance.exports[exp])
                // console.log(i)
                // console.log(instance.exports[exp])
                // console.log(self.wasabis[i].module.memories)
                // console.log(self.wasabis)
                // console.log(self.wasabis[i].module.memories)
            }
            if (self.wasabis[i].module.info.globalExportNames.includes(exp)) {
                self.wasabis[i].module.globals.push(instance.exports[exp]);
            }
        }
    };

    //@ts-ignore
    const binaryString = atob(wasabiBinary);
    const uint8Array = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
        uint8Array[i] = binaryString.charCodeAt(i);
    }
    const buffer = uint8Array.buffer;

    initSync(buffer)
    let original_instantiate = WebAssembly.instantiate
    WebAssembly.instantiate = function (buffer: ArrayBuffer, importObject: Object) {
        const i = self.i
        self.i += 1
        console.log('WebAssembly.instantiate')
        printWelcome()
        self.originalWasmBuffer.push(Array.from(new Uint8Array(buffer)))
        console.log('try to instrument')
        console.time('instrument')
        const { instrumented, js } = instrument_wasm({ original: new Uint8Array(buffer) });
        console.timeEnd('instrument')
        self.wasabis.push(eval(js + '\nWasabi'))
        buffer = new Uint8Array(instrumented)
        importObject = importObjectWithHooks(importObject, i)
        self.callGraphs.push(eval(`(${callGraphConstructor})(self.wasabis[i])`))
        let result
        result = original_instantiate(buffer, importObject)
        result.then(({ module, instance }) => {
            wireInstanceExports(instance, i)
        })
        return result
    };
    // replace instantiateStreaming
    WebAssembly.instantiateStreaming = async function (source, obj) {
        console.log('WebAssembly.instantiateStreaming')
        let response = await source;
        let body = await response.arrayBuffer();
        return WebAssembly.instantiate(body, obj);
    }
    console.timeEnd('setup')
}

let workerHandles: Worker[] = []

export async function launch(url: string, { headless } = { headless: false }) {
    const browser = await chromium.launch({ headless, args: ['--disable-web-security'] });
    const page = await browser.newPage();

    await page.addInitScript({ path: './dist/wasabi.js' })
    await page.addInitScript(setup, setupAnalysis.toString())

    await page.route(`**/*.js*`, async route => {
        const response = await route.fetch()
        const script = await response.text()
        const wasabi = fs.readFileSync('./dist/wasabi.js')
        const body = `${wasabi};(${setup.toString()})(\`${setupAnalysis.toString()}\`);${script}`
        await route.fulfill({ response, body: body })
    })
    page.on('worker', worker => {
        workerHandles.push(worker)
    })

    await page.goto(url)
    console.time('user interaction')
    return { browser, page }
}

export async function land(browser: Browser, page: Page): Promise<Record> {
    console.timeEnd('user interaction')
    console.log('landing...')
    console.time('trace download')
    let callGraphs: CallGraph[]
    const callGraphStrings: string[] = (await page.evaluate(() => {
        function stringifyCallGraph(callGraph: CallGraph) {
            let s = 'How to read this graph: $from -> [$params] -> $to [$memSizes] [$tableSizes]\n'
            callGraph.forEach((edge, i) => {
                s += `${i}: ${edge.from.funcidx} -> [${edge.params}] -> ${edge.to.funcidx}\t`
                if (edge.from.host) {
                    s += `[${(edge.to as ToFromHost).memSizes}] [${(edge.to as ToFromHost).tableSizes}]`
                }
                s += `\n`
            })
            return s
        }
        return callGraphs.map((g) => stringifyCallGraph(g as CallGraph))
    }))
    fs.writeFileSync('callgraph.txt', callGraphStrings[0])

    // console.timeEnd('parse')
    // console.timeEnd('trace download')
    // const workerTracesStrings = await Promise.all(workerHandles.map((w) => w.evaluate(() => JSON.stringify(callGraphStrings))))
    // const workerTraces = workerTracesStrings.map(t => JSON.parse(t))
    // if (workerTraces.length !== 0) {
    //     callGraphStrings.push(...workerTraces.flat(1))
    // }
    // console.time('trace processing')
    // callGraphStrings.forEach(trace => trace.forEach(event => event.type === 'Load' && Array.from(event.data)))
    // console.timeEnd('trace processing')
    // console.time('wasm download')
    // const originalWasmBuffer: number[][] = await page.evaluate(() => {
    //     try { originalWasmBuffer } catch {
    //         throw new Error('There is no WebAssembly instantiated on that page. Make sure this page actually uses WebAssembly and that you also invoked it through your interaction.')
    //     }
    //     return originalWasmBuffer.map(b => {
    //         return Array.from(b)
    //     })
    // });
    // let workerBuffers = await Promise.all(workerHandles.map(w => w.evaluate(() => originalWasmBuffer)))
    // if (workerBuffers.length !== 0) {
    //     originalWasmBuffer.push(...workerBuffers.flat(1))
    // }
    // console.timeEnd('wasm download')
    // workerHandles = []
    // console.time('browser close')
    browser.close()
    // console.timeEnd('browser close')
    // @ts-ignore
    return callGraphStrings.map((trace, i) => ({ binary: originalWasmBuffer[i], trace }))
}

export default async function record(url: string, options = { headless: false }) {
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    async function askQuestion(question: string) {
        return new Promise((resolve) => {
            rl.question(question, (answer) => {
                resolve(answer);
            });
        });
    }
    const { browser, page } = await launch(url, options)
    console.log(`Record is running. Enter 'Stop' to stop recording.`)
    await askQuestion('')
    rl.close()
    console.log(`Record stopped`)
    return await land(browser, page)
}