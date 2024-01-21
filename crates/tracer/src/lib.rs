use std::{collections::HashMap, fmt::Debug};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn instrument_wasm_js(buffer: &[u8]) -> Result<JsValue, JsValue> {
    let buffer = instrument_wasm(buffer).map_err(|e| JsValue::from_str(e))?;
    let uint8_array = js_sys::Uint8Array::new_with_length(buffer.len() as u32);
    uint8_array.copy_from(&buffer);
    Ok(JsValue::from(uint8_array))
}

const TRACE_MEM: &str = "$trace_mem";
const TRACE_MEM_PAGES: u32 = 10_000;
const TRACE_MEM_MAX_FILL_PERCENT: f32 = 0.6;
const TRACE_MEM_EXPORT_NAME: &str = "trace";
const MEM_POINTER: &str = "$mem_pointer";
const MEM_POINTER_EXPORT_NAME: &str = "trace_byte_length";
const LOCAL_FUNCREF: &str = "$funcref";
const FUNCREF_TABLE: &str = "$table";
const FUNCREF_TABLE_SIZE: u32 = 100_000;
const FUNCREF_TABLE_EXPORT_NAME: &str = "lookup_table";
const TABLE_POINTER: &str = "$table_pointer";
const TABLE_POINTER_EXPORT_NAME: &str = "lookup_table_pointer";
const CHECK_MEM: &str = "$check_mem";
const CHECK_MEM_IMPORT_MODULE: &str = "r3";
const CHECK_MEM_IMPORT_NAME: &str = "check_mem";
const LOCAL_ADDR: &str = "$addr_local";
const LOCAL_I32: &str = "$i32";
const LOCAL_I64: &str = "$i64";
const LOCAL_F32: &str = "$f32";
const LOCAL_F64: &str = "$f64";

pub fn instrument_wasm(buffer: &[u8]) -> Result<Vec<u8>, &'static str> {
    let orig_wat = wasmprinter::print_bytes(buffer).unwrap();
    let mut orig_wat = orig_wat.split('\n').peekable();

    // First loop: gather metadata
    let mut types = Vec::new();
    let mut types_by_functions = HashMap::new();
    let mut type_idx = 0;
    let mut globals = Vec::new();
    let mut func_idx = 0;
    for l in orig_wat.clone() {
        let l = l.trim();
        if l.starts_with("(type") {
            types.push(FuncType::new(l, type_idx)?);
            type_idx += 1;
        } else if l.starts_with("(import") && l.contains("(func") {
            let type_idx = get_type_idx_by_func_import(l)?;
            types_by_functions.insert(func_idx, types.get(type_idx as usize).unwrap().clone());
            func_idx += 1;
        } else if l.starts_with("(func") {
            let type_idx = get_type_idx_by_func_def(l)?;
            types_by_functions.insert(func_idx, types.get(type_idx as usize).unwrap().clone());
            func_idx += 1;
        } else if l.starts_with("(global") {
            globals.push(ValType::from_global(l)?);
        }
    }

    // Second loop: generate the instrumented module
    let mut gen_wat = Vec::new();
    func_idx = 0;
    let mut local_count = 0;
    let mut typ: FuncType = FuncType {
        idx: 0,
        params: vec![],
        results: vec![],
    };
    let mut inside_func = false;
    while let Some(l) = orig_wat.next() {
        let mut l = l.trim().to_string();
        let offset = &mut 0;
        if is_new_section(orig_wat.peek()) && (inside_func || l.starts_with("(func")) {
            l.pop();
        }
        if l.starts_with("(import") && l.contains("(func") || l.starts_with("(type") {
            if let Some(next) = orig_wat.peek() {
                if !next.trim().starts_with("(import") && !next.trim().starts_with("(type") {
                    gen_wat.push(l.into());
                    gen_wat.push(format!(
                        "(import \"{}\" \"{}\" (func {}))",
                        CHECK_MEM_IMPORT_MODULE, CHECK_MEM_IMPORT_NAME, CHECK_MEM
                    ));
                } else {
                    gen_wat.push(l.into());
                }
            } else {
                gen_wat.push(l.into());
            }
        } else if l.starts_with("(func") {
            inside_func = true;
            local_count = 0;
            typ = types_by_functions.get(&func_idx).unwrap().clone();
            local_count += typ.params.len();
            if let Some(next) = orig_wat.peek() {
                if next.starts_with("(local") {
                    gen_wat.push(l.into());
                    l = orig_wat.next().unwrap().trim().to_string();
                    local_count += l
                        .split_whitespace()
                        .filter(|&token| token != "local" && token != ")")
                        .count();
                }
            }
            gen_wat.push(l.into());
            gen_wat.push(typ.locals_wat());
            local_count += 6;
            gen_wat.extend(trace_u8(0x02, offset));
            gen_wat.extend(trace_u32(func_idx, offset));
            gen_wat.extend(trace_u32(typ.idx, offset));
            gen_wat.extend(typ.trace_params(offset));
            gen_wat.extend(increment_mem_pointer(offset));
            func_idx += 1;
        } else if l == "return" {
            trace_return(&mut gen_wat, &typ, local_count, offset);
            gen_wat.push(l);
        } else if l.contains("call_indirect") {
            let called_type_idx = get_type_idx_by_call_indirect(&l)?;
            let called_type = types.get(called_type_idx).unwrap();
            gen_wat.extend(trace_u8(0x11, offset));
            gen_wat.extend(trace_stack_value(Some(&ValType::I32), offset));
            gen_wat.push(format!("local.get {}", LOCAL_I32));
            gen_wat.push(format!("table.get {}", FUNCREF_TABLE));
            gen_wat.extend(save_funcref(offset));
            gen_wat.push("drop".into());
            gen_wat.extend(increment_mem_pointer(offset));
            gen_wat.push(l.into());
            gen_wat.extend(trace_u8(0xFE, offset));
            gen_wat.push(format!("local.get {}", LOCAL_FUNCREF));
            gen_wat.extend(save_funcref(offset));
            gen_wat.push("drop".into());
            gen_wat.extend(trace_u32(called_type.idx, offset));
            gen_wat.extend(trace_stack_value(called_type.results.get(0), offset));
            gen_wat.extend(increment_mem_pointer(offset));
        } else if l.contains("call") {
            let called_func_idx = get_func_idx_by_call_instr(&l)?;
            let called_type = types_by_functions.get(&called_func_idx).unwrap();
            gen_wat.extend(trace_u8(0x10, offset));
            gen_wat.extend(trace_u32(called_func_idx, offset));
            gen_wat.extend(increment_mem_pointer(offset));
            gen_wat.push(l.into());
            gen_wat.extend(trace_u8(0xFF, offset));
            gen_wat.extend(trace_u32(called_func_idx, offset));
            gen_wat.extend(trace_u32(called_type.idx, offset));
            gen_wat.extend(trace_stack_value(called_type.results.get(0), offset));
            gen_wat.extend(increment_mem_pointer(offset));
        } else if l.contains(".load") {
            let (code, typ) = get_load_info(&l)?;
            gen_wat.extend(trace_u8(code, offset));
            gen_wat.extend(trace_stack_value(Some(&ValType::I32), offset));
            gen_wat.push(l.into());
            gen_wat.extend(trace_stack_value(Some(&typ), offset));
            gen_wat.extend(increment_mem_pointer(offset));
        } else if l.contains(".store") {
            let (code, typ) = get_store_info(&l)?;
            gen_wat.extend(trace_u8(code, offset));
            gen_wat.extend(trace_store_stack(&typ, offset));
            gen_wat.push(l.into());
            gen_wat.extend(increment_mem_pointer(offset));
        } else if l.contains("table.get") {
            gen_wat.extend(trace_u8(0x25, offset));
            gen_wat.extend(trace_stack_value(Some(&ValType::I32), offset));
            gen_wat.push(l.into());
            gen_wat.extend(save_funcref(offset));
            gen_wat.extend(increment_mem_pointer(offset));
        } else if l.contains("table.set") {
            gen_wat.extend(trace_u8(0x26, offset));
            gen_wat.extend(trace_store_stack(&ValType::Funcref, offset));
            gen_wat.push(l.into());
            gen_wat.extend(increment_mem_pointer(offset));
        } else if l.contains("global.get") {
            let global_idx = get_global_idx(&l)?;
            let typ = globals.get(global_idx as usize).unwrap();
            gen_wat.extend(trace_u8(0x23, offset));
            gen_wat.extend(trace_u8(typ.get_code(), offset));
            gen_wat.extend(trace_u32(global_idx, offset));
            gen_wat.push(l.into());
            gen_wat.extend(trace_stack_value(Some(typ), offset));
            gen_wat.extend(increment_mem_pointer(offset));
        } else if l.contains("global.set") {
            let global_idx = get_global_idx(&l)?;
            let typ = globals.get(global_idx as usize).unwrap();
            gen_wat.extend(trace_u8(0x24, offset));
            gen_wat.extend(trace_u8(typ.get_code(), offset));
            gen_wat.extend(trace_u32(global_idx, offset));
            gen_wat.extend(trace_stack_value(Some(typ), offset));
            gen_wat.push(l.into());
            gen_wat.extend(increment_mem_pointer(offset));
        } else if let None = orig_wat.peek() {
            l.pop();
            gen_wat.push(l);
            gen_wat.push(format!(
                "(memory {} (export \"{}\") {})",
                TRACE_MEM, TRACE_MEM_EXPORT_NAME, TRACE_MEM_PAGES
            ));
            gen_wat.push(format!(
                "(global {} (export \"{}\") (mut i32) (i32.const 0))",
                MEM_POINTER, MEM_POINTER_EXPORT_NAME
            ));
            gen_wat.push(format!(
                "(table {} (export \"{}\") {} funcref)",
                FUNCREF_TABLE, FUNCREF_TABLE_EXPORT_NAME, FUNCREF_TABLE_SIZE
            ));
            gen_wat.push(format!(
                "(global {} (export \"{}\") (mut i32) (i32.const 0))",
                TABLE_POINTER, TABLE_POINTER_EXPORT_NAME
            ));
            gen_wat.push(")".into())
        } else {
            gen_wat.push(l.into());
        }
        if is_new_section(orig_wat.peek()) && inside_func {
            trace_return(&mut gen_wat, &typ, local_count, offset);
            gen_wat.push(")".into());
            inside_func = false;
        }
    }
    let gen_wat: String = gen_wat.join("\n");
    let gen = gen_wat.clone();
    return Err(Box::leak(gen.into_boxed_str()));
    match wat::parse_str(gen_wat) {
        Ok(r) => match wasmparser::Validator::new().validate_all(&r) {
            Ok(_) => Ok(r),
            Err(_) => Err(Box::leak(gen.into_boxed_str())),
        },
        Err(_) => Err(Box::leak(gen.into_boxed_str())),
    }
}

