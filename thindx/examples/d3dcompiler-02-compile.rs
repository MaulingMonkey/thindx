//! Use [d3d::Compiler::compile] and friends to compile HLSL to bytecode
#![allow(unused_variables)]
use thindx::*;
use thindx::d3d::*;

use std::path::*;

fn main() {
    let d3dc = d3d::Compiler::load_system(47).unwrap();
    let basic_hlsl   : &[u8] = include_bytes!("../test/data/basic.hlsl");
    let library_hlsl : &[u8] = include_bytes!("../test/data/library.hlsl");

    // Option A:  just preprocess instead of fully compiling
    let pixel_shader  = d3dc.preprocess(basic_hlsl,   (),                               None, None,                  ).unwrap();
    let vertex_shader = d3dc.preprocess(basic_hlsl,   r"thindx\test\data\basic.hlsl",   None, StandardFileInclude    ).unwrap();
    let library       = d3dc.preprocess(library_hlsl, r"thindx\test\data\library.hlsl", None, None,                  ).unwrap();
    println!("pixel_shader\n============\n{}\n",    pixel_shader .shader);
    println!("vertex_shader\n=============\n{}\n",  vertex_shader.shader);
    println!("library\n=======\n{}\n",              library      .shader);

    // Option B:  compile_from_file
    let pixel_shader  = d3dc.compile_from_file(r"thindx\test\data\basic.hlsl",   None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let vertex_shader = d3dc.compile_from_file(r"thindx\test\data\basic.hlsl",   None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let library       = d3dc.compile_from_file(r"thindx\test\data\library.hlsl", None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
    // resulting blobs are binary data

    // Option C:  compile
    let pixel_shader  = d3dc.compile(basic_hlsl,   (),                                  None, None,                "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let vertex_shader = d3dc.compile(basic_hlsl,   r"thindx\test\data\basic.hlsl",      None, StandardFileInclude, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let library       = d3dc.compile(library_hlsl, r"thindx\test\data\library.hlsl",    None, None,                (),       "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
    // resulting blobs are binary data

    // Option D:  compile2
    let pixel_shader  = d3dc.compile2(basic_hlsl,   (), None, None, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    let vertex_shader = d3dc.compile2(basic_hlsl,   (), None, None, "vs_main", "vs_4_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    let library       = d3dc.compile2(library_hlsl, (), None, None, (),       "lib_5_0", Compile::Debug, CompileEffect::None, CompileSecdata::None, None).unwrap();
    // resulting blobs are binary data



    // ID3DInclude
    println!("ID3DInclude\n===========");
    let include1 = d3d::Include::from_blob_meta_fn(|include_type, file_name, parent|{
        let (quote, unquote) = match include_type {
            d3d::Include::Local     => ('"', '"'),
            d3d::Include::System    => ('<', '>'),
            _                       => ('?', '?'),
        };

        let file_name   = file_name.to_str().map_err(|_| E::FAIL)?;
        let path        = Path::new(r"thindx\test\data").join(file_name);

        println!("resolving `#include {quote}{file_name}{unquote}` to {path:?}");

        let data        = std::fs::read(&path).map_err(|_| E::FAIL)?;

        println!("  read {} bytes", data.len());

        if let Some(parent_path) = parent {
            println!("  into {:?}", parent_path);
        } else {
            println!("  into root file");
        }

        Ok((data, path))
    });

    let include2 = d3d::Include::from_path_fn(Path::new(r"thindx\test\data"), |dir, _ty, include| Ok(dir.join(include.to_str().map_err(|_| D3D11_ERROR::FILE_NOT_FOUND)?)));

    let ic1a = d3dc.compile_from_file(r"thindx\test\data\include-chain-1.hlsl", None, &include1,           "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let ic1b = d3dc.compile_from_file(r"thindx\test\data\include-chain-1.hlsl", None, &include2,           "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    let ic1c = d3dc.compile_from_file(r"thindx\test\data\include-chain-1.hlsl", None, StandardFileInclude, "ps_main", "ps_4_0", Compile::Debug, CompileEffect::None).unwrap();
    println!();

    // TODO: show defines usage
    // TODO: show effects usage?
}

// TODO: include hlsl in docs (add an xtask directive?)
