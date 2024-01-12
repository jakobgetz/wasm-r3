use walrus::{DataKind, ImportKind, Module};

use crate::trace::{LoadValue, WasmEvent};

pub trait TraceOptimiser {
    fn discard_event(&mut self, event: &WasmEvent) -> bool;
}

pub struct ShadowMemory(Vec<u8>);
impl ShadowMemory {
    fn new(module: &Module) -> Self {
        if module.memories.len() > 1 {
            todo!("Multiple memories not supported yet");
        }
        let mut memory = ShadowMemory(vec![
            0u8;
            module.memories.iter().find(|_| true).map(|m| m.initial * 65536).unwrap() as usize
        ]);
        module.data.iter().for_each(|d| {
            if let DataKind::Active(data) = &d.kind {
                let offset = match data.location {
                    walrus::ActiveDataLocation::Absolute(n) => n as usize,
                    walrus::ActiveDataLocation::Relative(_) => todo!("Relative data segment offset not supported yet"),
                };
                memory.store(offset, d.value.clone());
            }
        });
        memory
    }

    fn store(&mut self, offset: usize, data: impl Into<Vec<u8>>) {
        let data = data.into();
        self.0[offset..(offset + data.len())].copy_from_slice(&data);
    }

    fn contains_already(&mut self, offset: usize, data: LoadValue) -> bool {
        let data: Vec<u8> = data.into();
        if data == self.0[offset..(offset + data.len())].to_vec() {
            true
        } else {
            false
        }
    }
}

pub struct ShadowMemoryOptimiser {
    shadow_mem: ShadowMemory,
}

impl ShadowMemoryOptimiser {
    pub fn new(module: &Module) -> Self {
        Self { shadow_mem: ShadowMemory::new(module) }
    }
}

impl TraceOptimiser for ShadowMemoryOptimiser {
    fn discard_event(&mut self, event: &WasmEvent) -> bool {
       match event {
            WasmEvent::Load { offset, data, .. } => {
                if self.shadow_mem.contains_already(*offset as usize, data.clone()) {
                    false
                } else {
                    self.shadow_mem.store(*offset as usize, data.clone());
                    true
                }
            }
            WasmEvent::Store { offset, data, .. } => {
                self.shadow_mem.store(*offset as usize, data.clone());
                false
            }
            _ => true,
        }
    }
}

pub struct ShadowTableOptimiser {
    shadow_tables: u8,
}

impl ShadowTableOptimiser {
    pub fn new(module: &Module) -> Self {
        Self { shadow_tables: 8 }
    }
}

impl TraceOptimiser for ShadowTableOptimiser {
    fn discard_event(&mut self, event: &WasmEvent) -> bool {
        todo!()
    }
}

enum Scope {
    Internal,
    External(usize),
}
struct Union;
pub struct CallOptimiser {
    call_stack: Vec<Scope>,
    import_functions: Vec<Union>,
}

impl CallOptimiser {
    pub fn new(module: &Module) -> CallOptimiser {
        let mut import_functions = Vec::new();
        module.imports.iter().for_each(|i| {
            if let ImportKind::Function(_) = i.kind {
                import_functions.push(Union);
            }
        });
        CallOptimiser { import_functions, call_stack: vec![Scope::External(0)] }
    }
}

impl TraceOptimiser for CallOptimiser {
    fn discard_event(&mut self, event: &WasmEvent) -> bool {
        match event {
            WasmEvent::FuncEntry { .. } => {
                let mut keep = true;
                if let Scope::Internal = self.call_stack.last().unwrap() {
                    keep = false
                }
                self.call_stack.push(Scope::Internal);
                keep
            }
            WasmEvent::FuncReturn => {
                self.call_stack.pop();
                if self.call_stack.len() == 1 {
                    true
                } else {
                    false
                }
            }
            WasmEvent::Call { idx } => {
                if let None = self.import_functions.get(*idx) {
                    false
                } else {
                    self.call_stack.push(Scope::External(*idx));
                    true
                }
            }
            WasmEvent::CallReturn { .. } => {
                if let Scope::External(_) = self.call_stack.last().unwrap() {
                    self.call_stack.pop();
                    true
                } else {
                    false
                }
            }
            _ => true,
        }
    }
}