fn trace_u8(value: u8, offset: &mut u32) -> Vec<String> {
    vec![
        format!("global.get {}", MEM_POINTER),
        format!("i32.const {}", value),
        store_value(&ValType::U8, offset),
    ]
}

fn trace_u32(value: u32, offset: &mut u32) -> Vec<String> {
    vec![
        format!("global.get {}", MEM_POINTER),
        format!("i32.const {}", value),
        store_value(&ValType::I32, offset),
    ]
}

fn trace_stack_value(typ: Option<&ValType>, offset: &mut u32) -> Vec<String> {
    match typ {
        Some(typ) => {
            let local = typ.to_local();
            vec![
                format!("local.tee {}", local),
                format!("global.get {}", MEM_POINTER),
                format!("local.get {}", local),
                store_value(typ, offset),
            ]
        }
        None => vec![],
    }
}

fn trace_store_stack(typ: &ValType, offset: &mut u32) -> Vec<String> {
    let local = typ.to_local();
    let mut instrs = vec![];
    match typ {
        ValType::Funcref => {
            instrs.extend(save_funcref(offset));
            instrs.push("drop".into());
        }
        _ => instrs.extend(vec![
            format!("local.set {}", local),
            format!("global.get {}", MEM_POINTER),
            store_value(&typ, offset),
        ]),
    }
    instrs.push(format!("local.tee {}", LOCAL_ADDR));
    instrs.push(format!("global.get {}", MEM_POINTER));
    instrs.push(format!("local.get {}", LOCAL_ADDR));
    instrs.push(store_value(&ValType::I32, offset));
    instrs.push(format!("local.get {}", local));
    instrs
}

fn save_funcref(offset: &mut u32) -> Vec<String> {
    vec![
        format!("local.tee {}", LOCAL_FUNCREF),
        format!("global.get {}", TABLE_POINTER),
        format!("local.get {}", LOCAL_FUNCREF),
        format!("table.set {}", FUNCREF_TABLE),
        format!("global.get {}", MEM_POINTER),
        format!("global.get {}", TABLE_POINTER),
        store_value(&ValType::I32, offset),
        format!("global.get {}", TABLE_POINTER),
        format!("i32.const 1"),
        format!("i32.add"),
        format!("global.set {}", TABLE_POINTER),
    ]
}

fn increment_mem_pointer(offset: &mut u32) -> Vec<String> {
    let instrs = vec![
        format!("global.get {}", MEM_POINTER),
        format!("i32.const {}", offset),
        format!("i32.add"),
        format!("global.set {}", MEM_POINTER),
    ];
    *offset = 0;
    instrs
}

fn is_new_section(wat: Option<&&str>) -> bool {
    match wat {
        Some(l) => {
            let l = l.trim();
            if l.starts_with("(func")
                || l.starts_with("(global")
                || l.starts_with("(memory")
                || l.starts_with("(table")
                || l.starts_with("(export")
            {
                true
            } else {
                false
            }
        }
        None => true,
    }
}

fn store_value(typ: &ValType, offset: &mut u32) -> String {
    match typ {
        ValType::U8 => {
            let instr = format!("i32.store8 {} offset={}", TRACE_MEM, offset);
            *offset += 1;
            instr
        }
        ValType::I32 => {
            let instr = format!("i32.store {} offset={}", TRACE_MEM, offset);
            *offset += 4;
            instr
        }
        ValType::I64 => {
            let instr = format!("i64.store {} offset={}", TRACE_MEM, offset);
            *offset += 8;
            instr
        }
        ValType::F32 => {
            let instr = format!("f32.store {} offset={}", TRACE_MEM, offset);
            *offset += 4;
            instr
        }
        ValType::F64 => {
            let instr = format!("f64.store {} offset={}", TRACE_MEM, offset);
            *offset += 8;
            instr
        }
        ValType::Funcref => panic!("You cannot store a funcref to the memory"),
    }
}

