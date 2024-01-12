import { Browser, chromium, Frame, Page, Worker } from 'playwright'
import { createMeasure, StopMeasure } from './performance.cjs'
import fs from 'fs/promises'
import fss from 'fs'
import { Trace } from './tracer.cjs'
import acorn from 'acorn'
import { trimFromLastOccurance } from '../tests/test-utils.cjs'
import { WebSocketServer } from 'ws';
import path from 'path'
import { execSync } from 'child_process'

export interface AnalysisI<T> {
    getResult(): T,
}

export interface AnalyserI {
    start: (url: string, options: { headless: boolean }) => Promise<Page>
    stop: () => Promise<AnalysisResult>
}

export type AnalysisResult = {
    result: string,
    wasm: number[]
}[]

type Options = { extended?: boolean, noRecord?: boolean }
export class Analyser implements AnalyserI {

    private analysisPath: string
    private options: Options
    private browser: Browser
    private page: Page
    private contexts: (Frame | Worker)[] = []
    private isRunning = false
    private p_measureUserInteraction: StopMeasure


    constructor(analysisPath: string, options: Options = { extended: false, noRecord: false }) {
        this.analysisPath = analysisPath
        this.options = options
    }

    async start(url, { headless } = { headless: false }) {
        if (this.isRunning === true) {
            throw new Error('Analyser is already running. Stop the Analyser before starting again')
        }
        const p_measureStart = createMeasure('start', { phase: 'record', description: `The time it takes start the chromium browser and open the webpage until the 'load' event is fired.` })
        this.isRunning = true
        this.browser = await chromium.launch({ // chromium version: 119.0.6045.9 (Developer Build) (x86_64); V8 version: V8 11.9.169.3; currently in node I run version 11.8.172.13-node.12
            headless, args: [
                // '--disable-web-security',
                '--js-flags="--max_old_space_size=8192"',
                '--enable-experimental-web-platform-features'
            ]
        });
        this.page = await this.browser.newPage();
        this.page.setDefaultTimeout(120000);
        if (this.options.noRecord !== true) {
            await this.attachRecorder()
        }

        await this.page.goto(url, { timeout: 60000 })
        p_measureStart()
        this.p_measureUserInteraction = createMeasure('user interaction', { phase: 'record', description: `The time the user interacts with the webpage during recording, from the time the 'loal' event got fired in the browser until the user stopps the recording.` })
        return this.page
    }

    async stop() {
        if (this.isRunning === false) {
            throw new Error('Analyser is not running. Start the Analyser before stopping')
        }
        this.p_measureUserInteraction()
        const p_measureStop = createMeasure('stop', { phase: 'record', description: `The time it takes to stop the recording, from when the user stopped the recording until all data is downloaded from the browser and the browser is closed.` })
        this.contexts = this.contexts.concat(this.page.frames())
        const p_measureDataDownload = createMeasure('data download', { phase: 'record', description: `The time it takes to download all data from the browser.` })
        const p_measureTraceDownload = createMeasure('trace download', { phase: 'record', description: `The time it takes to download all traces from the browser.` })
        const traces = (await this.getResults()).map(t => trimFromLastOccurance(t, 'ER'))
        p_measureTraceDownload()
        const p_measureBufferDownload = createMeasure('buffer download', { phase: 'record', description: `The time it takes to download all wasm binaries from the browser.` })
        const originalWasmBuffer = await this.getBuffers()
        p_measureBufferDownload()
        p_measureDataDownload()
        this.contexts = []
        this.browser.close()
        this.isRunning = false
        p_measureStop()
        return traces.map((result, i) => ({ result: result, wasm: originalWasmBuffer[i] }))
    }

    private async attachRecorder() {
        const initScript = await this.constructInitScript()

        await this.page.addInitScript({ content: initScript })

        await this.page.route('**/*', async (route) => {
            const response = await route.fetch()
            const headers = response.headers()

            // Remove or modify the CSP header
            delete headers['content-security-policy'];
            delete headers['content-security-policy-report-only']

            await route.fulfill({
                status: response.status(),
                headers: headers,
                body: await response.body()
            });
        })

        await this.page.route(`**/*.js*`, async route => {
            let response
            try {
                response = await route.fetch()
            } catch {
                // this usually only happens when the browser is already closed. Only happens when running the test suite
                return
            }
            const script = await response.text()
            try {
                acorn.parse(script, { ecmaVersion: 'latest' })
                const body = `${initScript}${script}`
                await route.fulfill({ response, body: body })
            } catch {
                route.fulfill({ response, body: script })
            }
        })
        this.page.on('worker', worker => {
            this.contexts.push(worker)
        })
    }

