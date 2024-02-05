//! Trace is just a sequence of WasmEvents.
//! WasmEvent corresponds to a single event that can happen during the execution of a Wasm program.
//! Most usually corresponds to one wasm instruction, e.g. WasmEvent::Load corresponds to one wasm load,
//! but some of them are not. e.g. FuncEntry and FuncReturn correspond to the entry and return of a wasm function.
//! There are also some events that are not part of the execution like Import*, which can be removed later.
use std::fmt::{self, Write};
use std::fmt::{Debug, Display};
use std::str::FromStr;

pub type Trace = Vec<WasmEvent>;

pub enum WasmEvent {
    // Each corresponds to a single wasm instruction.
    Load {
        idx: usize,
        name: String,
        offset: i32,
        data: Vec<F64>,
    },
    MemGrow {
        idx: usize,
        name: String,
        amount: i32,
    },
    TableGet {
        tableidx: usize,
        name: String,
        idx: i32,
        funcidx: usize,
        funcname: String,
    },
    TableGrow {
        idx: usize,
        name: String,
        amount: i32,
    },
    GlobalGet {
        idx: usize,
        name: String,
        value: F64,
        valtype: ValType,
    },
    ImportCall {
        idx: usize,
        name: String,
    },
    ImportReturn {
        idx: i32,
        name: String,
        results: Vec<F64>,
    },
    // These do not correspond to a wasm instruction, but used to track control flow
    FuncEntry {
        idx: usize,
        name: String,
        params: Vec<F64>,
    },
    FuncEntryTable {
        idx: usize,
        tablename: String,
        tableidx: i32,
        params: Vec<F64>,
    },
    FuncReturn,
    ImportGlobal {
        idx: usize,
        module: String,
        name: String,
        mutable: bool,
        initial: F64,
        value: ValType,
    },
}

pub fn encode_trace(trace: Trace) -> Result<String, std::fmt::Error> {
    let mut s = String::new();
    for event in trace {
        write!(&mut s, "{:?}\n", event)?;
    }
    Ok(s)
}

// TODO: this is a hack to get around the fact that the trace generated by js. Remove when we discard js based trace.
#[derive(Copy, Clone, PartialEq)]
pub struct F64(pub f64);

impl fmt::Display for F64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_infinite() {
            write!(f, "Infinity")
        } else if self.0.is_nan() {
            write!(f, "NaN")
        } else {
            write!(f, "{}", self.0)
        }
    }
}
// TODO: make this more elegant
// This is currently done for outputting to WAT.
impl fmt::Debug for F64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_infinite() {
            write!(f, "0x7FF0000000000000")
        } else if self.0.is_nan() {
            write!(f, "nan")
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl std::str::FromStr for F64 {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Infinity" => Ok(Self(std::f64::INFINITY)),
            _ => s.parse::<f64>().map(Self),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    Anyref,
    Externref,
}

#[derive(Clone, PartialEq)]
pub enum RefType {
    Anyref,
}
impl fmt::Display for RefType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "anyfunc")
    }
}
// TODO: make this more elegant
// This is currently done for outputting to WAT.
impl fmt::Debug for RefType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "funcref")
    }
}

impl Display for ValType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32 => write!(f, "i32"),
            Self::I64 => write!(f, "i64"),
            Self::F32 => write!(f, "f32"),
            Self::F64 => write!(f, "f64"),
            Self::V128 => write!(f, "v128"),
            Self::Anyref => write!(f, "anyref"),
            Self::Externref => write!(f, "externref"),
        }
    }
}

impl std::str::FromStr for ValType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "v128" => Ok(Self::V128),
            "anyref" => Ok(Self::Anyref),
            "externref" => Ok(Self::Externref),
            _ => Err(()),
        }
    }
}

impl From<walrus::ValType> for ValType {
    fn from(val: walrus::ValType) -> Self {
        match val {
            walrus::ValType::I32 => Self::I32,
            walrus::ValType::I64 => Self::I64,
            walrus::ValType::F32 => Self::F32,
            walrus::ValType::F64 => Self::F64,
            walrus::ValType::V128 => Self::V128,
            walrus::ValType::Externref => Self::Externref,
            walrus::ValType::Funcref => Self::Anyref,
        }
    }
}