fn check_mem() -> Vec<String> {
    vec![
        format!(
            "i32.const {}",
            64000.0 * TRACE_MEM_PAGES as f32 * TRACE_MEM_MAX_FILL_PERCENT
        ),
        format!("global.get {}", MEM_POINTER),
        format!("i32.eq"),
        format!("(if (then call {}))", CHECK_MEM),
    ]
}

fn get_func_idx_by_call_instr(input: &str) -> Result<u32, &'static str> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() < 2 {
        return Err("Couldnt extract func idx from call instr");
    }

    match parts[1].parse::<u32>() {
        Ok(number) => Ok(number),
        Err(_) => Err("Couldnt extract func idx from call instr"),
    }
}

fn get_type_idx_by_func_def(input: &str) -> Result<u32, &'static str> {
    let start_pattern = "(type ";
    let start = input.find(start_pattern).ok_or("Type pattern not found")?;
    let number_start = start + start_pattern.len();
    let end = input[number_start..]
        .find(|c: char| c == ' ' || c == ')')
        .ok_or("Closing pattern not found")?
        + number_start;
    input[number_start..end]
        .parse::<u32>()
        .map_err(|_| "Failed to parse type index")
}

fn get_type_idx_by_func_import(input: &str) -> Result<u32, &'static str> {
    let start_pattern = "(type ";
    let start = input.find(start_pattern).ok_or("Type pattern not found")?;
    let number_start = start + start_pattern.len();
    let end = input[number_start..]
        .find(|c: char| c == ' ' || c == ')')
        .ok_or("Closing pattern not found")?
        + number_start;
    input[number_start..end]
        .parse::<u32>()
        .map_err(|_| "Failed to parse type index")
}

fn get_load_info(wat: &str) -> Result<(u8, ValType), &'static str> {
    if wat.starts_with("i32.load8") {
        Ok((0x2C, ValType::I32))
    } else if wat.starts_with("i32.load16") {
        Ok((0x2E, ValType::I32))
    } else if wat.starts_with("i32.load") {
        Ok((0x28, ValType::I32))
    } else if wat.starts_with("i64.load8") {
        Ok((0x30, ValType::I64))
    } else if wat.starts_with("i64.load16") {
        Ok((0x32, ValType::I64))
    } else if wat.starts_with("i64.load32") {
        Ok((0x34, ValType::I64))
    } else if wat.starts_with("i64.load") {
        Ok((0x29, ValType::I64))
    } else if wat.starts_with("f32.load") {
        Ok((0x2A, ValType::F32))
    } else if wat.starts_with("f64.load") {
        Ok((0x2B, ValType::F64))
    } else {
        Err("Getting load info failed")
    }
}

fn get_store_info(wat: &str) -> Result<(u8, ValType), &'static str> {
    if wat.starts_with("i32.store8") {
        Ok((0x3A, ValType::I32))
    } else if wat.starts_with("i32.store16") {
        Ok((0x3B, ValType::I32))
    } else if wat.starts_with("i32.store") {
        Ok((0x36, ValType::I32))
    } else if wat.starts_with("i64.store8") {
        Ok((0x3C, ValType::I64))
    } else if wat.starts_with("i64.store16") {
        Ok((0x3D, ValType::I64))
    } else if wat.starts_with("i64.store32") {
        Ok((0x3E, ValType::I64))
    } else if wat.starts_with("i64.store") {
        Ok((0x37, ValType::I64))
    } else if wat.starts_with("f32.store") {
        Ok((0x38, ValType::F32))
    } else if wat.starts_with("f64.store") {
        Ok((0x39, ValType::F64))
    } else {
        Err("Getting store info failed")
    }
}

fn get_global_idx(input: &str) -> Result<u32, &'static str> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    // Ensure there are enough parts in the string
    if parts.len() < 2 {
        return Err("String does not contain enough parts");
    }

    // The number is expected to be the last part of the split string
    parts
        .last()
        .unwrap()
        .parse::<u32>()
        .map_err(|_| "Failed to parse number")
}

#[derive(Clone)]
struct FuncType {
    idx: u32,
    params: Vec<ValType>,
    results: Vec<ValType>,
}
impl FuncType {
    fn new(wat: &str, idx: u32) -> Result<FuncType, &'static str> {
        if !wat.starts_with("(type") {
            return Err("Cannot parse type");
        }
        let mut params = Vec::new();
        let mut results = Vec::new();
        let mut current_target = &mut params;
        for token in wat.split_whitespace() {
            match token {
                "param" => current_target = &mut params,
                "result" => current_target = &mut results,
                "i32" => current_target.push(ValType::I32),
                "i64" => current_target.push(ValType::I64),
                "f32" => current_target.push(ValType::F32),
                "f64" => current_target.push(ValType::F64),
                _ => {}
            }
        }
        Ok(FuncType {
            idx,
            params,
            results,
        })
    }

    fn trace_params(&self, offset: &mut u32) -> Vec<String> {
        let mut local_id = -1;
        self.params
            .iter()
            .flat_map(|v| {
                local_id += 1;
                vec![
                    format!("global.get {}", MEM_POINTER),
                    format!("local.get {}", local_id),
                    store_value(v, offset),
                ]
            })
            .collect()
    }

    fn trace_results(&self, local_count: usize, offset: &mut u32) -> Vec<String> {
        let mut local_id = local_count - 1 + 6;
        self.results
            .iter()
            .flat_map(|v| {
                local_id += 1;
                vec![
                    format!("global.get {}", MEM_POINTER),
                    format!("local.get {}", local_id),
                    store_value(v, offset),
                ]
            })
            .collect()
    }

    fn locals_wat(&self) -> String {
        let mut wat = format!(
            "(local {} i32)(local {} i32)(local {} i64)(local {} f32)(local {} f64)(local {} funcref)",
            LOCAL_ADDR, LOCAL_I32, LOCAL_I64, LOCAL_F32, LOCAL_F64, LOCAL_FUNCREF
        );
        for r in &self.results {
            match r {
                ValType::U8 => unreachable!("This will not be contained in the results"),
                ValType::I32 => wat.push_str(" i32"),
                ValType::I64 => wat.push_str(" i64"),
                ValType::F32 => wat.push_str(" f32"),
                ValType::F64 => wat.push_str(" f64"),
                ValType::Funcref => unreachable!("This will not be contained in the results"),
            }
        }
        wat
    }
}

fn get_type_idx_by_call_indirect(wat: &str) -> Result<usize, &'static str> {
    let type_pattern = "(type ";
    let start = wat.find(type_pattern).ok_or("Pattern not found")?;
    let id_start = start + type_pattern.len();
    let remaining = &wat[id_start..];
    let end = remaining.find(')').ok_or("Closing parenthesis not found")?;
    remaining[..end]
        .trim()
        .parse::<usize>()
        .map_err(|_| "Failed to parse type ID")
}

