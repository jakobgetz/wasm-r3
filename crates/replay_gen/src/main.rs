use replay_gen::codegen::{generate_javascript, generate_standalone};
use replay_gen::irgen::IRGenerator;
use replay_gen::opt::Optimiser;
use replay_gen::trace::{ErrorKind, WasmEvent};
use replay_gen::trace_optimisation::{ShadowMemoryOptimiser, ShadowTableOptimiser, TraceOptimiser};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::{env, fs};
use walrus::Module;

fn main() -> io::Result<()> {
    // TODO: use clap to parse args. currently just panics.
    let args: Vec<String> = env::args().collect();
    let trace_path = Path::new(&args[1]);
    let wasm_path = Path::new(&args[2]);
    let binary = &args[3];
    let binding = args.get(3);
    let js_path = match &binding {
        Some(str) => Some(Path::new(str)),
        None => None,
    };

    let buffer = &fs::read(wasm_path).unwrap();
    let module = Module::from_buffer(buffer).unwrap();
    let mut shadow_mem_optimiser = ShadowMemoryOptimiser::new(&module);
    let mut shadow_table_optimiser = ShadowTableOptimiser::new(&module);
    let mut generator = IRGenerator::new(&module);
    let trace = Trace::new(trace_path, &module, binary == "true");
    trace
        .map(|e| e.unwrap())
        // .filter(|e| shadow_mem_optimiser.inspect_event(e))
        // .filter(|e| shadow_table_optimiser.inspect_event(e))
        .for_each(|e| generator.consume_event(e));

    // let file = File::open(trace_path).unwrap();
    // let mut reader = BufReader::new(file);
    // let buffer = &fs::read(wasm_path).unwrap();
    // let module = Module::from_buffer(buffer).unwrap();
    // let mut shadow_mem_optimiser = ShadowMemoryOptimiser::new(&module);
    // let mut generator = IRGenerator::new(&module);
    // while let Some(event) = decode_event(&mut reader, &module, binary == "true").unwrap() {}
    // dbg!("SUCCESS");
    // while let Ok(event) = decode_event(&mut reader, &module, binary == "true") {
    // println!("{}", event);
    // drop(event);
    // let event = match shadow_mem_optimiser.consume_event(event) {
    //     Some(e) => e,
    //     None => continue,
    // };
    // generator.consume_event(event);
    // }

    // opt replay
    Optimiser::merge_fn_results(&mut generator.replay);

    let is_standalone = js_path.is_none();
    if is_standalone {
        generate_standalone(wasm_path, &generator.replay)?;
    } else {
        generate_javascript(js_path.unwrap(), &generator.replay)?;
    }

    Ok(())
}

fn decode_event(reader: &mut BufReader<File>, module: &Module, binary: bool) -> Result<Option<WasmEvent>, ErrorKind> {
    if binary == true {
        WasmEvent::decode_bin(reader, module)
    } else {
        WasmEvent::decode_string(reader)
    }
}

struct Trace<'a> {
    reader: BufReader<File>,
    module: &'a Module,
    binary: bool,
}

impl<'a> Trace<'a> {
    fn new(path: &Path, module: &'a Module, binary: bool) -> Trace<'a> {
        let file = File::open(path).unwrap();
        let mut reader = BufReader::new(file);
        Trace { reader, module, binary }
    }
}

impl<'a> Iterator for Trace<'a> {
    type Item = Result<WasmEvent, ErrorKind>; // Replace `YourErrorType` with the actual error type

    fn next(&mut self) -> Option<Self::Item> {
        match decode_event(&mut self.reader, self.module, self.binary) {
            Ok(Some(event)) => Some(Ok(event)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
