// extern crate tracer;
// use std::fs;

// use tracer::instrument_wasm;

// fn main() {
//     let test_name = "tests/rust-game-of-life";
//     let buffer = &fs::read(format!("{}.wasm", test_name)).unwrap();
//     let _ = match instrument_wasm(buffer) {
//         Ok(output) => {
//             dbg!(output.stats);
//             let wat = wasmprinter::print_bytes(output.instrumented).unwrap();
//             fs::write("tests/generated.wat", wat)
//         }
//         Err(str) => {
//             let _ = fs::write("tests/generated.wat", str);
//             panic!("{str}");
//         }
//     };
// }

fn main() {}