fn trace_return(gen_wat: &mut Vec<String>, typ: &FuncType, local_count: usize, offset: &mut u32) {
    gen_wat.extend(trace_u8(0x0F, offset));
    gen_wat.extend(typ.trace_results(local_count, offset));
    gen_wat.extend(increment_mem_pointer(offset));
    gen_wat.extend(check_mem());
}

#[derive(Clone, Debug)]
enum ValType {
    U8,
    I32,
    I64,
    F32,
    F64,
    Funcref,
}

impl ValType {
    fn from_global(wat: &str) -> Result<ValType, &'static str> {
        let tokens: Vec<&str> = wat.split_whitespace().collect();
        if tokens.len() < 3 {
            return Err("String does not contain enough parts");
        }
        let type_token = if tokens[1].starts_with("(mut") {
            tokens.get(2).ok_or("Type not found")
        } else {
            tokens.get(1).ok_or("Type not found")
        };
        let typ = type_token.map(|&s| s.trim_matches(|c: char| c == '(' || c == ')'))?;
        ValType::from_type_str(typ)
    }

    fn from_type_str(typ: &str) -> Result<ValType, &'static str> {
        match typ {
            "i32" => Ok(ValType::I32),
            "i64" => Ok(ValType::I64),
            "f32" => Ok(ValType::F32),
            "f64" => Ok(ValType::F64),
            "funcref" => Ok(ValType::Funcref),
            _ => Err("Could not parse valtype"),
        }
    }

    fn to_local(&self) -> &'static str {
        match self {
            ValType::U8 => panic!("There is no local for U8"),
            ValType::I32 => LOCAL_I32,
            ValType::I64 => LOCAL_I64,
            ValType::F32 => LOCAL_F32,
            ValType::F64 => LOCAL_F64,
            ValType::Funcref => LOCAL_FUNCREF,
        }
    }

    fn get_code(&self) -> u8 {
        match self {
            ValType::I32 => 0,
            ValType::I64 => 1,
            ValType::F32 => 2,
            ValType::F64 => 3,
            ValType::Funcref => 4,
            ValType::U8 => 5,
        }
    }
}

// pub fn instrument_wasm(buffer: &[u8]) -> Result<Module> {
//     let mut module = Module::from_buffer(buffer)?;

//     // I create this data in order to keep track on which instructions I actually do need to instrument
//     let mut pub_memories: Vec<MemoryId> = Vec::new();
//     let mut pub_tables: Vec<TableId> = Vec::new();
//     let mut pub_globals: Vec<GlobalId> = Vec::new();
//     for i in module.imports.iter() {
//         match i.kind {
//             ImportKind::Table(tid) => pub_tables.push(tid),
//             ImportKind::Memory(mid) => pub_memories.push(mid),
//             // Global is handled by the trace.
//             ImportKind::Global(gid) => pub_globals.push(gid),
//             _ => {}
//         };
//     }
//     module.exports.iter().for_each(|e| {
//         match e.item {
//             walrus::ExportItem::Table(id) => pub_tables.push(id),
//             walrus::ExportItem::Memory(id) => pub_memories.push(id),
//             walrus::ExportItem::Global(id) => pub_globals.push(id),
//             _ => {}
//         };
//     });
//     pub_globals = pub_globals
//         .into_iter()
//         .filter(|g| module.globals.get(*g).mutable)
//         .collect();

//     let trace_mem_id = module.memories.add_local(false, 10_000, None); // around 2 GB
//     module.exports.add("trace", trace_mem_id);
//     let mem_pointer = module.globals.add_local(
//         walrus::ValType::I32,
//         true,
//         walrus::InitExpr::Value(walrus::ir::Value::I32(0)),
//     );
//     module.exports.add("trace_byte_length", mem_pointer);
//     let lookup_table_id = module.tables.add_local(100_000, None, ValType::Funcref);
//     module.exports.add("lookup", lookup_table_id);
//     let lookup_pointer = module.globals.add_local(
//         walrus::ValType::I32,
//         true,
//         walrus::InitExpr::Value(walrus::ir::Value::I32(0)),
//     );
//     module.exports.add("lookup_table_pointer", lookup_pointer);
//     let (locals, added_locals) = add_locals(&mut module);
//     let funcref_local = added_locals
//         .get(&ValType::Funcref)
//         .unwrap()
//         .get(0)
//         .unwrap()
//         .clone();
//     let module_types = Types::new(&module);
//     // Add return instruction at the end of each function (Important for Instrumentation)
//     module.funcs.iter_local_mut().for_each(|(_, f)| {
//         f.builder_mut().func_body().return_();
//     });
//     // Mem check imported function
//     let typ = module.types.find(&[], &[]);
//     let typ = match typ {
//         Some(t) => t,
//         None => module.types.add(&[], &[]),
//     };
//     let (check_mem_id, _) = module.add_import_func("r3", "check_mem", typ);
//     // Mem check local function
//     let mut builder = FunctionBuilder::new(&mut module.types, &[], &[]);
//     builder
//         .func_body()
//         .const_(Value::I32(1000 * 64000))
//         .global_get(mem_pointer)
//         .binop(BinaryOp::I32LeU)
//         .if_else(
//             None,
//             |then| {
//                 then.call(check_mem_id)
//                     .const_(Value::I32(0))
//                     .global_set(mem_pointer);
//             },
//             |_| {},
//         );
//     let check_mem_id_local = builder.finish(vec![], &mut module.funcs);

//     let first_function = module.functions().find(|_| true);
//     let first_function = match first_function {
//         Some(f) => f.ty(),
//         None => return Ok(module),
//     };
//     let current_type = module.types.get(first_function).clone();

//     let mut generator = Generator::new(
//         trace_mem_id,
//         mem_pointer,
//         lookup_table_id,
//         lookup_pointer,
//         locals,
//         added_locals,
//         funcref_local,
//         module_types,
//         current_type,
//         check_mem_id,
//         check_mem_id_local,
//         pub_memories,
//         pub_tables,
//         pub_globals,
//     );
//     // Instrument everything exept function entry
//     module.funcs.iter_mut().for_each(|f| {
//         if f.id() == check_mem_id_local {
//             return;
//         }
//         if let FunctionKind::Local(f) = &mut f.kind {
//             generator.set_current_func_type(module.types.get(f.ty()).clone());
//             generator.set_func_entry(true);
//             ir::dfs_pre_order_mut(&mut generator, f, f.entry_block())
//         }
//     });
//     // Instrument function entry
//     module.funcs.iter_mut().for_each(|f| {
//         let fidx = f.id();
//         if fidx == check_mem_id_local {
//             return;
//         }
//         let offset: &mut u32 = &mut 0;
//         if let FunctionKind::Local(f) = &mut f.kind {
//             let mut binding = f.builder_mut().func_body();
//             let body = binding.instrs_mut();
//             let mut gen_seq: Vec<Instruction> = vec![];
//             let opcode = 0x02;
//             let typ = generator.current_func_type.clone();
//             let params = typ.params();
//             gen_seq.append(
//                 &mut InstructionsEnum::from_vec(vec![
//                     generator.trace_code(opcode, offset),
//                     generator.trace_func_idx(fidx, offset),
//                     generator.trace_type(&typ, offset),
//                     generator.save_params(params, offset),
//                     generator.increment_mem_pointer(offset),
//                 ])
//                 .flatten(),
//             );
//             for instr in gen_seq.iter().rev() {
//                 body.insert(0, instr.clone())
//             }
//         }
//     });