    private async getResults() {
        const results = await Promise.all(this.contexts.map(async (c) => {
            const p_measureTraceDownload = createMeasure(`trace download from context: ${c.url()}`, { phase: 'record', description: `The time it takes to download the trace from the browser context: ${c.url}.` })
            const traces = await c.evaluate(() => {
                try {
                    //@ts-ignore
                    const traces = (analysis as AnalysisI<Trace>[]).map((r, j) => {
                        const trace = r.getResult().toString()
                        return trace
                    })
                    return traces
                } catch {
                    return []
                }
            })
            p_measureTraceDownload()
            return traces.map(t => t.toString())
        }))
        return results.flat(1)
    }


    private async getBuffers() {
        const originalWasmBuffer = await Promise.all(this.contexts.map(async (c) => {
            const p_measureBufferDownload = createMeasure(`buffer download from context: ${c.url()}`, { phase: 'record', description: `The time it takes to download the wasm binary from the browser context: ${c.url}.` })
            const buffer = await c.evaluate(() => {
                try {
                    return Array.from(originalWasmBuffer)
                } catch {
                    return []
                }
            })
            p_measureBufferDownload()
            return buffer
        }))
        return originalWasmBuffer.flat(1) as number[][]
    }

    private async constructInitScript() {
        const wasabiScript = await fs.readFile('./dist/wasabi.js') + '\n'
        let analysisScript = await fs.readFile(this.analysisPath) + '\n'
        analysisScript = analysisScript.split('Object.defineProperty(exports, "__esModule", { value: true });').join('')
        analysisScript = analysisScript.split('exports.default = setupAnalysis;').join('')
        analysisScript = analysisScript.split('exports.stringifyTrace = void 0;').join('')
        analysisScript = analysisScript.split('exports.stringifyTrace = stringifyTrace;').join('')
        analysisScript = analysisScript.split('exports.Trace = void 0;').join('')
        analysisScript = analysisScript.split('exports.Trace = Trace;').join('')
        analysisScript = analysisScript.split('exports.default = Analysis;').join('')
        analysisScript = `function setupAnalysis(Wasabi) {\n ${analysisScript};\n return new Analysis(Wasabi, { extended: ${this.options.extended}})}\n`
        const setupScript = await fs.readFile('./src/wasabi-runtime.js') + '\n'
        return wasabiScript + ';' + analysisScript + ';' + setupScript + ';'
    }

    setExtended(extended: boolean) {
        this.options.extended = extended
    }

    getExtended() {
        return this.options.extended
    }
}

export class CustomAnalyser implements AnalyserI {
    private browser: Browser
    private page: Page
    private contexts: (Frame | Worker)[] = []
    private isRunning = false
    private p_measureUserInteraction: StopMeasure
    private benchmarkPath: string
    private traceFilePath: string
    private javascript: boolean;


    constructor(traceFilePath: string, options: { benchmarkPath: string, javascript: boolean }) {
        this.benchmarkPath = options.benchmarkPath;
        this.traceFilePath = traceFilePath;
        this.javascript = options.javascript;

        const wss = new WebSocketServer({ port: 8080 });
        function setupConnection(filePath: string) {
            wss.on('connection', function connection(ws) {
                console.log(`connection opened writing to ${filePath}`)
                const writeStream = fss.createWriteStream(filePath)
                ws.on('error', console.error)
                ws.on('message', function message(data) {
                    writeStream.write(data)
                })
            })
        }
        setupConnection(traceFilePath)
    }

    async start(url: string, { headless } = { headless: false }) {
        if (this.isRunning === true) {
            throw new Error('Analyser is already running. Stop the Analyser before starting again')
        }
        const p_measureStart = createMeasure('start', { phase: 'record', description: `The time it takes start the chromium browser and open the webpage until the 'load' event is fired.` })
        this.isRunning = true
        this.browser = await chromium.connectOverCDP('http://localhost:9222');
        this.page = await this.browser.newPage();
        this.page.setDefaultTimeout(120000);
        await this.attachRecorder()

        await this.page.goto(url, { timeout: 60000 })
        p_measureStart()
        this.p_measureUserInteraction = createMeasure('user interaction', { phase: 'record', description: `The time the user interacts with the webpage during recording, from the time the 'loal' event got fired in the browser until the user stopps the recording.` })
        return this.page
    }

