import cp from 'child_process'
import fs from 'fs/promises'
import path from 'path'
import { writeWithSpaces } from './test-utils.cjs'
import { online_filter } from './filter.cjs'
const onlinePath = './tests/online'
const wizeng = process.env.WIZENG || 'wizeng.x86-64-linux';

async function run() {
    try {
        await fs.access(wizeng)
    } catch {
        console.log('wizeng not found')
        return
    }
    let folders = await fs.readdir(onlinePath);
    let custom_filter = online_filter.concat([
        'fib', // takes to long
        'guiicons', // jit panicks
        "mandelbrot", // jit, dyn panicks
        "virtualkc", // jit panicks
        "kittygame", // jit, dyn gets stuck
    ])
    folders = folders.filter(folder => !custom_filter.includes(folder));
    // folders = ['ffmpeg']
    for (let folder of folders) {
        let benchmarkPath = path.join(onlinePath, folder, 'benchmark')
        let subBenchmarks
        try {
            subBenchmarks = await fs.readdir(benchmarkPath)
        } catch {
            continue
        }
        let node: any = true
        let deno: any = true
        let bun: any = true
        writeWithSpaces(folder + ':')
        for (let subBenchmark of subBenchmarks) {
            let replay = path.join(benchmarkPath, subBenchmark, 'replay.wasm')
            let config = {
                'int': { command: `${wizeng} --monitors=profile ${replay}`, r3: -1, total: -1 },
                'dyn': { command: `${wizeng} -mode=dyn --monitors=profile ${replay}`, r3: -1, total: -1 },
                'jit': { command: `${wizeng} -mode=jit --monitors=profile ${replay}`, r3: -1, total: -1 },
            }
            for (let key in config) {
                let command = config[key].command
                let buffer = cp.execSync(command, { maxBuffer: 1024 * 5000 * 1000 })
                let lines = buffer.toString().split('\n')
                let total = 0;
                let r3 = 0;
                for (let line of lines) {
                    line = line.replace(/\x1B\[\d+m/g, '');
                    let percentageIndex = line.indexOf('%');

                    if (percentageIndex !== -1) {
                        // Get the substring that ends with the '%' character
                        let percentage = line.slice(0, percentageIndex);

                        // Split the substring into words
                        let words = percentage.split(/\s+/);

                        // Get the third last word (the percentage number)
                        let number = parseFloat(words[words.length - 1]);
                        if (isNaN(number)) continue
                        total += number
                        if (words[2].startsWith('"r3')) {
                            r3 += number;
                        }
                    }
                }
                config[key].r3 = r3
                config[key].total = total
            }

            console.log(`int: ${config.int.r3.toFixed(2)}% / ${config.int.total.toFixed(2)}% | dyn: ${config.dyn.r3.toFixed(2)}% / ${config.dyn.total.toFixed(2)}% | jit: ${config.jit.r3.toFixed(2)}% / ${config.jit.total.toFixed(2)}%`);
        }
    }
}


run();
console.log("You have to first run `run-tests.cts` because the replays should be already generated.")