//     // dbg!(&module);
//     Ok(module)
// }

// #[derive(Debug)]
// struct Locals(HashMap<ValType, Vec<LocalId>>);
// impl Locals {
//     fn insert(&mut self, mut typ: ValType, locals: Vec<LocalId>) {
//         // if let ValType::Funcref = typ {
//         //     typ = ValType::I32
//         // }
//         self.0.insert(typ, locals);
//     }

//     fn entry(
//         &mut self,
//         mut typ: ValType,
//     ) -> std::collections::hash_map::Entry<'_, ValType, Vec<LocalId>> {
//         // if let ValType::Funcref = typ {
//         //     typ = ValType::I32
//         // }
//         self.0.entry(typ)
//     }

//     fn get(&self, mut typ: &ValType) -> Option<&Vec<LocalId>> {
//         // if let ValType::Funcref = typ {
//         //     typ = &ValType::I32
//         // }
//         self.0.get(typ)
//     }
// }
// fn add_locals(module: &mut Module) -> (Locals, Locals) {
//     let mut locals = Locals(HashMap::new());
//     module.locals.iter().for_each(|l| {
//         let _ = locals
//             .entry(l.ty())
//             .and_modify(|e: &mut Vec<LocalId>| {
//                 e.push(l.id());
//             })
//             .or_insert(vec![l.id()]);
//     });
//     let mut added_locals: Locals = Locals(HashMap::new());
//     added_locals.insert(ValType::Funcref, vec![module.locals.add(ValType::Funcref)]);
//     added_locals.insert(ValType::I32, vec![module.locals.add(ValType::I32)]);
//     added_locals.insert(ValType::I64, vec![module.locals.add(ValType::I64)]);
//     added_locals.insert(ValType::F32, vec![module.locals.add(ValType::F32)]);
//     added_locals.insert(ValType::F64, vec![module.locals.add(ValType::F64)]);
//     module.types.iter().for_each(|t| {
//         let params = t.params();
//         let mut amounts: HashMap<ValType, usize> = HashMap::new();
//         params.iter().for_each(|t| {
//             let _ = amounts.entry(*t).and_modify(|e| *e += 1).or_insert(1);
//         });
//         amounts.iter().for_each(|(t, a)| {
//             added_locals.0.entry(*t).and_modify(|e| {
//                 if e.len() < *a {
//                     let mut vec = Vec::with_capacity(*a);
//                     for _ in 0..*a {
//                         vec.push(module.locals.add(*t));
//                     }
//                     *e = vec;
//                 }
//             });
//         });
//         let results = t.results();
//         results.iter().for_each(|t| {
//             let _ = amounts.entry(*t).and_modify(|e| *e += 1).or_insert(1);
//         });
//         amounts.iter().for_each(|(t, a)| {
//             added_locals.entry(*t).and_modify(|e| {
//                 if e.len() < *a {
//                     let mut vec = Vec::with_capacity(*a);
//                     for _ in 0..*a {
//                         vec.push(module.locals.add(*t));
//                     }
//                     *e = vec;
//                 }
//             });
//         });
//     });
//     (locals, added_locals)
// }

// #[derive(Debug)]
// struct Types {
//     by_func: HashMap<FunctionId, Type>,
//     by_id: HashMap<TypeId, Type>,
//     global_types: HashMap<GlobalId, ValType>,
//     element_types: HashMap<TableId, ValType>,
// }

// impl Types {
//     fn new(module: &Module) -> Types {
//         let by_func = module
//             .functions()
//             .map(|f| (f.id(), module.types.get(f.ty()).clone()))
//             .collect();
//         let by_id = module
//             .functions()
//             .map(|f| (f.ty(), module.types.get(f.ty()).clone()))
//             .collect();
//         let global_types = module.globals.iter().map(|g| (g.id(), g.ty)).collect();
//         let element_types = module
//             .tables
//             .iter()
//             .map(|t| (t.id(), t.element_ty))
//             .collect();
//         Self {
//             by_func,
//             by_id,
//             global_types,
//             element_types,
//         }
//     }

//     fn get_by_func(&self, func: &FunctionId) -> Option<&Type> {
//         self.by_func.get(func)
//     }

//     fn get_by_id(&self, id: &TypeId) -> Option<&Type> {
//         self.by_id.get(id)
//     }

//     fn get_global_type(&self, id: &GlobalId) -> Option<&ValType> {
//         self.global_types.get(id)
//     }

//     fn get_element_type(&self, id: &TableId) -> Option<&ValType> {
//         self.element_types.get(id)
//     }
// }

// enum InstructionsEnum {
//     Sequence(Vec<Instruction>),
//     Single(Instruction),
// }

// impl InstructionsEnum {
//     pub fn from_vec(vec: Vec<InstructionsEnum>) -> Self {
//         Self::Sequence(
//             vec.into_iter()
//                 .map(|e| match e {
//                     InstructionsEnum::Sequence(s) => s,
//                     InstructionsEnum::Single(s) => vec![s],
//                 })
//                 .flat_map(|s| s.into_iter())
//                 .collect(),
//         )
//     }

//     pub fn flatten(&self) -> Vec<Instruction> {
//         match self {
//             InstructionsEnum::Sequence(s) => s.to_vec(),
//             InstructionsEnum::Single(s) => vec![s.clone()],
//         }
//     }
// }

// #[derive(Debug)]
// struct Generator {
//     trace_mem_id: MemoryId,
//     mem_pointer: GlobalId,
//     lookup_table_id: TableId,
//     lookup_pointer: GlobalId,
//     locals: Locals,
//     added_locals: Locals,
//     funcref_local: LocalId,
//     module_types: Types,
//     current_func_type: Type,
//     func_entry: bool,
//     check_mem_id: FunctionId,
//     check_mem_id_local: FunctionId,
//     pub_memories: Vec<MemoryId>,
//     pub_tables: Vec<TableId>,
//     pub_globals: Vec<GlobalId>,
// }

