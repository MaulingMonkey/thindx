use crate::*;

use minidl::*;



impl D3DCompiler {
    /// Attempt to load `d3dcompiler_{version}.dll` from system paths.
    ///
    /// ### Returns
    /// *   Err([std::io::Error])   - if `d3dcompiler_{version}.dll` could not be loaded
    /// *   Ok([D3DCompiler])       - `d3dcompiler_{version}.dll` was found
    ///
    /// ### Example
    /// ```rust
    /// use thin3dcompiler::*;
    /// let compiler = D3DCompiler::new(47).unwrap();
    /// ```
    pub fn new(version: u32) -> Result<Self> {
        let name = format!("d3dcompiler_{}.dll", version);
        let lib = Library::load(name)?;
        unsafe{Ok(Self{
            D3DCompile:             lib.sym_opt("D3DCompile\0"),
            D3DCompile2:            lib.sym_opt("D3DCompile2\0"),
            D3DCompileFromFile:     lib.sym_opt("D3DCompileFromFile\0"),
            D3DCompressShaders:     lib.sym_opt("D3DCompressShaders\0"),
            D3DCreateBlob:          lib.sym_opt("D3DCreateBlob\0"),
            D3DCreateFunctionLinkingGraph:  lib.sym_opt("D3DCreateFunctionLinkingGraph\0"),
            D3DCreateLinker:        lib.sym_opt("D3DCreateLinker\0"),

            D3DWriteBlobToFile:     lib.sym_opt("D3DWriteBlobToFile\0"),
        })}
    }
}
