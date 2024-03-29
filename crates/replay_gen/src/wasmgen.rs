use std::collections::{BTreeMap, HashSet};
use std::io::Write;
use std::process::Command;
use std::{fs, vec};
use std::{fs::File, path::Path};

use walrus::Module;

use crate::irgen::{FunctionTy, HostEvent, INIT_INDEX};
use crate::trace::{ValType, F64};
use crate::{irgen::Replay, write};

pub fn generate_replay_wasm(replay_path: &Path, code: &Replay) -> std::io::Result<()> {
    let mut module_set: HashSet<&String> = code
        .module
        .imports
        .iter()
        .map(|import| &import.module)
        .collect();
    let binding = "main".to_string();
    module_set.insert(&binding);
    for current_module in module_set.clone() {
        let module_wat_path = replay_path
            .parent()
            .unwrap()
            .join(&format!("{current_module}.wat"));
        let mut module_wat_file = File::create(&module_wat_path)?;
        let stream = &mut module_wat_file;
        write(stream, "(module\n")?;

        let mut data_segments: Vec<Vec<F64>> = vec![];

        // Import part
        for (_memidx, memory) in &code.exported_mems() {
            let name = memory.export.clone().unwrap().name.clone();
            let initial = memory.initial;
            let maximum = match memory.maximum {
                Some(max) => max.to_string(),
                None => "".to_string(),
            };
            write(
                stream,
                &format!("(import \"index\" \"{name}\" (memory {initial} {maximum}))\n",),
            )?;
        }

        for (_globalidx, global) in &code.exported_globals() {
            let name = global.export.clone().unwrap().name.clone();
            let valtype = global.valtype.clone();
            let typedecl = match global.mutable {
                true => format!("(mut {valtype})"),
                false => format!("{valtype}"),
            };

            write(
                stream,
                &format!("(import \"index\" \"{name}\" (global ${name} {typedecl}))\n",),
            )?;
        }

        for (_tableidx, table) in &code.exported_tables() {
            let name = table.export.clone().unwrap().name.clone();
            let initial = table.initial;
            let reftype = &table.reftype;

            write(
                stream,
                &format!("(import \"index\" \"{name}\" (table ${name} {initial} {reftype:?}))\n",),
            )?;
        }

        for (_funcidx, func) in &code.exported_funcs() {
            let name = func.export.clone().unwrap().name.clone();

            let paramtys: Vec<String> = func.ty.params.iter().map(|v| format!("{}", v)).collect();
            let paramtys = paramtys.join(" ");
            let resulttys: Vec<String> = func.ty.results.iter().map(|v| format!("{}", v)).collect();
            let resulttys = resulttys.join(" ");
            write(
            stream,
            &format!("(import \"index\" \"{name}\" (func ${name} (param {paramtys}) (result {resulttys})))\n",),
        )?;
        }
        let func_names = code
            .exported_funcs()
            .iter()
            .map(|(_i, f)| format!("${}", f.export.clone().unwrap().name.clone()))
            .collect::<Vec<String>>()
            .join(" ");
        write(stream, &format!("(elem declare func {func_names})\n",))?;

        // Export part
        // memories
        for (_i, memory) in &code.imported_mems() {
            let import = memory.import.clone().unwrap();
            let module = import.module.clone();
            let name = import.name.clone();
            let initial = memory.initial;
            let maximum = match memory.maximum {
                Some(max) => max.to_string(),
                None => "".to_string(),
            };

            write(
                stream,
                &format!("(import \"{module}\" \"{name}\" (memory {initial} {maximum}))\n",),
            )?;
        }
        for (_i, global) in &code.imported_globals() {
            let import = global.import.clone().unwrap();
            if import.module != *current_module {
                continue;
            }

            let module = import.module.clone();
            let name = import.name.clone();
            let valtype = global.valtype.clone();
            let typedecl = match global.mutable {
                true => format!("(mut {valtype})"),
                false => format!("{valtype}"),
            };
            write(
                stream,
                &format!("(import \"{module}\" \"{name}\" (global ${name} {typedecl}))\n",),
            )?;
        }
        // tables
        for (_i, table) in &code.imported_tables() {
            let import = table.import.clone().unwrap();
            if import.module != *current_module {
                continue;
            }
            let module = import.module.clone();
            let name = import.name.clone();
            let initial = table.initial;
            let maximum = match table.maximum {
                Some(max) => max.to_string(),
                None => "".to_string(),
            };
            let reftype = table.reftype.clone();
            write(
                stream,
                &format!(
                    "(import \"{module}\" \"{name}\" (table {initial} {maximum} {reftype:?}))\n",
                ),
            )?;
        }
        // functions

        for (funcidx, func) in &code.imported_funcs() {
            let import = func.import.clone().unwrap();
            if import.module != *current_module {
                continue;
            }
            // TODO: better handling of initialization
            if *funcidx == INIT_INDEX {
                continue;
            }
            let funcidx = *funcidx;
            let name = func.import.clone().unwrap().name.clone();
            let global_idx = format!("$global_{}", funcidx.to_string());
            let func = code.funcs.get(&funcidx).unwrap();
            write(
                stream,
                &format!("(global {global_idx} (mut i64) (i64.const 0))\n"),
            )?;
            let tystr = get_functy_strs(&func.ty);
            write(
                stream,
                &format!("(func ${name} (@name \"r3 {name}\") (export \"{name}\") {tystr}\n",),
            )?;
            for (i, body) in func.bodys.iter().enumerate() {
                if let Some(body) = body {
                    let mut bodystr = String::new();
                    let mut memory_writes = BTreeMap::new();
                    for event in body {
                        match event {
                            HostEvent::MutateMemory {
                                addr,
                                data,
                                import: _,
                                name: _,
                            } => {
                                memory_writes.insert(addr, data);
                            }
                            _ => bodystr += &hostevent_to_wat(event, code),
                        }
                    }
                    if memory_writes.len() > 0 {
                        merge_memory_writes(&mut bodystr, memory_writes, &mut data_segments);
                    }
                    write(
                        stream,
                        &format!(
                            "(if
                                    (i64.eq (global.get {global_idx}) (i64.const {i}))
                                    (then {bodystr}))\n"
                        ),
                    )?;
                }
            }
            write(
                stream,
                &format!(
                    "(global.get {global_idx}) (i64.const 1) (i64.add) (global.set {global_idx})\n"
                ),
            )?;
            let mut current = 0;
            for r in func.results.iter() {
                let ty = &code.funcs.get(&funcidx).unwrap().ty;
                let _param_tys = ty.params.clone();
                let new_c = current + r.reps;
                let c1 = current + 1;
                let c2 = new_c + 1;
                let res = match r.results.get(0) {
                    Some(v) => format!(
                        "(return ({} {v}))",
                        valty_to_const(ty.results.get(0).unwrap())
                    ),
                    None => "(return)".to_owned(),
                };
                write(
                    stream,
                    &format!(
                        " (if
                            (i32.and
                              (i64.ge_s (global.get {global_idx}) (i64.const {c1}))
                              (i64.lt_s (global.get {global_idx}) (i64.const {c2}))
                            )
                            (then
                                {res}
                            )
                          )"
                    ),
                )?;
                current = new_c;
            }
            let ty = &code.funcs.get(&funcidx).unwrap().ty;
            let _param_tys = ty.params.clone();
            let default_return = match ty.results.get(0) {
                Some(v) => match v {
                    ValType::I32 => "(i32.const 0)",
                    ValType::I64 => "(i64.const 0)",
                    ValType::F32 => "(f32.const 0)",
                    ValType::F64 => "(f64.const 0)",
                    ValType::V128 => todo!(),
                    ValType::Anyref => todo!(),
                    ValType::Externref => todo!(),
                },
                None => "",
            };
            write(stream, &format!("(return {})", default_return))?;
            write(stream, ")\n")?;
        }
        for data_segment in data_segments {
            write(stream, "(data \"")?;
            for byte in data_segment {
                let byte = byte.0 as usize;
                write(stream, &format!("\\{byte:02x}",))?;
            }
            write(stream, "\")\n")?;
        }

        if current_module == "main" {
            let initialization = code.funcs.get(&INIT_INDEX).unwrap().bodys.last().unwrap();
            write(
                stream,
                "(func (@name \"r3 main\")(export \"_start\") (export \"main\")\n",
            )?;
            if let Some(initialization) = initialization {
                for event in initialization {
                    write(stream, &format!("{}", hostevent_to_wat(&event, code)))?
                }
            }
            write(stream, "(return)\n)")?;
        }

        write(stream, ")\n")?;

        let binary = wat::parse_file(module_wat_path.clone()).unwrap();
        let module_wasm_path = replay_path
            .parent()
            .unwrap()
            .join(&format!("{current_module}.wasm"));
        let mut modle_wasm_file = File::create(&module_wasm_path).unwrap();
        modle_wasm_file.write_all(&binary).unwrap();
    }

    generate_replay_js(replay_path, &module_set, code)?;
    generate_single_wasm(replay_path, &module_set, code)?;

    Ok(())
}

