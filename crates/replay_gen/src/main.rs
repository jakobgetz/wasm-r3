use replay_gen::codegen::{generate_javascript, generate_standalone};
use replay_gen::irgen::IRGenerator;
use replay_gen::opt::Optimiser;
use replay_gen::trace::{ErrorKind, WasmEvent};
use replay_gen::trace_optimisation::{CallOptimiser, ShadowMemoryOptimiser, ShadowTableOptimiser, TraceOptimiser};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};
use walrus::Module;

fn main() -> io::Result<()> {
    // TODO: use clap to parse args. currently just panics.
    let args: Vec<String> = env::args().collect();
    let subcommand = &args[1];
    if subcommand == "generate" {
        generate(args)
    } else if subcommand == "stringify" {
        bin_trace_to_string_representation(args)
    } else {
        panic!()
    }
}

fn generate(args: Vec<String>) -> io::Result<()> {
    let trace_path = Path::new(&args[2]);
    let wasm_path = Path::new(&args[3]);
    let binary = &args[4];
    let binding = args.get(5);
    let js_path = match &binding {
        Some(str) => Some(Path::new(str)),
        None => None,
    };

    let buffer = &fs::read(wasm_path).unwrap();
    let module = Module::from_buffer(buffer).unwrap();
    let mut shadow_mem_optimiser = ShadowMemoryOptimiser::new(&module);
    let mut shadow_table_optimiser = ShadowTableOptimiser::new(&module);
    let mut call_optimiser = CallOptimiser::new(&module);
    let mut generator = IRGenerator::new(&module);
    let trace = Trace::new(trace_path, &module, binary == "true");
    trace
        .map(|e| e.unwrap())
        .filter(|e| shadow_mem_optimiser.discard_event(e))
        // .filter(|e| shadow_table_optimiser.inspect_event(e))
        .filter(|e| call_optimiser.discard_event(e))
        .map(|e| shadow_table_optimiser.transform_event(e))
        .for_each(|e| generator.consume_event(e));

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

fn bin_trace_to_string_representation(args: Vec<String>) -> io::Result<()> {
    let trace_path = Path::new(&args[2]);
    let wasm_path = Path::new(&args[3]);
    let string_trace_path = Path::new(&args[4]);
    let buffer = &fs::read(wasm_path).unwrap();
    let module = Module::from_buffer(buffer).unwrap();
    let trace = Trace::new(trace_path, &module, true);
    let output_file = File::create(string_trace_path).unwrap();
    let mut writer = BufWriter::new(output_file);
    trace.for_each(|e| {
        writer.write_fmt(format_args!("{}", e.unwrap())).unwrap();
    });
    writer.flush()
}

fn decode_event(
    reader: &mut BufReader<File>,
    module: &Module,
    binary: bool,
    lookup: &Vec<i32>,
) -> Result<Option<WasmEvent>, ErrorKind> {
    if binary == true {
        WasmEvent::decode_bin(reader, module, lookup)
    } else {
        WasmEvent::decode_string(reader)
    }
}

struct Trace<'a> {
    reader: BufReader<File>,
    module: &'a Module,
    binary: bool,
    lookup: Vec<i32>,
}

impl<'a> Trace<'a> {
    fn new(path: &Path, module: &'a Module, binary: bool) -> Trace<'a> {
        let file = File::open(path).unwrap();
        let lookup = match fs::read_to_string(PathBuf::from(format!("{}.lookup", path.display()))) {
            Ok(l) => l
                .split("\n")
                .map(|i| if i.len() == 0 { 0 } else { i.parse().unwrap() })
                .collect(),
            Err(_) => Vec::new(),
        };
        let reader = BufReader::new(file);
        Trace { reader, module, binary, lookup }
    }
}

impl<'a> Iterator for Trace<'a> {
    type Item = Result<WasmEvent, ErrorKind>; // Replace `YourErrorType` with the actual error type

    fn next(&mut self) -> Option<Self::Item> {
        match decode_event(&mut self.reader, self.module, self.binary, &self.lookup) {
            Ok(Some(event)) => Some(Ok(event)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}
