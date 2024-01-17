//! IRGenerator is a tool to generate a replay IR from a trace file. The IR has type Replay.
//! It keeps the auxiliary states needed at field state.
//! Its main job is to put the right HostEvent into a right spot.
//! HostEvent corresponds to some event in the host context, which is classified into the effect
//! it has on wasm state. They get translated into different host code depending on the backend.
use std::collections::{BTreeMap, HashMap};

use walrus::Module;

use crate::trace::{ValType, WasmEvent, F64};

pub struct IRGenerator {
    pub replay: Replay,
    state: State,
    pub_functions: HashMap<usize, String>,
    pub_tables: HashMap<usize, String>,
    pub_globals: HashMap<usize, String>,
    pub_memories: HashMap<usize, String>,
}

pub struct Replay {
    // original index is usize but we use i32 to handle host initialization code
    // TODO: more elegant solution
    pub func_imports: BTreeMap<i32, Function>,
    pub func_idx_to_ty: BTreeMap<usize, FunctionTy>,
    pub mem_imports: BTreeMap<usize, Memory>,
    pub table_imports: BTreeMap<usize, Table>,
    pub global_imports: BTreeMap<usize, Global>,
    pub modules: Vec<String>,
}

struct State {
    host_call_stack: Vec<i32>,
    last_func: i32,
    last_table_get: usize,
}

