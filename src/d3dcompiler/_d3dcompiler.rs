use winapi::shared::basetsd::*;
use winapi::shared::guiddef::*;
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
mod create_linker;                  pub use create_linker::*;

mod disassemble_region;             pub use disassemble_region::*;
mod get_blob_part;                  pub use get_blob_part::*;
mod get_debug_info;                 pub use get_debug_info::*;
mod get_signature_blob;             pub use get_signature_blob::*;
mod get_trace_instruction_offsets;  pub use get_trace_instruction_offsets::*;
mod load_module;                    pub use load_module::*;
mod preprocess;                     pub use preprocess::*;
mod read_file_to_blob;              pub use read_file_to_blob::*;
mod reflect;                        pub use reflect::*;
mod reflect_library;                pub use reflect_library::*;
mod set_blob_part;                  pub use set_blob_part::*;
mod strip_shader;                   pub use strip_shader::*;
mod write_blob_to_file;             pub use write_blob_to_file::*;

/// Lazily-loaded `d3dcompiler_NN.dll`
#[allow(non_snake_case)] // fn ptrs
pub struct D3DCompiler {
    // https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/


    // https://github.com/MaulingMonkey/thin3dcompiler/issues/2
    // D3D10 APIs not exposed:
    // D3DDisassemble10Effect


    // https://github.com/MaulingMonkey/thin3dcompiler/issues/3
    // Undocumented APIs not exposed:
    // D3DAssemble


    // https://github.com/MaulingMonkey/thin3dcompiler/issues/4
    // Legacy APIs not (yet?) exposed:
    // D3DCompileFromMemory
    // D3DDisassembleCode
    // D3DDisassembleEffect
    // D3DGetCodeDebugInfo
    // D3DPreprocessFromMemory

    // https://github.com/MaulingMonkey/thin3dcompiler/issues/6
    // d3d11shadertracing.h not (yet?) exposed:
    // D3DDisassemble11Trace

    /// D3DCompiler_40.dll ..=
    D3DCompile: Option<unsafe extern "system" fn (
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
    D3DCompile2: Option<unsafe extern "system" fn (
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
    D3DCompileFromFile: Option<unsafe extern "system" fn (
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
    D3DCompressShaders: Option<unsafe extern "system" fn (
        uNumShaders:        UINT,
        pShaderData:        *mut D3D_SHADER_DATA,
        uFlags:             UINT,
        ppCompressedData:   *mut *mut ID3DBlob,
    ) -> HRESULT>,

    /// D3DCOMPILER_43.dll
    D3DCreateBlob: Option<unsafe extern "system" fn (
        Size:               SIZE_T,
        ppBlob:             *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DCreateFunctionLinkingGraph: Option<unsafe extern "system" fn (
        uFlags:                 UINT,
        ppFunctionLinkingGraph: *mut *mut ID3D11FunctionLinkingGraph,
    ) -> HRESULT>,

    D3DCreateLinker: Option<unsafe extern "system" fn (
        ppLinker:               *mut *mut ID3D11Linker,
    ) -> HRESULT>,

    D3DDecompressShaders: Option<unsafe extern "system" fn (
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        uNumShaders:        UINT,
        uStartIndex:        UINT,
        pIndices:           *mut UINT,
        uFlags:             UINT,
        ppShaders:          *mut *mut ID3DBlob,
        pTotalShaders:      *mut UINT,
    ) -> HRESULT>,

    D3DDisassemble: Option<unsafe extern "system" fn(
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        Flags:              UINT,
        szComments:         LPCSTR,
        ppDisassembly:      *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DDisassembleRegion: Option<unsafe extern "system" fn (
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        Flags:              UINT,
        szComments:         LPCSTR,
        StartByteOffset:    SIZE_T,
        NumInsts:           SIZE_T,
        pFinishByteOffset:  *mut SIZE_T,
        ppDisassembly:      *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DGetBlobPart: Option<unsafe extern "system" fn(
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        Part:               D3D_BLOB_PART,
        Flags:              UINT,
        ppPart:             *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DGetDebugInfo: Option<unsafe extern "system" fn(
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        ppSignatureBlob:    *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DGetInputAndOutputSignatureBlob: Option<unsafe extern "system" fn(
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        ppSignatureBlob:    *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DGetInputSignatureBlob: Option<unsafe extern "system" fn(
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        ppSignatureBlob:    *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DGetOutputSignatureBlob: Option<unsafe extern "system" fn(
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        ppSignatureBlob:    *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DGetTraceInstructionOffsets: Option<unsafe extern "system" fn (
        pSrcData:       LPCVOID,
        SrcDataSize:    SIZE_T,
        Flags:          UINT,
        StartInstIndex: SIZE_T,
        NumInsts:       SIZE_T,
        pOffsets:       *mut SIZE_T,
        pTotalInsts:    *mut SIZE_T,
    ) -> HRESULT>,

    D3DLoadModule: Option<unsafe extern "system" fn (
        pSrcData:       LPCVOID,
        cbSrcDataSize:  SIZE_T,
        ppModule:       *mut *mut ID3D11Module,
    ) -> HRESULT>,

    D3DPreprocess: Option<unsafe extern "system" fn (
        pSrcData:       LPCVOID,
        SrcDataSize:    SIZE_T,
        pSourceName:    LPCSTR,
        pDefines:       *const D3D_SHADER_MACRO,
        pInclude:       *mut ID3DInclude,
        ppCodeText:     *mut *mut ID3DBlob,
        ppErrorMsgs:    *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DReadFileToBlob: Option<unsafe extern "system" fn (
        pFileName:          LPCWSTR,
        ppContents:         *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DReflect: Option<unsafe extern "system" fn (
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        pInterface:         REFIID,
        ppReflector:        *mut LPVOID,
    ) -> HRESULT>,

    D3DReflectLibrary: Option<unsafe extern "system" fn (
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        riid:               REFIID,
        ppReflector:        *mut LPVOID,
    ) -> HRESULT>,

    D3DSetBlobPart: Option<unsafe extern "system" fn (
        pSrcData:           LPCVOID,
        SrcDataSize:        SIZE_T,
        Part:               D3D_BLOB_PART,
        Flags:              UINT,
        pPart:              LPCVOID,
        PartSize:           SIZE_T,
        ppNewShader:        *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DStripShader: Option<unsafe extern "system" fn (
        pShaderBytecode:    LPCVOID ,
        BytecodeLength:     SIZE_T,
        uStripFlags:        UINT,
        ppStrippedBlob:     *mut *mut ID3DBlob,
    ) -> HRESULT>,

    D3DWriteBlobToFile: Option<unsafe extern "system" fn (
        pBlob:      *mut ID3DBlob,
        pFileName:  LPCWSTR,
        bOverwrite: BOOL,
    ) -> HRESULT>,
}



#[test] fn d3dcompiler_nn() {
    for nn in [
        // All versions I have installed
        33, 34, 35, 36, 37, 38, 38, 40, 41, 42, 43, 47,
        // Build servers have less installed, so I've neutered this test :(
    ].iter().copied() {
        println!("d3dcompiler_{}.dll: {}", nn, D3DCompiler::new(nn).is_ok());
    }
}

#[test] fn d3dcompiler_47() {
    let _d3dc47 = D3DCompiler::new(47).unwrap();
}

#[should_panic] #[test] fn d3dcompiler_48() {
    let _d3dc48 = D3DCompiler::new(48).unwrap();
}