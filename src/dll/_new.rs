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
            D3DDecompressShaders:   lib.sym_opt("D3DDecompressShaders\0"),

            D3DDisassembleRegion:               lib.sym_opt("D3DDisassembleRegion\0"),
            D3DGetBlobPart:                     lib.sym_opt("D3DGetBlobPart\0"),
            D3DGetDebugInfo:                    lib.sym_opt("D3DGetDebugInfo\0"),
            D3DGetInputAndOutputSignatureBlob:  lib.sym_opt("D3DGetInputAndOutputSignatureBlob\0"),
            D3DGetInputSignatureBlob:           lib.sym_opt("D3DGetInputSignatureBlob\0"),
            D3DGetOutputSignatureBlob:          lib.sym_opt("D3DGetOutputSignatureBlob\0"),
            D3DGetTraceInstructionOffsets: lib.sym_opt("D3DGetTraceInstructionOffsets\0"),
            D3DLoadModule:          lib.sym_opt("D3DLoadModule\0"),
            D3DPreprocess:          lib.sym_opt("D3DPreprocess\0"),
            D3DReadFileToBlob:      lib.sym_opt("D3DReadFileToBlob\0"),
            D3DReflect:             lib.sym_opt("D3DReflect\0"),
            D3DReflectLibrary:      lib.sym_opt("D3DReflectLibrary\0"),
            D3DSetBlobPart:         lib.sym_opt("D3DSetBlobPart\0"),
            D3DStripShader:         lib.sym_opt("D3DStripShader\0"),
            D3DWriteBlobToFile:     lib.sym_opt("D3DWriteBlobToFile\0"),
        })}
    }
}
