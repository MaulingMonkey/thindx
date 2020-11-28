use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;

use winapi::um::d3dcommon::*;
use winapi::um::d3dcompiler::*;
use winapi::um::d3d11shader::*;



mod _new;                           pub use _new::*;
mod compile_from_file;              pub use compile_from_file::*;
mod compile;                        pub use compile::*;
mod compile2;                       pub use compile2::*;
mod compress_shaders;               pub use compress_shaders::*;
mod create_blob;                    pub use create_blob::*;
mod create_function_linking_graph;  pub use create_function_linking_graph::*;

/// Lazily-loaded `d3dcompiler_NN.dll`
#[allow(non_snake_case)] // fn ptrs
pub struct D3DCompiler {
    // https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/

    // ..= D3DCompiler_39.dll
    // D3DCompileFromMemory

    /// D3DCompiler_40.dll ..=
    pub(crate) D3DCompile: Option<unsafe extern "system" fn (
        pSrcData:       LPCVOID,
        SrcDataSize:    SIZE_T,
        pSourceName:    LPCSTR,
        pDefines:       *const D3D_SHADER_MACRO,
        pInclude:       *mut ID3DInclude,
        pEntrypoint:    LPCSTR,
        pTarget:        LPCSTR,
        Flags1:         UINT,
        Flags2:         UINT,
        ppCode:         *mut *mut ID3DBlob,
        ppErrorMsgs:    *mut *mut ID3DBlob,
    ) -> HRESULT>,

    /// D3DCompiler_47.dll
    /// Possibly as early as D3DCompiler_44.dll ?  Need more install
    pub(crate) D3DCompile2: Option<unsafe extern "system" fn (
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        pSourceName:        LPCSTR,
        pDefines:           *const D3D_SHADER_MACRO,
        pInclude:           *mut ID3DInclude,
        pEntrypoint:        LPCSTR,
        pTarget:            LPCSTR,
        Flags1:             UINT,
        Flags2:             UINT,
        SecondaryDataFlags: UINT,
        pSecondaryData:     LPCVOID,
        SecondaryDataSize:  SIZE_T,
        ppCode:             *mut *mut ID3DBlob,
        ppErrorMsgs:        *mut *mut ID3DBlob,
    ) -> HRESULT>,

    /// D3DCompiler_47.dll
    /// Possibly as early as D3DCompiler_44.dll ?  Need more install
    pub(crate) D3DCompileFromFile: Option<unsafe extern "system" fn (
        pFileName:          LPCWSTR,
        pDefines:           *const D3D_SHADER_MACRO,
        pInclude:           *mut ID3DInclude,
        pEntrypoint:        LPCSTR,
        pTarget:            LPCSTR,
        Flags1:             UINT,
        Flags2:             UINT,
        ppCode:             *mut *mut ID3DBlob,
        ppErrorMsgs:        *mut *mut ID3DBlob,
    ) -> HRESULT>,

    /// D3DCOMPILER_43.dll
    pub(crate) D3DCompressShaders: Option<unsafe extern "system" fn (
        uNumShaders:        UINT,
        pShaderData:        *mut D3D_SHADER_DATA,
        uFlags:             UINT,
        ppCompressedData:   *mut *mut ID3DBlob,
    ) -> HRESULT>,

    /// D3DCOMPILER_43.dll
    pub(crate) D3DCreateBlob: Option<unsafe extern "system" fn (
        Size:               SIZE_T,
        ppBlob:             *mut *mut ID3DBlob,
    ) -> HRESULT>,

    pub(crate) D3DCreateFunctionLinkingGraph: Option<unsafe extern "system" fn (
        uFlags:                 UINT,
        ppFunctionLinkingGraph: *mut *mut ID3D11FunctionLinkingGraph,
    ) -> HRESULT>,

    // TODO: D3DCreateLinker
    // TODO: D3DDecompressShaders
    // TODO: D3DDisassemble
    // TODO: D3DDisassemble10Effect
    // TODO: D3DDisassembleRegion
    // TODO: D3DGetBlobPart
    // TODO: D3DGetDebugInfo
    // TODO: D3DGetInputAndOutputSignatureBlob
    // TODO: D3DGetInputSignatureBlob
    // TODO: D3DGetOutputSignatureBlob
    // TODO: D3DGetTraceInstructionOffsets
    // TODO: D3DLoadModule
    // TODO: D3DPreprocess
    // TODO: D3DReadFileToBlob
    // TODO: D3DReflect
    // TODO: D3DReflectLibrary
    // TODO: D3DSetBlobPart
    // TODO: D3DStripShader
    // TODO: D3DWriteBlobToFile
}



#[test] fn d3dcompiler_nn() {
    for nn in [
        // All versions I have installed
        33, 34, 35, 36, 37, 38, 38, 40, 41, 42, 43, 47,
    ].iter().copied() {
        println!("loading d3dcompiler_{}.dll", nn);
        let _ = D3DCompiler::new(nn).unwrap();
    }
}

#[test] fn d3dcompiler_47() {
    let _d3dc47 = D3DCompiler::new(47).unwrap();
}

#[should_panic] #[test] fn d3dcompiler_48() {
    let _d3dc48 = D3DCompiler::new(48).unwrap();
}
