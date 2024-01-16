import { chromium } from 'playwright'

async function run() {
    let browser = await chromium.launch({ headless: false, args: ['--experimental-wasm-multi-memory'] });
    let page = await browser.newPage();
    await page.goto('https://playgameoflife.com')
}

run()