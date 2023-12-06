import { StoreOp, LoadOp, ImpExp, Wasabi } from '../wasabi.cjs'
import { Trace as TraceType, ValType, WasmEvent } from '../trace.d.cjs'
import { AnalysisI } from './analyser.cjs'
// import fs from 'fs'

function parseNumber(str): number {
    str = str.trim(); // Remove leading/trailing whitespace

    if (str === '' || str === '+' || str === '-') {
        // Handle empty or only sign character
        return NaN;
    }

    if (str === 'Infinity' || str === '+Infinity') {
        return Infinity;
    }

    if (str === '-Infinity') {
        return -Infinity;
    }

    if (!isNaN(str)) {
        if (str.includes('.') || str.toLowerCase().includes('e')) {
            // Handle floats and scientific notation
            return parseFloat(str);
        }

        let num = BigInt(str);
        if (num >= Number.MIN_SAFE_INTEGER && num <= Number.MAX_SAFE_INTEGER) {
            // Convert to Number if within safe range
            return Number(num);
        }

        //@ts-ignore
        return num; // Return as BigInt
    }

    return NaN; // Not a number
}

export class Trace {
    private trace: string[]
    private worker: Worker
    private cache: string[] = []
    private socketConnected = false


    constructor() {
        this.trace = []

        const script = `
        function chunkArray(array, chunkSize) {
            const chunks = [];
            for (let i = 0; i < array.length; i += chunkSize) {
                chunks.push(array.slice(i, i + chunkSize));
            }
            return chunks;
        }
        const socket = new WebSocket("ws://localhost:8080/trace.r3")
        self.onmessage = function(e) {
            console.log("SEND")
            if (e.data.length > 10000) {
                console.log("CHUNK")
                chunkArray(e.data, 10000).forEach(chunk => {
                    console.log(chunk)
                    socket.send(chunk)})
            } else {
                socket.send(e.data)
            }
        }
        socket.onerror = function(e) {
            console.log(e)
        }
        socket.onopen = function(e) {
            self.postMessage(true)
        }
        `
        const blob = new Blob([script], { type: 'application/javascript' });
        const workerUrl = URL.createObjectURL(blob);
        this.worker = new Worker(workerUrl)
        this.worker.onmessage = (e) => {
            this.socketConnected = true
        }
    }

    push(event: string) {
        this.trace.push(event)
        if (this.socketConnected && this.trace.length >= 10000) {
            this.worker.postMessage(this.trace)
            this.trace = []
        }
    }
}

type Options = { extended: boolean }
export default class Analysis implements AnalysisI<Trace> {

    private trace: Trace = new Trace()
    private Wasabi: Wasabi
    private options: Options


    getResult(): Trace {
        return this.trace
    }

    constructor(Wasabi: Wasabi, options = { extended: false }) {
        this.Wasabi = Wasabi
        this.options = options

        Wasabi.analysis = {
            start(location) { },
            nop(location) { },
            unreachable(location) { },
            if_(location, condition) { },
            br(location, target) { },
            br_if(location, conditionalTarget, condition) { },
            br_table(location, table, defaultTarget, tableIdx) { },
            begin(location, type) { },
            drop(location, value) { },
            select(location, cond, first, second) { },
            const_(location, op, value) { },
            unary(location, op, input, result) { },
            binary(location, op, first, second, result) { },
            memory_size(location, memoryIdx, currentSizePages) { },
            local(location, op, localIndex, value) { },
            memory_fill(location, index, value, length) { },
            memory_copy(location, destination, source, length) { },
            memory_init(location, destination, source, length) { },
            table_size(location, currentSizeEntries) { },
            table_copy(location, destination, source, length) { },
            table_init(location, destination, source, length) { },
            table_grow(location, n, val, previusElement) { },
            table_fill(location, index, value, length) { },

            begin_function: (location, args) => {
                this.trace.push(`E;${location.func};${args.join('|')}`)
            },

            store: (location, op, target, memarg, value) => {
                // this.trace.push(`S;${op};${target.addr}`)
                // this.trace.push(`${memarg.offset};${value}`)
                this.trace.push(`S;${op};${target.addr};${memarg.offset};${value}`)
            },

            memory_grow: (location, memIdx, byPages, previousSizePages) => {
                this.trace.push(`MG;${byPages}`)
            },

            load: (location, op, target, memarg, value) => {
                // this.trace.push(`L;${op};${target.addr}`)
                // this.trace.push(`${memarg.offset};${value}`)
                this.trace.push(`L;${op};${target.addr};${memarg.offset};${value}`)
            },

            global: (location, op, idx, value) => {
                this.trace.push(`G;${idx};${op};${value}`)
            },

            call_pre: (location, op, funcidx, args, tableTarget) => {
                this.trace.push(`PR;${funcidx};${op};${tableTarget?.tableIdx};${tableTarget?.elemIdx}`)
            },

            call_post: (location, results) => {
                this.trace.push(`PO;${results.join('|')}`)
            },

            return_: (location, values) => {
                this.trace.push(`FR;${location.func};${values.join('|')}`)
            },

            table_set: (location, target, value) => {
                this.trace.push(`TS;${target.tableIdx};${target.elemIdx};${value.name}`)
            },

            table_get: (location, target, value) => {
                this.trace.push(`TS;${target.tableIdx};${target.elemIdx};${value.name}`)
            }
        }
    }

    init() {
        // Init Memories
        this.Wasabi.module.info.memories.forEach((m, idx) => {
            if (m.import !== null) {
                this.trace.push(`IM;${idx};${m.import[0]};${m.import[1]};${m.initial};${m.maximum === null ? '' : m.maximum}`)
            }
        })
        // Init Tables
        this.Wasabi.module.info.tables.forEach((t, idx) => {
            if (t.import !== null) {
                this.trace.push(`IT;${idx};${t.import![0]};${t.import![1]};${t.initial};${t.maximum === null ? '' : t.maximum};${'anyfunc'}`) // want to replace anyfunc through t.refType but it holds the wrong string ('funcref')
            }
            const table = this.Wasabi.module.tables[idx]
        })
        // Init Globals
        this.Wasabi.module.info.globals.forEach((g, idx) => {
            if (g.import !== null) {
                this.trace.push(`IG;${idx};${g.import[0]};${g.import[1]};${g.valType};${g.mutability === 'Mut' ? 1 : 0};${this.Wasabi.module.globals[idx].value}`)
            }
        })
        // Init Functions
        this.Wasabi.module.info.functions.forEach((f, idx) => {
            if (f.import !== null) {
                this.trace.push(`IF;${idx};${f.import![0]};${f.import[1]}`)
            }
        })
    }
}