    async stop(): Promise<AnalysisResult> {
        if (this.isRunning === false) {
            throw new Error('Analyser is not running. Start the Analyser before stopping')
        }
        this.p_measureUserInteraction()
        const p_measureStop = createMeasure('stop', { phase: 'record', description: `The time it takes to stop the recording, from when the user stopped the recording until all data is downloaded from the browser and the browser is closed.` })
        this.contexts = this.contexts.concat(this.page.frames())
        const p_measureDataDownload = createMeasure('data download', { phase: 'record', description: `The time it takes to download all data from the browser.` })
        await this.traceToServer()
        const p_measureBufferDownload = createMeasure('buffer download', { phase: 'record', description: `The time it takes to download all wasm binaries from the browser.` })
        const originalWasmBuffer = await this.getBuffers()
        p_measureBufferDownload()
        p_measureDataDownload()
        this.contexts = []
        this.browser.close()
        this.isRunning = false
        p_measureStop()
        const binName = 'index.wasm'
        const traceName = 'trace.bin'
        const replayName = 'replay.js'
        fss.mkdirSync(this.benchmarkPath);
        fss.writeFileSync(path.join(this.benchmarkPath, binName), new Int8Array(originalWasmBuffer[0]))
        const p_measureCodeGen = createMeasure('rust-backend', { phase: 'replay-generation', description: `The time it takes for rust backend to generate javascript` })
        execSync(`./target/debug/replay_gen generate ${this.traceFilePath} ${path.join(this.benchmarkPath, binName)} true ${path.join(this.benchmarkPath, replayName)}`);
        // execSync(`wasm-validate ${path.join(this.benchmarkPath, replayName)}`)
        p_measureCodeGen()
        return originalWasmBuffer.map((wasm) => ({ result: '', wasm }))
    }

    private async attachRecorder() {
        const initScript = await this.constructInitScript()

        await this.page.addInitScript({ content: initScript })

        await this.page.route('**/*', async (route) => {
            const response = await route.fetch()
            const headers = response.headers()

            // Remove or modify the CSP header
            delete headers['content-security-policy'];
            delete headers['content-security-policy-report-only']

            await route.fulfill({
                status: response.status(),
                headers: headers,
                body: await response.body()
            });
        })

        await this.page.route(`**/*.js*`, async route => {
            let response
            try {
                response = await route.fetch()
            } catch {
                // this usually only happens when the browser is already closed. Only happens when running the test suite
                return
            }
            const script = await response.text()
            try {
                acorn.parse(script, { ecmaVersion: 'latest' })
                const body = `${initScript}${script}`
                await route.fulfill({ response, body: body })
            } catch {
                route.fulfill({ response, body: script })
            }
        })
        this.page.on('worker', worker => {
            this.contexts.push(worker)
        })
    }

    private async traceToServer() {
        const results = await Promise.all(this.contexts.map(async (c) => {
            await c.evaluate(() => {
                try {
                    //@ts-ignore
                    r3_check_mem()
                } catch {
                    // ok
                }
            })
        }))
    }


    private async getBuffers() {
        const originalWasmBuffer = await Promise.all(this.contexts.map(async (c) => {
            const p_measureBufferDownload = createMeasure(`buffer download from context: ${c.url()}`, { phase: 'record', description: `The time it takes to download the wasm binary from the browser context: ${c.url}.` })
            const buffer = await c.evaluate(() => {
                try {
                    return Array.from(originalWasmBuffer)
                } catch {
                    return []
                }
            })
            p_measureBufferDownload()
            return buffer
        }))
        return originalWasmBuffer.flat(1) as number[][]
    }

    private async constructInitScript() {
        const tracerScript = await fs.readFile('./dist/tracer.js') + '\n'
        const setupScript = await fs.readFile('./src/tracer-runtime.js') + '\n'
        return tracerScript + ';' + setupScript + ';'
    }
}