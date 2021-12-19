//! Use [d3d::Compiler::compile] and friends to compile HLSL to bytecode
#![allow(unused_variables)]
use thindx::d3d::{self, *};

fn main() {
    let compiler = d3d::Compiler::new(47).unwrap();
    let basic_hlsl   : &[u8] = include_bytes!(r"..\test\data\basic.hlsl");
    let library_hlsl : &[u8] = include_bytes!(r"..\test\data\library.hlsl");

    // Option A:  just preprocess instead of fully compiling
    let pixel_shader  = compiler.preprocess(basic_hlsl,   (),                      None, None,                  ).unwrap();
    let vertex_shader = compiler.preprocess(basic_hlsl,   r"test\data\basic.hlsl", None, StandardFileInclude    ).unwrap();
    let library       = compiler.preprocess(library_hlsl, r"library.hlsl",         None, None,                  ).unwrap();
    println!("pixel_shader\n============\n{}\n",    pixel_shader .shader.to_utf8_lossy());
    println!("vertex_shader\n=============\n{}\n",  vertex_shader.shader.to_utf8_lossy());
    println!("library\n=======\n{}\n",              library      .shader.to_utf8_lossy());

    // Option B:  compile_from_file
    let pixel_shader  = compiler.compile_from_file(r"test\data\basic.hlsl",   None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let vertex_shader = compiler.compile_from_file(r"test\data\basic.hlsl",   None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let library       = compiler.compile_from_file(r"test\data\library.hlsl", None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
    // resulting blobs are binary data

    // Option C:  compile
    let pixel_shader  = compiler.compile(basic_hlsl,   (),                      None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let vertex_shader = compiler.compile(basic_hlsl,   r"test\data\basic.hlsl", None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let library       = compiler.compile(library_hlsl, r"library.hlsl",         None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
    // resulting blobs are binary data

    // Option D:  compile2
    let pixel_shader  = compiler.compile2(basic_hlsl,   (), None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    let vertex_shader = compiler.compile2(basic_hlsl,   (), None, None, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    let library       = compiler.compile2(library_hlsl, (), None, None, (),       "lib_5_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    // resulting blobs are binary data

    // TODO: show ID3DInclude usage
    // TODO: show defines usage
    // TODO: show effects usage?
}

// TODO: include hlsl in docs (add an xtask directive?)
