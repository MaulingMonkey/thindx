//! [d3d::Compiler] construction / storage
#![allow(unused_variables)]
use thindx::d3d;

fn main() {
    // The simplest option is to simply hardcode a specific version
    let d3dc = d3d::Compiler::load_system(47).unwrap();
    let d3dc = d3d::Compiler::load_insecure("d3dcompiler_47.dll").unwrap();

    // However, you can potentially allow a range of versions as well
    let d3dc = (33..=47).rev().find_map(|ver| d3d::Compiler::load_system(ver).ok()).unwrap();

    // TLS is also an option
    thread_local! { static D3DC : d3d::Compiler = d3d::Compiler::load_system(47).unwrap(); }

    // And lazy_static! should be too
    lazy_static::lazy_static! { static ref D3DC2 : d3d::Compiler = d3d::Compiler::load_system(47).unwrap(); }

    // Init failures are simple std::io::Error s:
    let err : std::io::Error = d3d::Compiler::load_system(9001).map(|_d3dc|()).unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    println!("{}", err);
}
