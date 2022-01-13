#[allow(unused_imports)] use crate::*;
#[allow(unused_imports)] use crate::d3d::*;
use crate::ctypes::BOOL;

use winapi::shared::basetsd::*;
use winapi::shared::guiddef::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;

use winapi::um::d3dcommon::*;
use winapi::um::d3dcompiler::*;
use winapi::um::d3d11shader::*;



mod constructors;                   pub use constructors::*;
mod compile_;                       pub use compile_::*;
mod archive;                        pub use archive::*;
mod parts;                          pub use parts::*;
mod reflection;                     pub use reflection::*;
mod debugging;                      pub use debugging::*;
mod blobs;                          pub use blobs::*;
mod d3d11_methods;                  pub use d3d11_methods::*;



/// Lazily-loaded `d3dcompiler_NN.dll`
///
/// <div class="interface-tree">
///
/// &nbsp;   **[Constructors](#constructors)**
/// [new](Compiler::new)(version: [u32]) -> Result&lt;[Compiler]&gt;
///
/// &nbsp;   **[Compile & Preprocess HLSL to Bytecode](#compile)**
/// [compile_from_file](Compiler::compile_from_file)(...) - compile hlsl to bytecode
/// [compile](Compiler::compile)(...) - compile hlsl to bytecode
/// [compile2](Compiler::compile2)(...) - compile hlsl to bytecode
/// [preprocess](Compiler::preprocess)(...) - preprocess HLSL `#include`s and `#define`s
///
/// &nbsp;   **[Manipulate Bytecode Archives](#archive)**
/// [compress_shaders](Compiler::compress_shaders)(...) - compress hlsl or bytecode
/// [decompress_shaders](Compiler::decompress_shaders)(...) - decompress shaders
/// [decompress_shaders_inplace](Compiler::decompress_shaders_inplace)(...) - decompress shaders without allocating
/// [decompress_shaders_count](Compiler::decompress_shaders_count)(...) - get the number of shaders in a compressed archive
///
/// &nbsp;   **[Manipulate Bytecode by BlobPart](#parts)**
/// [get_blob_part](Compiler::get_blob_part)(...) - read a [BlobPart] of a shader bytecode blob
/// [get_debug_info](Compiler::get_debug_info)(...) - read [BlobPart::DebugInfo] of a shader bytecode blob
/// [get_input_and_output_signature_blob](Compiler::get_input_and_output_signature_blob)(...) - read [BlobPart::InputAndOutputSignatureBlob] of a shader bytecode blob
/// [get_input_signature_blob](Compiler::get_input_signature_blob)(...) - read [BlobPart::InputSignatureBlob] of a shader bytecode blob
/// [get_output_signature_blob](Compiler::get_output_signature_blob)(...) - read [BlobPart::OutputSignatureBlob] of a shader bytecode blob
/// [set_blob_part](Compiler::set_blob_part)(...) - write a [BlobPart] of a shader bytecode blob
/// [strip_shader](Compiler::strip_shader)(...) - strip debug information etc. from bytecode
///
/// &nbsp;   **[Bytecode Reflection](#reflection)**
/// [reflect](Compiler::reflect)(...) - reflect over a single shader's bytecode
/// [reflect_library](Compiler::reflect_library)(...) - ???
///
/// &nbsp;   **[Bytecode Debugging](#debugging)**
/// [disassemble](Compiler::disassemble)(...) - disassemble bytecode as human readable text
/// [disassemble_region](Compiler::disassemble_region)(...) - disassemble bytecode as human readable text
/// [get_trace_instruction_offsets_count](Compiler::get_trace_instruction_offsets_count)(...) - get the number of trace instruction byte offsets
/// [get_trace_instruction_offsets_inplace](Compiler::get_trace_instruction_offsets_inplace)(...) - read trace instruction byte offsets
/// [get_trace_instruction_offsets](Compiler::get_trace_instruction_offsets)(...) - read trace instruction byte offsets
///
/// &nbsp;   **[ReadOnlyBlob Utilities](#blobs)**
/// [create_read_only_blob](Compiler::create_read_only_blob)(...) - create a [ReadOnlyBlob] from memory
/// [read_file_to_blob](Compiler::read_file_to_blob)(...) - read a [ReadOnlyBlob] from disk
/// [write_blob_to_file](Compiler::write_blob_to_file)(...) - write a [ReadOnlyBlob] to disk
///
/// &nbsp;   **[D3D11 Factories & APIs](#d3d11)**
/// [create_function_linking_graph](Compiler::create_function_linking_graph)(...) - create a [d3d11::FunctionLinkingGraph]
/// [create_linker](Compiler::create_linker)(...) - create a [d3d11::Linker]
/// [load_module](Compiler::load_module)(...) - load a [d3d11::Module]</div>
#[allow(non_snake_case)] // fn ptrs
#[allow(clippy::type_complexity)] // no point extracting `type`s with these single use types... would just increase the chances of editing shrapnel
pub struct Compiler {
    // https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/


    // https://github.com/MaulingMonkey/thindx/issues/2
    // D3D10 APIs not exposed:
    // D3DDisassemble10Effect


    // https://github.com/MaulingMonkey/thindx/issues/3
    // Undocumented APIs not exposed:
    // D3DAssemble


    // https://github.com/MaulingMonkey/thindx/issues/4
    // Legacy APIs not (yet?) exposed:
    // D3DCompileFromMemory
    // D3DDisassembleCode
    // D3DDisassembleEffect
    // D3DGetCodeDebugInfo
    // D3DPreprocessFromMemory

    // https://github.com/MaulingMonkey/thindx/issues/6
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
        println!("d3dcompiler_{}.dll: {}", nn, Compiler::new(nn).is_ok());
    }
}

#[test] fn d3dcompiler_47() {
    let _d3dc47 = Compiler::new(47).unwrap();
}

#[should_panic] #[test] fn d3dcompiler_48() {
    let _d3dc48 = Compiler::new(48).unwrap();
}
