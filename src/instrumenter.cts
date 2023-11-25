import path from 'path';
import fss from 'fs'
import Analyser from './analyser.cjs';
import { askQuestion } from './util.cjs';
import { Options } from './cli/options.cjs'

export default async function run(url: string, options: Options) {
  if (options.callGraph === true) {
    throw new Error('Option callGraph currently not supported')
  } else {
    const analyser = new Analyser()
    await analyser.start(url, { headless: options.headless })
    await askQuestion(`Record is running. Enter 'Stop' to stop recording: `)
    console.log(`Record stopped. Downloading...`)
    const results = await analyser.stop()
  }
}