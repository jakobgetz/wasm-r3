import { Browser, chromium, Frame, Page, Worker } from 'playwright'
import fs from 'fs/promises'
import acorn from 'acorn'
import http from 'http'

export interface AnalysisI<T> {
    getResult(): T,
    getResultChunk(size: number): T
}

export type AnalysisResult = {
    result: string,
    wasm: number[]
}[]

export default class Analyser {

    private browser: Browser
    private page: Page
    private contexts: (Frame | Worker)[] = []
    private isRunning = false


    constructor() { }

    async start(url: string, { headless } = { headless: false }) {
        if (this.isRunning === true) {
            throw new Error('Analyser is already running. Stop the Analyser before starting again')
        }
        this.isRunning = true
        this.browser = await chromium.launch({
            headless, args: [
                '--disable-web-security',
                '--js-flags="--max_old_space_size=8192"'
            ]
        });
        this.page = await this.browser.newPage();
        const initScript = await this.constructInitScript()

        await this.page.addInitScript({ content: initScript })

        await this.page.route(`**/*.js*`, async route => {
            const response = await route.fetch()
            const script = await response.text()
            try {
                acorn.parse(script, { ecmaVersion: 'latest' })
                const body = `${initScript}${script}`
                await route.fulfill({ response, body: body })
            } catch {
                route.fulfill({ response, body: script })
            }
        })

        await this.page.route(`**/r3-trace`, async route => {
            const request = await route.request()
            // fetch(`http://localhost:8000/trace`, {
            //     method: 'POST',
            //     body: request.postData(),
            // })
            const url = new URL(route.request().url());
            url.hostname = '127.0.0.1';
            url.port = '8000';
            console.log(url.toString())
            route.continue({ url: url.toString(),  })
                .then(() => console.log('success'))
                .catch((e) => console.log(e))
        })
        await this.page.route('**/r3-trace', async (route) => {
            const interceptedRequest = await route.request();

            // Capture necessary request data
            const postData = interceptedRequest.postDataBuffer();
            const headers = interceptedRequest.headers();

            // Exclude certain headers that might interfere with the request
            delete headers['content-encoding']; // Remove if present
            delete headers['content-length']; // Remove if present

            // Options for the forwarding request
            const requestOptions = {
                hostname: 'localhost',
                port: 8000,
                path: '/trace',
                method: interceptedRequest.method(),
                headers: headers,
            };

            // Create a request to your local server
            const req = http.request(requestOptions, (res) => {
                // Optional: Handle the response from your local server
            });

            // Handle potential errors
            req.on('error', (e) => {
                console.error(`Problem with request: ${e.message}`);
            });

            // Write the captured data to the request body
            if (postData) {
                req.write(postData);
            }

            // End the request
            req.end();

            // You might need to send a dummy response back to the original requestor
            route.fulfill({
                status: 200,
                contentType: 'text/plain',
                body: 'Request forwarded'
            });
        });

        this.page.on('worker', worker => {
            this.contexts.push(worker)
        })

        await this.page.goto(url)
        return this.page
    }

    async stop(): Promise<number[][]> {
        if (this.isRunning === false) {
            throw new Error('Analyser is not running. Start the Analyser before stopping')
        }
        this.contexts = this.contexts.concat(this.page.frames())
        const originalWasmBuffer = await this.getBuffers()

        this.contexts = []
        this.browser.close()
        this.isRunning = false
        return originalWasmBuffer
    }

    private async getBuffers() {
        const originalWasmBuffer = await Promise.all(this.contexts.map(c => c.evaluate(() => {
            try {
                return Array.from(originalWasmBuffer)
            } catch {
                return []
            }
        })))
        return originalWasmBuffer.flat(1) as number[][]
    }

    private async constructInitScript() {
        const wasabiScript = await fs.readFile('./dist/wasabi.js') + '\n'
        const setupScript = await fs.readFile('./src/runtime.js') + '\n'
        return wasabiScript + ';' + setupScript + ';'
    }
}