// impl VisitorMut for Generator {
//     fn start_instr_seq_mut(&mut self, seq: &mut ir::InstrSeq) {
//         let mut added_instr_count = 0;
//         let mut instrumentation_code = Vec::new();
//         seq.clone().iter().enumerate().for_each(|(i, (instr, _))| {
//             let mut gen_seq: Vec<Instruction> = vec![];
//             let offset: &mut u32 = &mut 0;
//             match instr {
//                 Instr::Load(m) => {
//                     if self.pub_memories.contains(&m.memory) {
//                         let (opcode, local_type) = match m.kind {
//                             ir::LoadKind::I32 { .. } => (0x28, ValType::I32),
//                             ir::LoadKind::I64 { .. } => (0x29, ValType::I64),
//                             ir::LoadKind::F32 => (0x2A, ValType::F32),
//                             ir::LoadKind::F64 => (0x2B, ValType::F64),
//                             ir::LoadKind::V128 => todo!(),
//                             ir::LoadKind::I32_8 { .. } => (0x2C, ValType::I32),
//                             ir::LoadKind::I32_16 { .. } => (0x2E, ValType::I32),
//                             ir::LoadKind::I64_8 { .. } => (0x30, ValType::I64),
//                             ir::LoadKind::I64_16 { .. } => (0x32, ValType::I64),
//                             ir::LoadKind::I64_32 { .. } => (0x34, ValType::I64),
//                         };
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![
//                                 self.trace_code(opcode, offset),
//                                 self.save_stack(&[ValType::I32], offset),
//                                 self.instr(instr.clone()),
//                                 self.save_stack(&[local_type], offset),
//                                 self.increment_mem_pointer(offset),
//                             ])
//                             .flatten(),
//                         );
//                     } else {
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![self.instr(instr.clone())])
//                                 .flatten(),
//                         );
//                     }
//                 }
//                 Instr::Store(m) => {
//                     if self.pub_memories.contains(&m.memory) {
//                         let (opcode, local_type) = match m.kind {
//                             ir::StoreKind::I32 { .. } => (0x36, ValType::I32),
//                             ir::StoreKind::I64 { .. } => (0x37, ValType::I64),
//                             ir::StoreKind::F32 => (0x38, ValType::F32),
//                             ir::StoreKind::F64 => (0x39, ValType::F64),
//                             ir::StoreKind::V128 => todo!(),
//                             ir::StoreKind::I32_8 { .. } => (0x3A, ValType::I32),
//                             ir::StoreKind::I32_16 { .. } => (0x3B, ValType::I32),
//                             ir::StoreKind::I64_8 { .. } => (0x3C, ValType::I64),
//                             ir::StoreKind::I64_16 { .. } => (0x3D, ValType::I64),
//                             ir::StoreKind::I64_32 { .. } => (0x3E, ValType::I64),
//                         };
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![
//                                 self.trace_code(opcode, offset),
//                                 self.save_stack(&[ValType::I32, local_type], offset),
//                                 self.instr(instr.clone()),
//                                 self.increment_mem_pointer(offset),
//                             ])
//                             .flatten(),
//                         );
//                     } else {
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![self.instr(instr.clone())])
//                                 .flatten(),
//                         );
//                     }
//                 }
//                 Instr::MemoryGrow(m) => {
//                     if self.pub_memories.contains(&m.memory) {
//                         let opcode = 0x40;
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![
//                                 self.trace_code(opcode, offset),
//                                 self.save_stack(&[ValType::I32], offset),
//                                 self.instr(instr.clone()),
//                                 self.increment_mem_pointer(offset),
//                             ])
//                             .flatten(),
//                         );
//                     } else {
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![self.instr(instr.clone())])
//                                 .flatten(),
//                         );
//                     }
//                 }
//                 Instr::Call(call) => {
//                     let opcode = 0x10;
//                     let return_code = 0xFF;
//                     let typ = self.module_types.get_by_func(&call.func).unwrap().clone();
//                     gen_seq.append(
//                         &mut InstructionsEnum::from_vec(vec![
//                             self.trace_code(opcode, offset),
//                             self.trace_func_idx(call.func, offset),
//                             self.increment_mem_pointer(offset),
//                             self.instr(instr.clone()),
//                             self.trace_code(return_code, offset),
//                             self.trace_func_idx(call.func, offset),
//                             self.trace_type(&typ, offset),
//                             self.save_stack(typ.results(), offset),
//                             self.increment_mem_pointer(offset),
//                         ])
//                         .flatten(),
//                     );
//                 }
//                 Instr::GlobalGet(get) => {
//                     if self.pub_globals.contains(&get.global) {
//                         let opcode = 0x23;
//                         let typ = self
//                             .module_types
//                             .get_global_type(&get.global)
//                             .unwrap()
//                             .clone();
//                         let typ_code = get_typ_code(&typ);
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![
//                                 self.trace_code(opcode, offset),
//                                 self.trace_code(typ_code, offset),
//                                 self.global_get(self.mem_pointer),
//                                 self.get_const(Value::I32(get.global.index() as i32)),
//                                 self.store_val_to_trace(ValType::I32, offset),
//                                 self.instr(instr.clone()),
//                                 self.save_stack(&[typ], offset),
//                                 self.increment_mem_pointer(offset),
//                             ])
//                             .flatten(),
//                         );
//                     } else {
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![self.instr(instr.clone())])
//                                 .flatten(),
//                         );
//                     }
//                 }
//                 Instr::GlobalSet(set) => {
//                     if self.pub_globals.contains(&set.global) {
//                         let opcode = 0x24;
//                         let typ = self
//                             .module_types
//                             .get_global_type(&set.global)
//                             .unwrap()
//                             .clone();
//                         let typ_code = get_typ_code(&typ);
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![
//                                 self.trace_code(opcode, offset),
//                                 self.trace_code(typ_code, offset),
//                                 self.global_get(self.mem_pointer),
//                                 self.get_const(Value::I32(set.global.index() as i32)),
//                                 self.store_val_to_trace(ValType::I32, offset),
//                                 self.save_stack(&[typ], offset),
//                                 self.instr(instr.clone()),
//                                 self.increment_mem_pointer(offset),
//                             ])
//                             .flatten(),
//                         );
//                     } else {
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![self.instr(instr.clone())])
//                                 .flatten(),
//                         );
//                     }
//                 }
//                 Instr::TableSet(set) => {
//                     if self.pub_tables.contains(&set.table) {
//                         let opcode = 0x26;
//                         let typ = self
//                             .module_types
//                             .get_element_type(&set.table)
//                             .unwrap()
//                             .clone();
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![
//                                 self.trace_code(opcode, offset),
//                                 self.save_stack(&[ValType::I32, typ], offset),
//                                 self.instr(instr.clone()),
//                                 self.increment_mem_pointer(offset),
//                             ])
//                             .flatten(),
//                         );
//                     } else {
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![self.instr(instr.clone())])
//                                 .flatten(),
//                         );
//                     }
//                 }
//                 Instr::TableGet(set) => {
//                     if self.pub_tables.contains(&set.table) {
//                         let opcode = 0x25;
//                         let typ = self
//                             .module_types
//                             .get_element_type(&set.table)
//                             .unwrap()
//                             .clone();
//                         match typ {
//                             ValType::I32 => todo!(),
//                             ValType::I64 => todo!(),
//                             ValType::F32 => todo!(),
//                             ValType::F64 => todo!(),
//                             ValType::V128 => todo!(),
//                             ValType::Externref => todo!(),
//                             ValType::Funcref => {}
//                         };
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![
//                                 self.trace_code(opcode, offset),
//                                 self.save_stack(&[ValType::I32], offset),
//                                 self.instr(instr.clone()),
//                                 self.save_funcref(offset),
//                                 self.increment_mem_pointer(offset),
//                             ])
//                             .flatten(),
//                         );
//                     } else {
//                         gen_seq.append(
//                             &mut InstructionsEnum::from_vec(vec![self.instr(instr.clone())])
//                                 .flatten(),
//                         );
//                     }
//                 }
//                 Instr::CallIndirect(call) => {
//                     let call_code = 0x11;
//                     let return_code = 0xFE;
//                     let typ = self.module_types.get_by_id(&call.ty).unwrap().clone();
//                     let local = self
//                         .added_locals
//                         .get(&ValType::I32)
//                         .unwrap()
//                         .get(0)
//                         .unwrap()
//                         .clone();
//                     gen_seq.append(
//                         &mut InstructionsEnum::from_vec(vec![
//                             self.trace_code(call_code, offset),
//                             self.local_tee(local),
//                             self.save_stack(&[ValType::I32], offset),
//                             self.local_get(local),
//                             self.table_get(call.table),
//                             self.save_funcref(offset),
//                             self.drop(),
//                             self.increment_mem_pointer(offset),
//                             self.instr(instr.clone()),
//                             self.trace_code(return_code, offset),
//                             self.local_get(self.funcref_local),
//                             self.save_funcref(offset),
//                             self.drop(),
//                             self.trace_type(&typ, offset),
//                             self.save_stack(typ.results(), offset),
//                             self.increment_mem_pointer(offset),
//                         ])
//                         .flatten(),
//                     );
//                 }
//                 // Instr::TableGrow(_) => todo!(),
//                 // Instr::TableFill(_) => todo!(),
//                 // Instr::LoadSimd(_) => todo!(),
//                 // Instr::TableInit(_) => todo!(),
//                 // Instr::ElemDrop(_) => todo!(),
//                 // Instr::TableCopy(_) => todo!(),
//                 Instr::Return(_) => {
//                     let opcode = 0x0F;
//                     let c = self.current_func_type.clone();
//                     let returns = c.results();
//                     gen_seq.append(
//                         &mut InstructionsEnum::from_vec(vec![
//                             self.trace_code(opcode, offset),
//                             // self.save_stack(returns, offset),
//                             self.increment_mem_pointer(offset),
//                             self.call(self.check_mem_id_local),
//                             self.instr(instr.clone()),
//                         ])
//                         .flatten(),
//                     );
//                 }
//                 // Instr::MemoryInit(_) => todo!(),
//                 // Instr::DataDrop(_) => todo!(),
//                 // Instr::MemoryCopy(_) => todo!(),
//                 // Instr::MemoryFill(_) => todo!(),
//                 _ => return,
//             };
//             let gen_length = gen_seq.len() - 1;
//             instrumentation_code.push((i + added_instr_count, gen_seq));
//             added_instr_count += gen_length;
//         });
//         instrumentation_code.iter().for_each(|(i, gen_seq)| {
//             seq.splice(i.clone()..(i.clone() + 1), gen_seq.clone());
//         })
//     }
// }

// impl Generator {
//     fn new(
//         trace_mem_id: MemoryId,
//         mem_pointer: GlobalId,
//         lookup_table_id: TableId,
//         lookup_pointer: GlobalId,
//         locals: Locals,
//         added_locals: Locals,
//         funcref_local: LocalId,
//         module_types: Types,
//         current_func_type: Type,
//         check_mem_id: FunctionId,
//         check_mem_id_local: FunctionId,
//         pub_memories: Vec<MemoryId>,
//         pub_tables: Vec<TableId>,
//         pub_globals: Vec<GlobalId>,
//     ) -> Self {
//         Self {
//             trace_mem_id,
//             mem_pointer,
//             lookup_table_id,
//             lookup_pointer,
//             locals,
//             added_locals,
//             funcref_local,
//             module_types,
//             current_func_type,
//             func_entry: true,
//             check_mem_id,
//             check_mem_id_local,
//             pub_globals,
//             pub_tables,
//             pub_memories,
//         }
//     }

//     fn trace_func_idx(&mut self, idx: FunctionId, offset: &mut u32) -> InstructionsEnum {
//         InstructionsEnum::from_vec(vec![
//             self.global_get(self.mem_pointer),
//             self.get_const(ir::Value::I32(idx.index() as i32)),
//             self.store_val_to_trace(ValType::I32, offset),
//         ])
//     }

//     fn trace_type(&mut self, typ: &Type, offset: &mut u32) -> InstructionsEnum {
//         InstructionsEnum::from_vec(vec![
//             self.global_get(self.mem_pointer),
//             self.get_const(ir::Value::I32(typ.id().index() as i32)),
//             self.store_val_to_trace(ValType::I32, offset),
//         ])
//     }

//     fn check_mem(&self) -> InstructionsEnum {
//         InstructionsEnum::from_vec(vec![
//             self.get_const(ir::Value::I32(64000 * 20000)),
//             self.global_get(self.mem_pointer),
//             self.binop(BinaryOp::I32Eq),
//             // self.check_mem_and_call(),
//         ])
//     }

//     fn call(&self, func: FunctionId) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::Call(Call { func }), InstrLocId::default()))
//     }

//     fn save_params(&mut self, values: &[ValType], offset: &mut u32) -> InstructionsEnum {
//         let mut locals = Vec::new();
//         InstructionsEnum::from_vec(
//             values
//                 .iter()
//                 .map(|t| {
//                     let local = *self.locals.get(t).unwrap().get(0).unwrap();
//                     locals.push(local);
//                     self.locals.entry(*t).and_modify(|e| {
//                         let id = e.remove(0);
//                         e.push(id);
//                     });
//                     let instrs = InstructionsEnum::from_vec(vec![
//                         self.global_get(self.mem_pointer),
//                         self.local_get(local),
//                         self.store_val_to_trace(*t, offset),
//                     ]);
//                     instrs
//                 })
//                 .collect(),
//         )
//     }

//     fn save_funcref(&mut self, offset: &mut u32) -> InstructionsEnum {
//         InstructionsEnum::from_vec(vec![
//             // save funcref to lookup table
//             self.local_set(self.funcref_local),
//             self.global_get(self.lookup_pointer),
//             self.local_get(self.funcref_local),
//             self.table_set(self.lookup_table_id),
//             // store lookup pointer to trace
//             self.global_get(self.mem_pointer),
//             self.global_get(self.lookup_pointer),
//             self.store_val_to_trace(ValType::I32, offset),
//             // increment lookup pointer
//             self.global_get(self.lookup_pointer),
//             self.get_const(Value::I32(1)),
//             self.binop(BinaryOp::I32Add),
//             self.global_set(self.lookup_pointer),
//             // push funcref back on trace
//             self.local_get(self.funcref_local),
//         ])
//     }

//     fn save_stack(&mut self, values: &[ValType], offset: &mut u32) -> InstructionsEnum {
//         let mut locals = Vec::new();
//         InstructionsEnum::from_vec(vec![
//             InstructionsEnum::from_vec(
//                 values
//                     .iter()
//                     .rev()
//                     .map(|t| {
//                         let local = *self.added_locals.get(t).unwrap().get(0).unwrap();
//                         locals.push(local);
//                         self.added_locals.entry(*t).and_modify(|e| {
//                             let id = e.remove(0);
//                             e.push(id);
//                         });
//                         InstructionsEnum::from_vec(vec![
//                             self.local_set(local),
//                             self.global_get(self.mem_pointer),
//                             self.local_get(local),
//                             self.store_val_to_trace(*t, offset),
//                         ])
//                     })
//                     .collect(),
//             ),
//             InstructionsEnum::from_vec(locals.iter().rev().map(|l| self.local_get(*l)).collect()),
//         ])
//     }

//     fn trace_code(&self, code: u8, offset: &mut u32) -> InstructionsEnum {
//         InstructionsEnum::from_vec(vec![
//             self.global_get(self.mem_pointer),
//             self.get_const(Value::I32(code as i32)),
//             self.store_to_trace(StoreKind::I32_8 { atomic: false }, offset),
//         ])
//     }

//     fn store_val_to_trace(&self, val_type: ValType, offset: &mut u32) -> InstructionsEnum {
//         let kind = match val_type {
//             ValType::I32 => StoreKind::I32 { atomic: false },
//             ValType::I64 => StoreKind::I64 { atomic: false },
//             ValType::F32 => StoreKind::F32,
//             ValType::F64 => StoreKind::F64,
//             ValType::V128 => todo!(),
//             ValType::Externref => StoreKind::I32 { atomic: false },
//             ValType::Funcref => StoreKind::I32 { atomic: false },
//         };
//         self.store_to_trace(kind, offset)
//     }

//     // fn double_drop(&self) -> InstructionsEnum {
//     //     InstructionsEnum::from_vec(vec![self.drop(), self.drop()])
//     // }

//     // fn drop(&self) -> InstructionsEnum {
//     //     InstructionsEnum::Single((Instr::Drop(Drop {}), InstrLocId::default()))
//     // }

//     fn store_to_trace(&self, kind: StoreKind, offset: &mut u32) -> InstructionsEnum {
//         let align = match kind {
//             StoreKind::I32 { .. } => 4,
//             StoreKind::I64 { .. } => 8,
//             StoreKind::F32 => 4,
//             StoreKind::F64 => 8,
//             StoreKind::V128 => todo!(),
//             StoreKind::I32_8 { .. } => 1,
//             StoreKind::I32_16 { .. } => 2,
//             StoreKind::I64_8 { .. } => 1,
//             StoreKind::I64_16 { .. } => 2,
//             StoreKind::I64_32 { .. } => 4,
//         };
//         let instr = InstructionsEnum::Single((
//             Instr::Store(Store {
//                 memory: self.trace_mem_id,
//                 kind,
//                 arg: MemArg {
//                     align,
//                     offset: *offset,
//                 },
//             }),
//             InstrLocId::default(),
//         ));
//         match kind {
//             StoreKind::I32 { .. } => *offset += 4,
//             StoreKind::I64 { .. } => *offset += 8,
//             StoreKind::F32 => *offset += 4,
//             StoreKind::F64 => *offset += 8,
//             StoreKind::V128 => todo!(),
//             StoreKind::I32_8 { .. } => *offset += 1,
//             StoreKind::I32_16 { .. } => *offset += 2,
//             StoreKind::I64_8 { .. } => *offset += 1,
//             StoreKind::I64_16 { .. } => *offset += 2,
//             StoreKind::I64_32 { .. } => *offset += 4,
//         };
//         instr
//         // self.double_drop()
//     }

//     fn instr(&self, instr: Instr) -> InstructionsEnum {
//         InstructionsEnum::Single((instr, InstrLocId::default()))
//     }

//     fn get_const(&self, value: Value) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::Const(Const { value }), InstrLocId::default()))
//     }

//     fn global_get(&self, global: GlobalId) -> InstructionsEnum {
//         InstructionsEnum::Single((
//             Instr::GlobalGet(GlobalGet { global }),
//             InstrLocId::default(),
//         ))
//     }

//     fn global_set(&self, global: GlobalId) -> InstructionsEnum {
//         InstructionsEnum::Single((
//             Instr::GlobalSet(GlobalSet { global }),
//             InstrLocId::default(),
//         ))
//     }

//     fn call_check_mem(&self) -> InstructionsEnum {
//         InstructionsEnum::Single((
//             Instr::Call(Call {
//                 func: self.check_mem_id,
//             }),
//             InstrLocId::default(),
//         ))
//     }

//     fn local_tee(&self, local: LocalId) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::LocalTee(LocalTee { local }), InstrLocId::default()))
//     }

//     fn local_get(&self, local: LocalId) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::LocalGet(LocalGet { local }), InstrLocId::default()))
//     }

//     fn local_set(&self, local: LocalId) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::LocalSet(LocalSet { local }), InstrLocId::default()))
//     }

//     fn table_get(&self, table: TableId) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::TableGet(TableGet { table }), InstrLocId::default()))
//     }

//     fn table_set(&self, table: TableId) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::TableSet(TableSet { table }), InstrLocId::default()))
//     }

//     fn increment_mem_pointer(&self, amount: &mut u32) -> InstructionsEnum {
//         let instrs = InstructionsEnum::from_vec(vec![
//             self.global_get(self.mem_pointer),
//             self.get_const(Value::I32(*amount as i32)),
//             self.binop(ir::BinaryOp::I32Add),
//             self.global_set(self.mem_pointer),
//         ]);
//         *amount = 0;
//         instrs
//     }

//     fn binop(&self, op: BinaryOp) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::Binop(Binop { op }), InstrLocId::default()))
//     }

//     fn drop(&self) -> InstructionsEnum {
//         InstructionsEnum::Single((Instr::Drop(Drop {}), InstrLocId::default()))
//     }

//     // fn check_mem_and_call(&self) -> InstructionsEnum {
//     //     // let mut module = Module::with_config(ModuleConfig::new());
//     //     // let mut builder = FunctionBuilder::new(&mut module.types, &[], &[]);
//     //     // let id = builder
//     //     //     .dangling_instr_seq(None)
//     //     //     .call(self.check_mem_id)
//     //     //     .const_(Value::I32(0))
//     //     //     .global_set(self.mem_pointer)
//     //     //     .id();
//     //     // let empty_id = builder.dangling_instr_seq(None).id();
//     //     // InstructionsEnum::Single((
//     //     //     Instr::IfElse(IfElse {
//     //     //         consequent: id,
//     //     //         alternative: empty_id,
//     //     //     }),
//     //     //     InstrLocId::default(),
//     //     // ))
//     //     InstructionsEnum::from_vec(vec![self.branch_if()])
//     // }

//     // fn branch_if() -> InstructionsEnum {
//     //     // InstructionsEnum::Single((Instr::BrIf(BrIf { block }), InstrLocId::default()))
//     //     todo!()
//     // }

//     fn set_current_func_type(&mut self, typ: Type) {
//         self.current_func_type = typ;
//     }

//     fn set_func_entry(&mut self, entry: bool) {
//         self.func_entry = entry;
//     }
// }

// fn get_byte_length(valtype: &ValType) -> i32 {
//     match valtype {
//         ValType::I32 => 4,
//         ValType::I64 => 8,
//         ValType::F32 => 4,
//         ValType::F64 => 8,
//         ValType::V128 => todo!(),
//         ValType::Externref => todo!(),
//         ValType::Funcref => todo!(),
//     }
// }

// fn get_typ_code(valtype: &ValType) -> u8 {
//     match valtype {
//         ValType::I32 => 0,
//         ValType::I64 => 1,
//         ValType::F32 => 2,
//         ValType::F64 => 3,
//         ValType::V128 => todo!(),
//         ValType::Externref => todo!(),
//         ValType::Funcref => todo!(),
//     }
// }
