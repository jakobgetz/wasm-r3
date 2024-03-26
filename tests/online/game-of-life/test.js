import { delay } from '../../../dist/tests/test-utils.cjs'

export default async function test(analyser) {
  const url = 'https://playgameoflife.com/'
  const page = await analyser.start(url, { headless: true })

  // const button = page.locator('#start')
  // await button.waitFor({ state: 'visible' })
  // await button.click()
  // await delay(500)
  // await button.click()

  const nextButton = page.locator('#next')
  await nextButton.waitFor({ state: 'visible' })
  const N = 10
  for (let i = 0; i < N; i++) {
    await nextButton.click()
    await delay(1000)
  }

  return await analyser.stop()
}
