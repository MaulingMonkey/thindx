//! Use [d3d::Compiler::disassemble] and friends to inspect bytecode
#![allow(unused_variables)]
use thindx::*;

fn main() {
    let d3dc = d3d::Compiler::new(47).unwrap();
    let basic_hlsl : &[u8] = include_bytes!(r"..\test\data\basic.hlsl");
    let pixel_shader = d3dc.compile_from_file(r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0", d3d::Compile::Debug, d3d::CompileEffect::None).unwrap();
}
