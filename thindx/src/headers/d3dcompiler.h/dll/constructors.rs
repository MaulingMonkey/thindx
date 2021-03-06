use crate::d3d::*;

use std::path::*;



/// <h1 id="constructors" class="section-header"><a href="#constructors">Constructors</a></h1>
impl Compiler {
    /// Attempt to load d3dcompiler.dll
    ///
    /// ### Arguments
    /// *   `version` - the d3dcompiler.dll version to load
    ///     * [i32], [u32], [usize], [u64] - load `d3dcompiler_{version}.dll`
    ///     * &[str], &[String], &[Path], or &[PathBuf] - load `{version}`
    ///
    /// ### Returns
    /// *   Err([std::io::Error])   - if `d3dcompiler_{version}.dll` could not be loaded
    /// *   Ok([Compiler])          - `d3dcompiler_{version}.dll` was found
    ///
    /// ### Example
    /// ```rust
    /// use thindx::d3d;
    /// let d3dc = d3d::Compiler::new(47).unwrap();
    /// let d3dc = d3d::Compiler::new("d3dcompiler_47.dll").unwrap();
    /// ```
    pub fn new(version: impl CompilerNewVersion) -> Result<Self, std::io::Error> {
        let lib = version.try_load()?;
        unsafe{Ok(Self{
            D3DCompile:                         lib.sym_opt("D3DCompile\0"),
            D3DCompile2:                        lib.sym_opt("D3DCompile2\0"),
            D3DCompileFromFile:                 lib.sym_opt("D3DCompileFromFile\0"),
            D3DCompressShaders:                 lib.sym_opt("D3DCompressShaders\0"),
            D3DCreateBlob:                      lib.sym_opt("D3DCreateBlob\0"),
            D3DCreateFunctionLinkingGraph:      lib.sym_opt("D3DCreateFunctionLinkingGraph\0"),
            D3DCreateLinker:                    lib.sym_opt("D3DCreateLinker\0"),
            D3DDecompressShaders:               lib.sym_opt("D3DDecompressShaders\0"),
            D3DDisassemble:                     lib.sym_opt("D3DDisassemble\0"),
            D3DDisassembleRegion:               lib.sym_opt("D3DDisassembleRegion\0"),
            D3DGetBlobPart:                     lib.sym_opt("D3DGetBlobPart\0"),
            D3DGetDebugInfo:                    lib.sym_opt("D3DGetDebugInfo\0"),
            D3DGetInputAndOutputSignatureBlob:  lib.sym_opt("D3DGetInputAndOutputSignatureBlob\0"),
            D3DGetInputSignatureBlob:           lib.sym_opt("D3DGetInputSignatureBlob\0"),
            D3DGetOutputSignatureBlob:          lib.sym_opt("D3DGetOutputSignatureBlob\0"),
            D3DGetTraceInstructionOffsets:      lib.sym_opt("D3DGetTraceInstructionOffsets\0"),
            D3DLoadModule:                      lib.sym_opt("D3DLoadModule\0"),
            D3DPreprocess:                      lib.sym_opt("D3DPreprocess\0"),
            D3DReadFileToBlob:                  lib.sym_opt("D3DReadFileToBlob\0"),
            D3DReflect:                         lib.sym_opt("D3DReflect\0"),
            D3DReflectLibrary:                  lib.sym_opt("D3DReflectLibrary\0"),
            D3DSetBlobPart:                     lib.sym_opt("D3DSetBlobPart\0"),
            D3DStripShader:                     lib.sym_opt("D3DStripShader\0"),
            D3DWriteBlobToFile:                 lib.sym_opt("D3DWriteBlobToFile\0"),
        })}
    }
}

#[doc(hidden)] pub trait CompilerNewVersion : sealed::CompilerNewVersion {}
impl<T: sealed::CompilerNewVersion> CompilerNewVersion for T {}

mod sealed {
    use super::*;
    pub trait CompilerNewVersion             { fn try_load(self) -> minidl::Result<minidl::Library>; }
    impl CompilerNewVersion for i32          { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(format!("d3dcompiler_{}.dll", self)) } }
    impl CompilerNewVersion for u32          { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(format!("d3dcompiler_{}.dll", self)) } }
    impl CompilerNewVersion for usize        { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(format!("d3dcompiler_{}.dll", self)) } }
    impl CompilerNewVersion for u64          { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(format!("d3dcompiler_{}.dll", self)) } }
    impl CompilerNewVersion for &'_ Path     { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(self) } }
    impl CompilerNewVersion for &'_ PathBuf  { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(self) } }
    impl CompilerNewVersion for &'_ str      { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(self) } }
    impl CompilerNewVersion for &'_ String   { fn try_load(self) -> minidl::Result<minidl::Library> { minidl::Library::load(self) } }
}
