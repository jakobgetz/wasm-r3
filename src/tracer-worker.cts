import { StoreOp, LoadOp, ImpExp, Wasabi } from "../wasabi.cjs"
import { Trace as TraceType, ValType, WasmEvent } from "../trace.cjs"
import { AnalysisI } from "./analyser.cjs"

function numberToUint8Array(value, byteLength) {
    const buffer = new ArrayBuffer(byteLength);
    const view = new DataView(buffer);

    // Depending on the byte length, choose the appropriate method
    switch (byteLength) {
        case 1:
            view.setUint8(0, value);
            break;
        case 2:
            view.setUint16(0, value, true); // true for little endian
            break;
        case 4:
            view.setUint32(0, value, true); // true for little endian
            break;
        // Add more cases for larger byte lengths if needed
        default:
            throw new Error("Unsupported byte length");
    }

    return new Uint8Array(buffer);
}

function parseNumber(str): number {
    str = str.trim(); // Remove leading/trailing whitespace

    if (str === "" || str === "+" || str === "-") {
        // Handle empty or only sign character
        return NaN;
    }

    if (str === "Infinity" || str === "+Infinity") {
        return Infinity;
    }

    if (str === "-Infinity") {
        return -Infinity;
    }

    if (!isNaN(str)) {
        if (str.includes(".") || str.toLowerCase().includes("e")) {
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

class Trace {
    private trace: string[]
    private cache: string[] = []
    private flag = true

    constructor() {
        this.trace = []
    }

    push(event: string) {
        if (event.startsWith("IC") || event.startsWith("IR")) {
            const cashIndex = this.cache.findIndex(v => v === event)
            if (cashIndex !== -1) {
                this.trace.push(cashIndex.toString())
                return
            } else {
                this.cache.push(event)
            }
        }
        this.trace.push(event)
        // if (this.trace.length > 15000 && this.flag === true) {
        //     fs.writeFileSync("/Users/jakob/Desktop/wasm-r3/ffmpeg/bin_0/replay-trace.r3", this.toString())
        //     this.flag = false
        // }
    }

    forEach(callbackFn: (value: WasmEvent, index: number) => void) {
        this.trace.forEach((e, i) => {
            const event = this.parseEventToObj(e)
            callbackFn(event, i)
        })
    }

    parseEventToObj(event: string): WasmEvent {
        const cacheIdx = Number(event)
        if (Number.isFinite(cacheIdx)) {
            event = this.cache[cacheIdx]
        }
        function splitList(c?: string) {
            let list = c?.split(",").map(parseNumber)
            if (list === undefined || (list.length === 1 && Number.isNaN(list[0]))) {
                return []
            }
            return list
        }

        let components = event.split(";")
        switch (components[0]) {
            case "IM":
                return {
                    type: "ImportMemory",
                    idx: parseInt(components[1]),
                    module: components[2],
                    name: components[3],
                    initial: parseInt(components[4]),
                    maximum: components[5] === "" ? undefined : parseInt(components[5])
                }
            case "EC":
                return {
                    type: "ExportCall",
                    name: components[1],
                    params: splitList(components[2])
                }
            case "TC":
                return {
                    type: "TableCall",
                    tableName: components[1],
                    funcidx: parseInt(components[2]),
                    params: splitList(components[3])
                }
            case "ER":
                return {
                    type: "ExportReturn"
                }
            case "IC":
                return {
                    type: "ImportCall",
                    idx: parseInt(components[1]),
                    name: components[2]
                }
            case "IR":
                return {
                    type: "ImportReturn",
                    idx: parseInt(components[1]),
                    name: components[2],
                    results: splitList(components[3]),
                }
            case "L":
                return {
                    type: "Load",
                    idx: parseInt(components[1]),
                    name: components[2],
                    offset: parseInt(components[3]),
                    data: splitList(components[4]),
                }
            case "LE":
                return {
                    type: "LoadExt",
                    idx: parseInt(components[1]),
                    name: components[2],
                    offset: parseInt(components[3]),
                    data: splitList(components[4]),
                }
            case "MG":
                return {
                    type: "MemGrow",
                    idx: parseInt(components[1]),
                    name: components[2],
                    amount: parseInt(components[3])
                }
            case "T":
                return {
                    type: "TableGet",
                    tableidx: parseInt(components[1]),
                    name: components[2],
                    idx: parseInt(components[3]),
                    funcidx: parseInt(components[4]),
                    funcName: components[5],
                }
            case "TE":
                return {
                    type: "TableGetExt",
                    tableidx: parseInt(components[1]),
                    name: components[2],
                    idx: parseInt(components[3]),
                    funcidx: parseInt(components[4]),
                    funcName: components[5],
                }
            case "TG":
                return {
                    type: "TableGrow",
                    idx: parseInt(components[1]),
                    name: components[2],
                    amount: parseInt(components[3])
                }
            case "G":
                return {
                    type: "GlobalGet",
                    idx: parseInt(components[1]),
                    name: components[2],
                    value: parseNumber(components[3]),
                    valtype: components[4] as ValType,
                }
            case "IG":
                return {
                    type: "ImportGlobal",
                    idx: parseInt(components[1]),
                    module: components[2],
                    name: components[3],
                    initial: parseNumber(components[6]),
                    value: components[4] as ValType,
                    mutable: parseInt(components[5]) === 1
                }
            case "IF":
                return {
                    type: "ImportFunc",
                    idx: parseInt(components[1]),
                    module: components[2],
                    name: components[3],
                }
            case "IT":
                return {
                    type: "ImportTable",
                    idx: parseInt(components[1]),
                    module: components[2],
                    name: components[3],
                    initial: parseInt(components[4]),
                    maximum: components[5] === "" ? undefined : parseInt(components[5]),
                    element: components[6] as "anyfunc"
                }
            case "FE":
                return {
                    type: "FuncEntry",
                    idx: parseInt(components[1]),
                    args: splitList(components[2])
                }
            case "FR":
                return {
                    type: "FuncReturn",
                    idx: parseInt(components[1]),
                    values: splitList(components[2])
                }
            default:
                throw new Error(`${components[0]}: Not a valid trace event. The whole event: ${event}.`)
        }
    }

    toString() {
        return this.trace.join(`\n`)
    }

    static fromString(traceString: string) {
        let self = new Trace()
        self.trace = []
        let events = traceString.trim().split("\n")
        for (let event of events) {
            self.push(event)
        }
        return self
    }

    getTop() {
        return this.trace[this.trace.length - 1]
    }

    resolve(event: string) {
        if (Number.isFinite(Number(event))) {
            return this.cache[parseInt(event)]
        }
        return event
    }

    getLength() {
        return this.trace.length
    }
}

type Options = { extended: boolean }
class Analysis implements AnalysisI<Trace> {

    trace: Trace = new Trace()
    private info: any
    private options: Options

    // shadow stuff
    private shadowMemories: ArrayBuffer[] = []
    private shadowGlobals: number[] = []
    private shadowTables: WebAssembly.Table[] = []

    // helpers
    private callStack: ("int" | { name: string, idx: number })[] = [{ name: "main", "idx": -1 }]

    private MEM_PAGE_SIZE = 65536


    getResult(): Trace {
        return this.trace
    }

    constructor(options = { extended: false }) {
        this.options = options
    }

    begin_function(location, args) {
        if (this.options.extended) {
            this.trace.push(`FE;${location.func};${args.join(",")}`)
        }
        if (this.callStack[this.callStack.length - 1] !== "int") {
            const exportName = this.info.functions[location.func].export[0]
            const CALLED_WITH_TABLE_GET = exportName === undefined
            if (CALLED_WITH_TABLE_GET) {
                // if (!this.Wasabi.module.tables.some((table, i) => {
                //     for (let tableIndex = 0; tableIndex < table.length; tableIndex++) {
                //         const funcidx = this.resolveFuncIdx(table, tableIndex)
                //         if (funcidx === location.func) {
                //             this.trace.push(`TC;${this.getName(this.info.tables[i])};${tableIndex};${args.join(",")}`)
                //             return true
                //         }
                //     }
                //     return false
                // })) {
                //     throw new Error("The function you where calling from outside the wasm module is neither exported nor in a table...")
                // }
            } else {
                this.trace.push(`EC;${exportName};${args.join(",")}`)
                // this.checkMemGrow()
                // this.checkTableGrow()
            }
        }
        this.callStack.push("int")
    }

    store(location, op, target, memarg, value) {
        const addr = target.addr + memarg.offset
        if (this.shadowMemories.length === 0) {
            return
        }
        let byteLength = this.getByteLength(op)
        const data = numberToUint8Array(value, byteLength)
        for (let i = 0; i < byteLength; i++) {
            const value = data[i]
            new Uint8Array(this.shadowMemories[0])[addr + i] = value
        }
    }

    memory_grow(location, memIdx, byPages, previousSizePages) {
        if (this.shadowMemories.length === 0) {
            return
        }
        // this.growShadowMem(memIdx, byPages)
    }

    load(location, op, target, memarg, value) {
        if (this.shadowMemories.length === 0) {
            return
        }
        const addr = target.addr + memarg.offset
        const memName = this.getName(this.info.memories[target.memIdx])
        let byteLength = this.getByteLength(op)
        const data = numberToUint8Array(value, byteLength)
        const res = this.mem_content_equals(target.memIdx, addr, byteLength, data)
        if (this.options.extended === true) {
            this.trace.push(`LE;${0};${memName};${addr};${data.join(",")}`)
        }
        res.forEach((r, i) => {
            if (r !== true) {
                new Uint8Array(this.shadowMemories[0])[addr + i] = data[i]
                this.trace.push(`L;${0};${memName};${addr + i};${[r as number]}`)
            }
        })
    }

    global(location, op, idx, value) {
        if (op === "global.set") {
            this.shadowGlobals[idx] = value
            return
        }
        if (op === "global.get") {
            let globalInfo = this.info.globals[idx]
            // can be NaN in case of the NaN being imported to the WebAssembly Module. Google it!
            if (this.shadowGlobals[idx] !== value && !Number.isNaN(this.shadowGlobals[idx]) && !Number.isNaN(value)) {
                let valtype = globalInfo.valType
                this.trace.push(`G;${idx};${this.getName(globalInfo)};${value};${valtype}`)
            }
        }
    }

    call_pre(location, op, funcidx, args, tableTarget) {
        if (op === "call_indirect") {
            this.tableGetEvent(tableTarget.tableIdx, tableTarget.elemIdx)
        }
        let funcImport = this.info.functions[funcidx].import
        if (funcImport !== null) {
            let name = funcImport[1]
            this.callStack.push({ name, idx: funcidx })
            this.trace.push(`IC;${funcidx};${name}`)
        }
    }

    call_post(location, results) {
        const func = this.callStack[this.callStack.length - 1]
        if (func === "int") {
            return
        }
        this.callStack.pop()
        this.trace.push(`IR;${func.idx};${func.name};${results.join(",")}`)
        // this.checkMemGrow()
        // this.checkTableGrow()
    }

    return_(location, values) {
        this.callStack.pop()
        if (this.options.extended === true) {
            this.trace.push(`FR;${location.func};${values.join(",")}`)
        }
        if (this.callStack.length === 1) {
            this.trace.push(`ER`)
        }
    }

    table_set(location, target, value) {
        this.shadowTables[target.tableIdx].set(target.elemIdx, value)
    }

    table_get(location, target, value) {
        this.tableGetEvent(target.tableIdx, target.elemIdx)
    }

    private mem_content_equals(memIdx: number, addr: number, numBytes: number, data: Uint8Array): (number | boolean)[] {
        let result = []
        for (let i = 0; i < numBytes; i++) {
            if (new Uint8Array(this.shadowMemories[memIdx])[addr + i] !== data[i]) {
                result.push(data)
            } else {
                result.push(true)
            }
        }
        return result
    }

    private tableGetEvent(tableidx: number, idx: number) {
        // let table = this.Wasabi.module.tables[tableidx]
        // let shadowTable = this.shadowTables[tableidx]
        // let resolvedFuncIdx = this.resolveFuncIdx(table, idx)
        // if (shadowTable.get(idx) !== table.get(idx)) {
        //     let name = this.getName(this.info.tables[tableidx])
        //     let funcidx = parseInt(table.get(idx).name)
        //     let funcName = this.getName(this.info.functions[resolvedFuncIdx])
        //     this.trace.push(`T;${tableidx};${name};${idx};${funcidx};${funcName}`)
        //     this.shadowTables[0].set(0, table.get(idx))
        // }
        // if (this.options.extended === true) {
        //     let name = this.getName(this.info.tables[tableidx])
        //     let funcidx = parseInt(table.get(idx).name)
        //     let funcName = this.getName(this.info.functions[resolvedFuncIdx])
        //     this.trace.push(`TE;${tableidx};${name};${idx};${funcidx};${funcName}`)
        // }
    }

    private resolveFuncIdx(table: WebAssembly.Table, idx: number) {
        let resolvedFuncIdx = parseInt(table.get(idx).name)
        // if (resolvedFuncIdx >= this.info.originalFunctionImportsCount) {
        //     resolvedFuncIdx = resolvedFuncIdx - Object.keys(this.Wasabi.module.lowlevelHooks).length
        // }
        return resolvedFuncIdx
    }

    // private growShadowMem(memIdx, byPages: number) {
    //     const newShadow = new ArrayBuffer(this.shadowMemories[memIdx].byteLength + byPages * this.MEM_PAGE_SIZE)
    //     new Uint8Array(this.shadowMemories[memIdx]).forEach((b, i) => {
    //         new Uint8Array(newShadow)[i] = b
    //     })
    //     this.shadowMemories[0] = newShadow
    // }

    // private growShadowTable(tableIdx, amount: number) {
    //     const newShadow = new WebAssembly.Table({ initial: this.Wasabi.module.tables[0].length, element: "anyfunc" })
    //     for (let i = 0; i < this.Wasabi.module.tables[tableIdx].length; i++) {
    //         newShadow.set(i, this.Wasabi.module.tables[tableIdx].get(i))
    //     }
    //     this.shadowTables[0] = newShadow
    // }

    // private checkMemGrow() {
    //     this.Wasabi.module.memories.forEach((mem, idx) => {
    //         if (mem.buffer.byteLength !== this.shadowMemories[idx].byteLength) {
    //             let memGrow: any = {}
    //             let amount = mem.buffer.byteLength / this.MEM_PAGE_SIZE - this.shadowMemories[idx].byteLength / this.MEM_PAGE_SIZE
    //             memGrow[idx] = amount
    //             this.growShadowMem(idx, amount)
    //             this.trace.push(`MG;${idx};${this.getName(this.info.memories[idx])};${amount}`)
    //         }
    //     })
    // }

    // private checkTableGrow() {
    //     this.Wasabi.module.tables.forEach((t, idx) => {
    //         if (t.length !== this.shadowTables[idx].length) {
    //             let tableGrow: any = {}
    //             let amount = this.Wasabi.module.tables[idx].length - this.shadowTables[idx].length
    //             tableGrow[idx] = amount
    //             this.growShadowTable(idx, amount)
    //             this.trace.push(`TG;${idx};${this.getName(this.Wasabi.module.info.tables[0])};${amount}`)
    //         }
    //     })
    // }

    private getName(entity: ImpExp) {
        if (entity.import !== null) {
            return entity.import[1]
        } else {
            return entity.export[0]
        }
    }


    /**
     * @example "i32.load" => 4
     * "i32.load16" => 2
     * "i64.store" => 8
     * "i64.store8_u" => 1
     */
    private getByteLength(instr: StoreOp | LoadOp) {
        let typeIndex = 9
        if (instr.charAt(4) === "l") {
            typeIndex = 8
        }
        if (instr.charAt(typeIndex) === "8") {
            return parseInt(instr.charAt(typeIndex)) / 8
        } else if (instr.charAt(typeIndex) === "1" || instr.charAt(typeIndex) === "3") {
            return parseInt(instr.substring(typeIndex, typeIndex + 2)) / 8
        }
        return parseInt(instr.substring(1, 3)) / 8
    }

    private cloneArrayBuffer(original: ArrayBuffer) {
        const cloned = new ArrayBuffer(original.byteLength);
        const sourceArray = new Uint8Array(original);
        const clonedArray = new Uint8Array(cloned);

        // Copy the data from the original ArrayBuffer to the cloned ArrayBuffer
        for (let i = 0; i < original.byteLength; i++) {
            clonedArray[i] = sourceArray[i];
        }

        return cloned;
    }

    setInfo(info: any) {
        this.info = info
    }

    setShadowMemories(memories) {
        this.shadowMemories = memories
    }

    setShadowTables(tables) {
        this.shadowTables = tables
    }

    setShadowGlobals(globals) {
        this.shadowGlobals = globals
    }
}

const analysis = new Analysis()

self.onmessage = function (e) {
    if (typeof e === "string") {
        analysis.trace.push(e)
    }
    switch (e.type) {
        case "begin_function":
            analysis.begin_function(e.location, e.args)
            break
        case "store":
            analysis.store(e.location, e.op, e.target, e.memarg, e.value)
            break
        case "memory_grow":
            analysis.memory_grow(e.location, e.memIdx, e.byPages, e.previousSizePages)
            break
        case "load":
            analysis.load(e.location, e.op, e.target, e.memarg, e.value)
            break
        case "global":
            analysis.global(e.location, e.op, e.idx, e.value)
            break
        case "call_pre":
            analysis.call_pre(e.location, e.op, e.funcidx, e.args, e.tableTarget)
            break
        case "call_post":
            analysis.call_post(e.location, e.results)
            break
        case "return":
            analysis.return_(e.location, e.values)
            break
        case "table_set":
            analysis.table_set(e.location, e.target, e.value)
            break
        case "table_get":
            analysis.table_get(e.location, e.target, e.value)
            break
        case "info":
            analysis.setInfo(e.info)
            break
        case "memories":
            analysis.setShadowMemories(e.memories)
            break
        case "tables":
            analysis.setShadowTables(e.tables)
            break
        case "globals":
            analysis.setShadowGlobals(e.globals)
            break
        default:
            throw new Error("Unknown message type to tracer worker")
    }
}