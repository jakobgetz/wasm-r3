/*
 * type definitions for trace implementation
 * according to the trace specification
 */

type Trace = WasmEvent[]

type WasmEvent = Load | MemGrow | TableGet | TableGrow | GlobalGet | ExportCall | ImportCall | ImportReturn | ImportMemory | ImportTable | ImportGlobal

type Load = { type: "Load", name: string, offset: number, data: Uint8Array }

type MemGrow = { type: 'MemGrow', name: string, amount: number }

type TableGet = { type: "TableGet", name: string, idx: number }

type TableGrow = { type: 'TableGrow', name: string, amount: number }

type GlobalGet = { type: 'GlobalGet', name: string, value: number }

type ExportCall = { type: "ExportCall", names: string[], params: number[] }

type ImportCall = { type: "ImportCall", funcidx: number, module: string, name: string }

type ImportReturn = { type: "ImportReturn", funcidx: number, results: number[] }

type ImportMemory = { type: 'ImportMemory', module: string, name: string, pages: number }

type ImportTable = { type: 'ImportTable', module: string, name: string, reftype: "funcref" | "externref" }

type ImportGlobal = { type: 'ImportGlobal', module: string, name: string, valtype: string, value: number }