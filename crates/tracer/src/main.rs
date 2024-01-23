// extern crate tracer;
// use std::fs;

// use tracer::instrument_wasm;

// fn main() {
//     let test_name = "tests/heatmap";
//     let buffer = &fs::read(format!("{}.wasm", test_name)).unwrap();
//     let _ = match instrument_wasm(buffer) {
//         Ok(buffer) => fs::write("tests/instrumented.wasm", buffer),
//         Err(str) => {
//             let _ = fs::write("tests/generated.wat", str);
//             panic!("Didnt work");
//         },
//     };

// }

fn main() {}
