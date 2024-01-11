use walrus::{DataKind, Module};

use crate::trace::WasmEvent;

pub trait TraceOptimiser {
    fn inspect_event(&mut self, event: &WasmEvent) -> bool;
}

pub struct ShadowMemory(Vec<u8>);
impl ShadowMemory {
    fn new(module: &Module) -> Self {
        if module.memories.len() > 1 {
            todo!("Multiple memories not supported yet");
        }
        let mut memory = ShadowMemory(vec![
            0u8;
            module.memories.iter().find(|_| true).map(|m| m.initial).unwrap() as usize
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
        self.0[offset..data.len()].copy_from_slice(&data);
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
    fn inspect_event(&mut self, event: &WasmEvent) -> bool {
        todo!()
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
    fn inspect_event(&mut self, event: &WasmEvent) -> bool {
        todo!()
    }
}
