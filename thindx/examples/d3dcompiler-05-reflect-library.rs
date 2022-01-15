//! Use [d3d::Compiler] to inspect shader bytecode
use thindx::*;
use thindx::d3d::*;

fn main() {
    let d3dc = d3d::Compiler::load_system(47).unwrap();
    let library = d3dc.compile_from_file(r"thindx\test\data\library.hlsl", None, None, (), "lib_5_0", Compile::Debug, CompileEffect::None).unwrap();
    let _library = d3dc.reflect_library::<d3d11::LibraryReflection>(&library).unwrap();
    let library  = d3dc.reflect_library_11(&library).unwrap(); // equivalent shorthand

    println!("library");
    println!("=======");
    println!("{:?}\n", library.get_desc().unwrap());
    for function in library.functions().unwrap() {
        let desc = function.get_desc().unwrap();
        println!("{:#?}\n", desc);
        // TODO: plenty
    }
}