fn generate_replay_js(
    replay_path: &Path,
    module_set: &HashSet<&String>,
    code: &Replay,
) -> Result<(), std::io::Error> {
    let replay_js_path = replay_path.parent().unwrap().join(&format!("replay.js"));
    let stream = &mut File::create(replay_js_path).unwrap();

    for (_i, memory) in &code.imported_mems() {
        let import = memory.import.clone().unwrap();
        let module = import.module.clone();
        let name = import.name.clone();
        let module_name = &format!("{module}_{name}");
        let initial = memory.initial;
        let maximum = match memory.maximum {
            Some(max) => max.to_string(),
            None => "undefined".to_string(),
        };

        write(
            stream,
            &format!(
                "const {module_name} = new WebAssembly.Memory({{ initial: {initial}, maximum: {maximum} }})\n"
            ),
        )?;
    }
    for (_i, global) in &code.imported_globals() {
        let import = global.import.clone().unwrap();
        let module = import.module.clone();
        let name = import.name.clone();
        let module_name = &format!("{module}_{name}");
        let valtype = global.valtype.clone();
        let mutable = global.mutable;
        let initial = global.initial;
        write(
            stream,
            &format!(
                "const {module_name} = new WebAssembly.Global({{ value: '{valtype}', mutable: {mutable}}}, {initial})\n"
            ),
        )?;
    }
    for (_i, table) in &code.imported_tables() {
        let import = table.import.clone().unwrap();
        let module = import.module.clone();
        let name = import.name.clone();
        let module_name = &format!("{module}_{name}");
        let initial = table.initial;
        let maximum = match table.maximum {
            Some(max) => max.to_string(),
            None => "undefined".to_string(),
        };
        let reftype = table.reftype.clone();
        write(
            stream,
            &format!(
                "const {module_name} = new WebAssembly.Table({{ initial: {initial}, maximum: {maximum}, element: '{reftype}'}})\n",)
        )?;
    }
    write(stream, "\n")?;
    let var_name = "index".to_string();
    let iterable = std::iter::once(&var_name).chain(module_set.clone().into_iter());
    for current_module in iterable {
        let wasm_path = replay_path
            .parent()
            .unwrap()
            .join(&format!("{current_module}.wasm"));
        let buffer = &fs::read(wasm_path).unwrap();
        let walrus_module = Module::from_buffer(buffer).unwrap();
        let module_set = walrus_module
            .imports
            .iter()
            .map(|import| &import.module)
            .collect::<HashSet<_>>();
        let module_escaped = current_module.replace(|c: char| !c.is_alphanumeric(), "_");
        let object = format!("{module_escaped}Import");
        let mut import_object_str = format!("const {object}  = {{}}\n");
        for module in module_set {
            import_object_str += &format!("{object}['{module}'] = {{}}\n");
        }
        for import in walrus_module.imports.iter() {
            let module = &import.module;
            let module_escaped = module.replace(|c: char| !c.is_alphanumeric(), "_");
            let name = &import.name;
            let module_name = &format!("{module_escaped}_{name}");
            let value = match import.kind {
                walrus::ImportKind::Function(_) => {
                    format!("(...args) => {{ return {module_escaped}.exports['{name}'](...args) }}")
                }
                _ => {
                    if module == "index" {
                        format!("index.exports.{name}")
                    } else {
                        format!("{module_name}")
                    }
                }
            };
            import_object_str += &format!("{object}['{module}']['{name}'] = {value}\n",);
        }

        write(
            stream,
            &format!(
                "{import_object_str}
const {module_escaped} = new WebAssembly.Instance(new WebAssembly.Module(await readFile(\"{current_module}.wasm\")), {module_escaped}Import)\n\n",
            ),
        )?;
    }

    write(
        stream,
        "main.exports.main();
async function readFile(filename) {
let data;
if (typeof Deno !== 'undefined') {
    data = await Deno.readFile(filename);
} else if (typeof process !== 'undefined') {
    const fs = await import('fs').then(module => module.promises);
    data = await fs.readFile(filename);
} else if (typeof Bun !== 'undefined') {
    data = await Bun.fs.readFile(filename, 'utf8');
} else {
    throw new Error('Not suppported');
}
return data;
} ",
    )?;
    Ok(())
}

fn generate_single_wasm(
    replay_path: &Path,
    module_set: &HashSet<&String>,
    code: &Replay,
) -> Result<(), std::io::Error> {
    let mut module_args = module_set
        .iter()
        .map(|module| vec![format!("{}.wasm", module), module.to_string()])
        .flatten()
        .collect::<Vec<String>>();
    // some shuffling of module args to make --rename-export-conflicts work
    if let Some(index) = module_args.iter().position(|x| *x == "main.wasm") {
        let main_wasm = module_args.remove(index);
        let main = module_args.remove(index);
        module_args.insert(0, main_wasm);
        module_args.insert(1, main);
        module_args.insert(2, "index.wasm".to_string());
        module_args.insert(3, "index".to_string());
    }
    let args = [
        "--rename-export-conflicts",
        "--enable-reference-types",
        "--enable-multimemory",
        "--enable-bulk-memory",
        "--debuginfo",
    ]
    .iter()
    .cloned()
    .chain(module_args.iter().map(|s| s.as_str()))
    .chain(["-o", "merged_1.wasm"]);
    let _output = Command::new("wasm-merge")
        .current_dir(replay_path.parent().unwrap())
        .args(args)
        .output()
        .expect("Failed to execute wasm-merge");

    let module_list = code.imported_modules();
    for module in &module_list {
        let module_wat_path = replay_path
            .parent()
            .unwrap()
            .join(&format!("{module}_merge.wat"));
        let stream = &mut File::create(&module_wat_path).unwrap();

        write!(stream, "(module\n")?;
        for (_, mem) in code.imported_mems() {
            match mem.import {
                Some(import) => {
                    if import.module == *module {
                        let name = import.name;
                        let initial = mem.initial;
                        let maximum = match mem.maximum {
                            Some(max) => max.to_string(),
                            None => "".to_string(),
                        };
                        write!(stream, "(memory (export \"{name}\") {initial} {maximum})\n")?;
                    }
                }
                None => {}
            }
        }
        for (_, table) in code.imported_tables() {
            match table.import {
                Some(import) => {
                    if import.module == *module {
                        let name = import.name;
                        let initial = table.initial;
                        let maximum = match table.maximum {
                            Some(max) => max.to_string(),
                            None => "".to_string(),
                        };
                        let reftype = &table.reftype;
                        write!(
                            stream,
                            "(table (export \"{name}\") {initial} {maximum} {reftype})\n"
                        )?;
                    }
                }
                None => {}
            }
        }
        for (_, global) in code.imported_globals() {
            match global.import {
                Some(import) => {
                    if import.module == *module {
                        let name = import.name;
                        let initial = global.initial;
                        let valtype = global.valtype.clone();
                        write!(
                            stream,
                            "(global (export \"{name}\") {valtype} ({valtype}.const {initial:?}))\n"
                        )?;
                    }
                }
                None => {}
            }
        }
        write!(stream, ")\n")?;
        let binary = wat::parse_file(module_wat_path.clone()).unwrap();
        let module_wasm_path = replay_path
            .parent()
            .unwrap()
            .join(&format!("{module}_merge.wasm"));
        let mut modle_wasm_file = File::create(&module_wasm_path).unwrap();
        modle_wasm_file.write_all(&binary).unwrap();
    }

    let module_args = module_list
        .iter()
        .map(|module| vec![format!("{module}_merge.wasm"), module.to_string()])
        .flatten()
        .collect::<Vec<String>>();
    let args = [
        "--rename-export-conflicts",
        "--enable-reference-types",
        "--enable-multimemory",
        "--enable-bulk-memory",
        "--debuginfo",
        "merged_1.wasm",
        "index",
    ]
    .iter()
    .cloned()
    .chain(module_args.iter().map(|s| s.as_str()))
    .chain(["-o", "merged_2.wasm"]);
    let _output = Command::new("wasm-merge")
        .current_dir(replay_path.parent().unwrap())
        .args(args)
        .output()
        .expect("Failed to execute wasm-merge");

    let _output = Command::new("wasm-opt")
        .current_dir(replay_path.parent().unwrap())
        .args([
            "--enable-reference-types",
            "--enable-gc",
            "--enable-bulk-memory",
            "--debuginfo",
            // for handling inlining of imported globals. Without this glob-merge node test will fail.
            "--simplify-globals",
            "merged_2.wasm",
            "-o",
            replay_path.to_str().unwrap(),
        ])
        .output()?;
    Ok(())
}

fn merge_memory_writes(
    bodystr: &mut String,
    memory_writes: BTreeMap<&i32, &Vec<F64>>,
    data_segments: &mut Vec<Vec<F64>>,
) {
    let mut partitions: Vec<(i32, Vec<F64>)> = Vec::new();
    let mut current_partition: Vec<F64> = Vec::new();
    let mut last_key: Option<i32> = None;
    let mut start_key: Option<i32> = None;

    for (key, value) in memory_writes {
        match last_key {
            Some(last_key) if last_key + 1 == *key => {
                current_partition.extend(value);
            }
            _ => {
                if !current_partition.is_empty() {
                    partitions.push((start_key.unwrap(), current_partition));
                }
                current_partition = value.clone();
                start_key = Some(*key);
            }
        }
        last_key = Some(*key);
    }

    if !current_partition.is_empty() {
        partitions.push((start_key.unwrap(), current_partition));
    }

    for (start_addr, data) in partitions {
        let memoryinit_threshold = 9;
        if data.len() >= memoryinit_threshold {
            let data_segment_idx = data_segments.len();
            let data_len = data.len();
            bodystr.push_str(&format!(
                    "(memory.init {data_segment_idx} (i32.const {start_addr}) (i32.const 0) (i32.const {data_len}))\n",
                ));
            data_segments.push(data);
        } else {
            // merging 4 bytes and 8 bytes is also possible
            for (j, byte) in data.iter().enumerate() {
                let addr = start_addr + j as i32;
                bodystr.push_str(&format!(
                    "(i32.store8 (i32.const {addr}) (i32.const {byte}))\n",
                ));
            }
        }
    }
}

fn valty_to_const(valty: &ValType) -> String {
    match valty {
        ValType::I32 => "i32.const",
        ValType::I64 => "i64.const",
        ValType::F32 => "f32.const",
        ValType::F64 => "f64.const",
        ValType::V128 => todo!(),
        ValType::Anyref => todo!(),
        ValType::Externref => todo!(),
    }
    .to_string()
}

fn hostevent_to_wat(event: &HostEvent, code: &Replay) -> String {
    let str = match event {
        HostEvent::ExportCall { idx, name, params } => {
            let func = code.funcs.get(idx).unwrap();

            let param_tys = func.ty.params.clone();
            let result_count = func.ty.results.len();
            let params = params
                .iter()
                .zip(param_tys.clone())
                .map(|(p, p_ty)| format!("({} {p:?})", valty_to_const(&p_ty)))
                .collect::<Vec<String>>()
                .join("\n");
            params + &format!("(call ${name})") + &("(drop)".repeat(result_count))
        }
        HostEvent::ExportCallTable {
            idx,
            table_name: _,
            funcidx,
            params,
        } => {
            let func = code.funcs.get(idx).unwrap();

            let param_tys = func.ty.params.clone();
            let result_tys = func.ty.results.clone();
            let tystr = get_functy_strs(&func.ty);
            let params = params
                .iter()
                .zip(param_tys.clone())
                .map(|(p, p_ty)| format!("({} {p})", valty_to_const(&p_ty)))
                .collect::<Vec<String>>()
                .join("\n");
            params
                + &format!("(call_indirect {tystr} (i32.const {funcidx}))",)
                + &("(drop)".repeat(result_tys.len()))
        }
        HostEvent::MutateMemory {
            addr,
            data,
            import: _,
            name: _,
        } => {
            let mut js_string = String::new();
            for (j, byte) in data.iter().enumerate() {
                js_string += &format!("i32.const {}\n", addr + j as i32);
                js_string += &format!("i32.const {}\n", byte);
                js_string += &format!("i32.store8\n",);
            }
            js_string
        }
        HostEvent::GrowMemory {
            amount,
            import: _,
            name: _,
        } => {
            format!("(memory.grow (i32.const {})) (drop)\n", amount)
        }
        HostEvent::MutateTable {
            tableidx,
            funcidx: _,
            idx,
            func_import: _,
            func_name,
            import: _,
            name: _,
        } => {
            format!("(table.set {tableidx} (i32.const {idx}) (ref.func ${func_name}))",)
        }
        HostEvent::GrowTable {
            idx,
            amount,
            import: _,
            name: _,
        } => {
            // TODO: check if (ref.null func) is correct
            format!("(table.grow {idx} (ref.null func) (i32.const {amount})) (drop)\n")
        }
        HostEvent::MutateGlobal {
            idx,
            value,
            valtype,
            import: _,
            name,
        } => {
            let valtype = match valtype {
                ValType::I32 => "i32.const",
                ValType::I64 => "i64.const",
                ValType::F32 => "f32.const",
                ValType::F64 => "f64.const",
                ValType::V128 => todo!(),
                ValType::Anyref => todo!(),
                ValType::Externref => todo!(),
            };
            let value = value;
            let _globalidx = idx;
            format!("({valtype} {value})\n") + &format!("(global.set ${name})")
        }
    };
    str
}

fn get_functy_strs(ty: &FunctionTy) -> String {
    let paramstr = ty
        .params
        .iter()
        .map(|p| format!("{}", p))
        .collect::<Vec<String>>()
        .join(" ");
    let resultstr = ty
        .results
        .iter()
        .map(|r| format!("{}", r))
        .collect::<Vec<String>>()
        .join(" ");
    format!("(param {paramstr}) (result {resultstr})")
}
