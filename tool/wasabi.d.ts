type Location = { func: number, instr: number }
type ValType = 'i32' | 'i64' | 'f32' | 'f64' | 'anyfunc' | 'funcref' | 'externref'

export declare type Wasabi = {
    HOOK_NAMES: [
        "start",
        "if_",
        "br",
        "br_if",
        "br_table",
        "begin",
        "end",
        "nop",
        "unreachable",
        "drop",
        "select",
        "call_pre",
        "call_post",
        "return_",
        "const_",
        "unary",
        "binary",
        "load",
        "store",
        "memory_size",
        "memory_grow",
        "local",
        "global"
    ],
    module: {
        info: {
            functions: {
                type: string,
                import: string[],
                export: string[],
                locals: string,
                instrCount: number
            }[],
            memories: {
                import: string[] | null,
                export: string[]
            }[],
            tables: {
                import: string[] | null,
                export: string,
                ref_type: any
            }[],
            globals: {
                import: string[] | null,
                export: string[],
                valType: ValType,
            }[],
            start: any,
            tableExportName: string,
            brTables: any[],
            originalFunctionImportsCount: number
        },
        lowlevelHooks: any,
        exports: {
            [name: string]: any
        },
        tables: WebAssembly.Table[],
        memories: WebAssembly.Memory[],
        globals: WebAssembly.Global[],
    },
    resolveTableIdx: Function,
    analysis: {
        start?: (location: Location, ...args: any) => void,
        if_?: (location: Location, ...args: any) => void,
        br?: (location: Location, ...args: any) => void,
        br_if?: (location: Location, ...args: any) => void,
        br_table?: (location: Location, ...args: any) => void,
        begin?: (location: Location, ...args: any) => void,
        begin_function?: (location: Location, args: number[]) => void, 
        end?: (location: Location, ...args: any) => void,
        nop?: (location: Location, ...args: any) => void,
        unreachable?: (location: Location, ...args: any) => void,
        drop?: (location: Location, ...args: any) => void,
        select?: (location: Location, ...args: any) => void,
        call_pre?: (location: Location, targetFunc: number, args: number[], indirectTableIdx: number | undefined) => void,
        call_post?: (location: Location, values: number[]) => void,
        return_?: (location: Location, ...args: any) => void,
        const_?: (location: Location, ...args: any) => void,
        unary?: (location: Location, ...args: any) => void,
        binary?: (location: Location, ...args: any) => void,
        load?: (location: Location, ...args: any) => void,
        store?: (location: Location, ...args: any) => void,
        memory_size?: (location: Location, ...args: any) => void,
        memory_grow?: (location: Location, ...args: any) => void,
        local?: (location: Location, ...args: any) => void,
        global?: (location: Location, op: 'global.get' | 'global.set', ...args: any) => void,
    }
}