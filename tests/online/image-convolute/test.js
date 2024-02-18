export default async function test(analyser) {
    // analyser.setExtended(false)
    const url = 'https://takahirox.github.io/WebAssembly-benchmark/tests/imageConvolute.html'
    const page = await analyser.start(url, { headless: false })
    const buttonLocator = page.locator('#run_button')
    await buttonLocator.waitFor({ state: 'visible' })
    const doneLocator = page.locator('#message')
    await buttonLocator.click()
    await doneLocator.waitFor({ state: 'visible', timeout: 240000 })
    await doneLocator.waitFor(() => this.textContent().includes('Done'), { timeout: 240000 })
    return await analyser.stop()
}