//! Use [d3d::Compiler::compile] and friends to compile HLSL to bytecode
#![allow(unused_variables)]
use thindx::*;
use thindx::d3d::*;

fn main() {
    let d3dc = d3d::Compiler::new(47).unwrap();
    let basic_hlsl   : &[u8] = include_bytes!("../test/data/basic.hlsl");
    let library_hlsl : &[u8] = include_bytes!("../test/data/library.hlsl");

    // Option A:  just preprocess instead of fully compiling
    let pixel_shader  = d3dc.preprocess(basic_hlsl,   (),                      None, None,                  ).unwrap();
    let vertex_shader = d3dc.preprocess(basic_hlsl,   r"test\data\basic.hlsl", None, StandardFileInclude    ).unwrap();
    let library       = d3dc.preprocess(library_hlsl, r"library.hlsl",         None, None,                  ).unwrap();
    println!("pixel_shader\n============\n{}\n",    pixel_shader .shader);
    println!("vertex_shader\n=============\n{}\n",  vertex_shader.shader);
    println!("library\n=======\n{}\n",              library      .shader);

    // Option B:  compile_from_file
    let pixel_shader  = d3dc.compile_from_file(r"test\data\basic.hlsl",   None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let vertex_shader = d3dc.compile_from_file(r"test\data\basic.hlsl",   None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let library       = d3dc.compile_from_file(r"test\data\library.hlsl", None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
    // resulting blobs are binary data

    // Option C:  compile
    let pixel_shader  = d3dc.compile(basic_hlsl,   (),                      None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let vertex_shader = d3dc.compile(basic_hlsl,   r"test\data\basic.hlsl", None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let library       = d3dc.compile(library_hlsl, r"library.hlsl",         None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
    // resulting blobs are binary data

    // Option D:  compile2
    let pixel_shader  = d3dc.compile2(basic_hlsl,   (), None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    let vertex_shader = d3dc.compile2(basic_hlsl,   (), None, None, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    let library       = d3dc.compile2(library_hlsl, (), None, None, (),       "lib_5_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    // resulting blobs are binary data

    // TODO: show ID3DInclude usage
    // TODO: show defines usage
    // TODO: show effects usage?
}

// TODO: include hlsl in docs (add an xtask directive?)