fn join_vec(args: &Vec<F64>) -> String {
    args.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn parse_number(s: &str) -> Option<F64> {
    let s = s.trim(); // Remove leading/trailing whitespace

    match s {
        "" | "+" | "-" => None, // Handle empty or only sign character
        "Infinity" | "+Infinity" => Some(F64(std::f64::INFINITY)),
        "-Infinity" => Some(F64(std::f64::NEG_INFINITY)),
        _ => {
            if let Ok(num) = s.parse::<f64>() {
                Some(F64(num)) // Handle floats and scientific notation
            } else {
                None // Not a number
            }
        }
    }
}

#[test]
fn test_parse_number() {
    // problematic case reading the trace generateed by js
    let s = "0.7614822387695312";
    assert_ne!(s, parse_number(s).unwrap().to_string());
}

#[derive(Debug)]
pub enum ErrorKind {
    LegacyTrace,
    UnknownTrace,
}
impl FromStr for WasmEvent {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn split_list(c: &str) -> Vec<F64> {
            let list = c
                .split(',')
                .filter_map(|s| parse_number(s))
                .collect::<Vec<_>>();
            if list.is_empty() || (list.len() == 1 && list[0].0.is_nan()) {
                vec![]
            } else {
                list
            }
        }

        let components: Vec<&str> = s.split(';').collect();
        match components[0] {
            "EC" => Ok(WasmEvent::FuncEntry {
                idx: components[1].parse().unwrap(),
                name: components[2].to_string(),
                params: split_list(components.get(3).unwrap()),
            }),
            "TC" => Ok(WasmEvent::FuncEntryTable {
                idx: components[1].parse().unwrap(),
                tablename: components[2].to_string(),
                tableidx: components[3].parse().unwrap(),
                params: split_list(components.get(4).unwrap()),
            }),
            "ER" => Ok(WasmEvent::FuncReturn),
            "IC" => Ok(WasmEvent::ImportCall {
                idx: components[1].parse().unwrap(),
                name: components[2].to_string(),
            }),
            "IR" => Ok(WasmEvent::ImportReturn {
                idx: components[1].parse().unwrap(),
                name: components[2].to_string(),
                results: split_list(components.get(3).unwrap()),
            }),
            "L" => Ok(WasmEvent::Load {
                idx: components[1].parse().unwrap(),
                name: components[2].to_string(),
                offset: components[3].parse().unwrap(),
                data: split_list(components.get(4).unwrap()),
            }),
            "MG" => Ok(WasmEvent::MemGrow {
                idx: components[1].parse().unwrap(),
                name: components[2].to_string(),
                amount: components[3].parse().unwrap(),
            }),
            "T" => Ok(WasmEvent::TableGet {
                tableidx: components[1].parse().unwrap(),
                name: components[2].to_string(),
                idx: components[3].parse().unwrap(),
                funcidx: components[4].parse().unwrap(),
                funcname: components[5].to_string(),
            }),
            "TG" => Ok(WasmEvent::TableGrow {
                idx: components[1].parse().unwrap(),
                name: components[2].to_string(),
                amount: components[3].parse().unwrap(),
            }),
            "G" => Ok(WasmEvent::GlobalGet {
                idx: components[1].parse().unwrap(),
                name: components[2].to_string(),
                value: parse_number(components[3]).unwrap(),
                valtype: components[4].parse().unwrap(),
            }),
            "IG" => Ok(WasmEvent::ImportGlobal {
                idx: components[1].parse().unwrap(),
                module: components[2].to_string(),
                name: components[3].to_string(),
                value: components[4].parse().unwrap(),
                mutable: if components[5] == "1" { true } else { false },
                initial: components[6].parse().unwrap(),
            }),
            "IT" | "IM" | "IF" => Err(ErrorKind::LegacyTrace),
            _ => Err(ErrorKind::UnknownTrace),
        }
    }
}

impl Debug for WasmEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WasmEvent::Load {
                idx,
                name,
                offset,
                data,
            } => write!(f, "L;{};{};{};{}", idx, name, offset, join_vec(data)),
            WasmEvent::MemGrow { idx, name, amount } => {
                write!(f, "MG;{};{};{}", idx, name, amount)
            }
            WasmEvent::TableGet {
                tableidx,
                name,
                idx,
                funcidx,
                funcname,
            } => {
                write!(
                    f,
                    "T;{};{};{};{};{}",
                    tableidx, name, idx, funcidx, funcname
                )
            }
            WasmEvent::TableGrow { idx, name, amount } => {
                write!(f, "MG;{};{};{}", idx, name, amount)
            }
            WasmEvent::GlobalGet {
                idx,
                name,
                value,
                valtype,
            } => {
                write!(f, "G;{};{};{};{}", idx, name, value, valtype)
            }
            WasmEvent::FuncEntry { name, params, idx } => {
                write!(f, "EC{};{};{}", idx, name, join_vec(params))
            }
            WasmEvent::FuncEntryTable {
                idx,
                tablename,
                tableidx: funcidx,
                params,
            } => write!(
                f,
                "TC;{};{};{};{}",
                idx,
                tablename,
                funcidx,
                join_vec(params),
            ),
            WasmEvent::FuncReturn => write!(f, "ER"),
            WasmEvent::ImportCall { idx, name } => write!(f, "IC;{};{}", idx, name),
            WasmEvent::ImportReturn { idx, name, results } => {
                write!(f, "IR;{};{};{}", idx, name, join_vec(results),)
            }
            WasmEvent::ImportGlobal {
                idx,
                module,
                name,
                mutable,
                initial,
                value,
            } => {
                write!(
                    f,
                    "IG;{};{};{};{};{};{}",
                    idx,
                    module,
                    name,
                    value,
                    if *mutable { '1' } else { '0' },
                    initial
                )
            }
        }
    }
}