#[derive(Clone, Debug)]
pub enum HostEvent {
    ExportCall {
        idx: usize,
        name: String,
        params: Vec<F64>,
    },
    ExportCallTable {
        idx: usize,
        table_name: String,
        funcidx: i32,
        params: Vec<F64>,
    },
    GrowMemory {
        amount: i32,
        import: bool,
        name: String,
    },
    GrowTable {
        idx: usize,
        amount: i32,
        import: bool,
        name: String,
    },
    MutateMemory {
        addr: i32,
        data: Vec<F64>,
        import: bool,
        name: String,
    },
    MutateGlobal {
        idx: usize,
        value: F64,
        valtype: ValType,
        import: bool,
        name: String,
    },
    MutateTable {
        tableidx: usize,
        funcidx: i32,
        idx: i32,
        func_import: bool,
        func_name: String,
        import: bool,
        name: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WriteResult {
    pub results: Vec<F64>,
    pub reps: i32,
}

pub type Context = Vec<HostEvent>;

#[derive(Clone, Debug)]
pub struct Function {
    pub module: String,
    pub name: String,
    pub bodys: Vec<Context>,
    pub results: Vec<WriteResult>,
}

#[derive(Clone, Debug)]
pub struct FunctionTy {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

pub struct Memory {
    pub module: String,
    pub name: String,
    pub initial: u32,
    pub maximum: Option<u32>,
}

pub struct Table {
    pub module: String,
    pub name: String,
    // enum is better
    pub element: String,
    pub initial: u32,
    pub maximum: Option<u32>,
}

pub struct Global {
    pub module: String,
    pub name: String,
    pub mutable: bool,
    pub initial: F64,
    pub value: ValType,
}

impl IRGenerator {
    pub fn new(module: &Module) -> Self {
        let mut pub_functions = HashMap::new();
        let mut pub_tables = HashMap::new();
        let mut pub_memories = HashMap::new();
        let mut pub_globals = HashMap::new();
        let mut func_imports = BTreeMap::new();

        func_imports.insert(
            -1,
            Function {
                module: "wasm-r3".to_string(),
                name: "initialization".to_string(),
                bodys: vec![vec![]],
                results: vec![],
            },
        );
        let mut mem_imports = BTreeMap::new();
        let mut table_imports = BTreeMap::new();
        let mut global_imports = BTreeMap::new();
        let mut func_idx_to_ty = BTreeMap::new();
        for f in module.funcs.iter() {
            let ty = module.types.get(f.ty());
            func_idx_to_ty.insert(
                f.id().index(),
                FunctionTy {
                    params: ty.params().iter().map(|p| (*p).into()).collect(),
                    results: ty.results().iter().map(|p| (*p).into()).collect(),
                },
            );
        }
        let mut modules = Vec::new();
        for i in module.imports.iter() {
            if let None = modules.iter().find(|m| i.module == **m) {
                modules.push(i.module.clone());
            }
            match i.kind {
                walrus::ImportKind::Function(f) => {
                    let _ty = module.types.get(module.funcs.get(f).ty());
                    func_imports.insert(
                        f.index() as i32,
                        Function {
                            module: i.module.to_string(),
                            name: i.name.to_string(),
                            bodys: vec![],
                            results: vec![],
                        },
                    );
                    pub_functions.insert(f.index(), i.name.clone());
                }
                walrus::ImportKind::Table(tid) => {
                    let table = module.tables.get(tid);
                    table_imports.insert(
                        tid.index(),
                        Table {
                            module: i.module.to_string(),
                            name: i.name.to_string(),
                            // want to replace anyfunc through t.refType but it holds the wrong string ('funcref')
                            element: "anyfunc".to_string(),
                            initial: table.initial,
                            maximum: table.maximum,
                        },
                    );
                    pub_tables.insert(tid.index(), i.name.clone());
                }
                walrus::ImportKind::Memory(mid) => {
                    let m = module.memories.get(mid);
                    mem_imports.insert(
                        mid.index(),
                        Memory {
                            module: i.module.to_string(),
                            name: i.name.to_string(),
                            initial: m.initial,
                            maximum: m.maximum,
                        },
                    );
                    pub_memories.insert(mid.index(), i.name.clone());
                }
                // Global is handled by the trace.
                walrus::ImportKind::Global(gid) => {
                    let g = module.globals.get(gid);
                    global_imports.insert(
                        gid.index(),
                        Global {
                            module: i.module.to_string(),
                            name: i.name.to_string(),
                            mutable: g.mutable,
                            initial: F64(0.0),
                            value: g.ty.into(),
                        },
                    );
                    pub_globals.insert(gid.index(), i.name.clone());
                }
            }
        }
        module.exports.iter().for_each(|e| {
            match e.item {
                walrus::ExportItem::Function(id) => pub_functions.insert(id.index(), e.name.clone()),
                walrus::ExportItem::Table(id) => pub_tables.insert(id.index(), e.name.clone()),
                walrus::ExportItem::Memory(id) => pub_memories.insert(id.index(), e.name.clone()),
                walrus::ExportItem::Global(id) => pub_globals.insert(id.index(), e.name.clone()),
            };
        });
        Self {
            replay: Replay {
                func_imports,
                mem_imports,
                table_imports,
                func_idx_to_ty,
                global_imports,
                modules,
            },
            // -1 is the _start function
            state: State {
                host_call_stack: vec![-1], //
                last_func: -1,
                last_table_get: 0,
            },
            pub_functions,
            pub_globals,
            pub_memories,
            pub_tables,
        }
    }

    fn push_call(&mut self, event: HostEvent) {
        let idx = self.state.host_call_stack.last().unwrap();
        let current_context = self.idx_to_cxt(*idx);

        current_context.push(event.clone());
    }

    pub fn consume_event(&mut self, event: WasmEvent) {
        match event {
            WasmEvent::FuncEntry { idx, params } => {
                match self.pub_functions.get(&idx) {
                    Some(name) => self.push_call(HostEvent::ExportCall { idx, name: name.clone(), params }),
                    None => {
                        self.push_call(HostEvent::ExportCallTable {
                            idx: self.state.last_table_get,
                            table_name: self.pub_tables.get(&self.state.last_table_get).unwrap().clone(),
                            funcidx: idx as i32,
                            params,
                        });
                    }
                };
            }
            WasmEvent::FuncEntryTable { idx, tableidx, params } => {
                self.push_call(HostEvent::ExportCallTable {
                    idx: tableidx,
                    table_name: self.pub_tables.get(&tableidx).unwrap().clone(),
                    funcidx: idx as i32,
                    params: params.clone(),
                });
            }
            WasmEvent::FuncReturn => {}
            WasmEvent::Load { idx, offset, data } => {
                self.splice_event(HostEvent::MutateMemory {
                    import: self.replay.mem_imports.contains_key(&idx),
                    name: self.pub_memories.get(&idx).unwrap().clone(),
                    addr: offset,
                    data: data.into(),
                });
            }
            WasmEvent::MemGrow { idx, amount } => {
                self.splice_event(HostEvent::GrowMemory {
                    import: self.replay.mem_imports.contains_key(&idx),
                    name: self.pub_memories.get(&idx).unwrap().clone(),
                    amount: amount,
                });
            }
            WasmEvent::TableGet { tableidx, idx, funcidx } => {
                if let Some(name) = self.pub_functions.get(&(funcidx as usize)) {
                    self.splice_event(HostEvent::MutateTable {
                        tableidx,
                        funcidx,
                        import: self.replay.table_imports.contains_key(&tableidx),
                        name: self.pub_tables.get(&idx).unwrap().clone(),
                        idx: idx as i32,
                        func_import: self.replay.func_imports.contains_key(&funcidx),
                        func_name: name.clone(),
                    });
                }
            }
            WasmEvent::TableGrow { idx, amount } => {
                self.splice_event(HostEvent::GrowTable {
                    import: self.replay.table_imports.contains_key(&idx),
                    name: self.pub_tables.get(&idx).unwrap().clone(),
                    idx: idx,
                    amount: amount,
                });
            }
            WasmEvent::GlobalGet { idx, value, valtype } => {
                self.splice_event(HostEvent::MutateGlobal {
                    idx: idx,
                    import: self.replay.global_imports.contains_key(&idx),
                    name: self.pub_globals.get(&idx).unwrap().clone(),
                    value: value,
                    valtype: valtype.clone(),
                });
            }

            WasmEvent::Call { idx } => {
                let idx = idx as i32;
                self.replay.func_imports.get_mut(&idx).unwrap().bodys.push(vec![]);
                self.state.host_call_stack.push(idx);
                self.state.last_func = idx;
            }
            WasmEvent::CallReturn { idx: _idx, results } => {
                let current_fn_idx = self.state.host_call_stack.last().unwrap();
                let r = &mut self.replay.func_imports.get_mut(&current_fn_idx).unwrap().results;
                r.push(WriteResult { results: results.clone(), reps: 1 });
                self.state.last_func = self.state.host_call_stack.pop().unwrap();
            }
            WasmEvent::Store { idx, offset, data } => {}
            WasmEvent::TableSet { tableidx, idx, funcidx } => {}
            WasmEvent::GlobalSet { idx, value, valtype } => {}
            WasmEvent::CallIndirect { tableidx, idx, funcidx } => todo!(),
        }
    }
    fn splice_event(&mut self, event: HostEvent) {
        let idx = self.state.host_call_stack.last().unwrap();
        let last_idx = self.state.last_func;
        let last_import_call = *idx == last_idx;
        let current_context = self.idx_to_cxt(*idx);

        if last_import_call {
            current_context.insert(current_context.len() - 1, event);
        } else {
            let last_idx = &self.state.last_func;
            let last_context = self.idx_to_cxt(*last_idx);
            last_context.push(event.clone());
        }
    }

    fn idx_to_cxt(&mut self, idx: i32) -> &mut Vec<HostEvent> {
        let current_context = self.replay.func_imports.get_mut(&idx).unwrap().bodys.last_mut().unwrap();
        current_context
    }
}
