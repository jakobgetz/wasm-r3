import fss from 'fs'
import { Analyser, CustomAnalyser } from './analyser.cjs';
import Benchmark from './benchmark.cjs';
import { askQuestion } from './util.cjs';
import { Options } from './cli/options.cjs'
import Generator from './replay-generator.cjs';
import { createMeasure, initPerformance } from './performance.cjs';
import { generateJavascript } from './js-generator.cjs';
import { AnalyserI } from './analyser.cjs';
import { execSync } from 'child_process';
import path from 'path';

export default async function run(url: string, options: Options) {
  await initPerformance(url, 'manual-run', 'performance.ndjson')
  if (options.file !== undefined) {
    const code = await new Generator().generateReplayFromStream(fss.createReadStream(options.file))
    generateJavascript(fss.createWriteStream(options.file + '.js'), code)
    return
  }
  let analyser: AnalyserI
  if (options.customInstrumentation === true) {
    console.log(`Using RUST frontend and backend`)
    const tracePath = 'bin_trace.r3'
    const binPath = 'binary.wasm'
    analyser = new CustomAnalyser(tracePath)
    await analyser.start(url, { headless: options.headless })
    await askQuestion(`Record is running. Enter 'Stop' to stop recording: `)
    console.log(`Record stopped. Downloading...`)
    const results = await analyser.stop()
    console.log('Download done. Generating Benchmark...')
    fss.writeFileSync(binPath, new Int8Array(results[0].wasm))
    const p_measureCodeGen = createMeasure('rust-backend', { phase: 'replay-generation', description: `The time it takes for rust backend to generate javascript` })
    execSync(`./target/debug/replay_gen ${tracePath} ${binPath} true`);
    execSync(`wasm-validate ${path.join(binPath, "canned.wasm")}`)
    p_measureCodeGen()
  } else {
    analyser = new Analyser('./dist/src/tracer.cjs', { extended: options.extended, noRecord: options.noRecord })
    await analyser.start(url, { headless: options.headless })
    await askQuestion(`Record is running. Enter 'Stop' to stop recording: `)
    console.log(`Record stopped. Downloading...`)
    const results = await analyser.stop()
    console.log('Download done. Generating Benchmark...')
    Benchmark.fromAnalysisResult(results).save(options.benchmarkPath, { trace: options.dumpTrace, rustBackend: options.rustBackend })
  }
}