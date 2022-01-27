#[allow(unused_imports)] use crate::*;
#[allow(unused_imports)] use crate::d3d::*;

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
/// &nbsp;   **[Constructors](#constructors)**<br>
/// [new](Compiler::load_system)(version: [u32]) -> Result&lt;[Compiler]&gt;<br>
///
/// &nbsp;   **[Compile & Preprocess HLSL to Bytecode](#compile)**<br>
/// [compile_from_file](Compiler::compile_from_file)(...) - compile hlsl to bytecode<br>
/// [compile](Compiler::compile)(...) - compile hlsl to bytecode<br>
/// [compile2](Compiler::compile2)(...) - compile hlsl to bytecode<br>
/// [preprocess](Compiler::preprocess)(...) - preprocess HLSL `#include`s and `#define`s<br>
///
/// &nbsp;   **[Manipulate Bytecode Archives](#archive)**<br>
/// [compress_shaders](Compiler::compress_shaders)(...) - compress hlsl or bytecode<br>
/// [decompress_shaders](Compiler::decompress_shaders)(...) - decompress shaders<br>
/// [decompress_shaders_inplace](Compiler::decompress_shaders_inplace)(...) - decompress shaders without allocating<br>
/// [decompress_shaders_count](Compiler::decompress_shaders_count)(...) - get the number of shaders in a compressed archive<br>
///
/// &nbsp;   **[Manipulate Bytecode by BlobPart](#parts)**<br>
/// [get_blob_part](Compiler::get_blob_part)(...) - read a [BlobPart] of a shader bytecode blob<br>
/// [get_debug_info](Compiler::get_debug_info)(...) - read [BlobPart::DebugInfo] of a shader bytecode blob<br>
/// [get_input_and_output_signature_blob](Compiler::get_input_and_output_signature_blob)(...) - read [BlobPart::InputAndOutputSignatureBlob] of a shader bytecode blob<br>
/// [get_input_signature_blob](Compiler::get_input_signature_blob)(...) - read [BlobPart::InputSignatureBlob] of a shader bytecode blob<br>
/// [get_output_signature_blob](Compiler::get_output_signature_blob)(...) - read [BlobPart::OutputSignatureBlob] of a shader bytecode blob<br>
/// [set_blob_part](Compiler::set_blob_part)(...) - write a [BlobPart] of a shader bytecode blob<br>
/// [strip_shader](Compiler::strip_shader)(...) - strip debug information etc. from bytecode<br>
///
/// &nbsp;   **[Bytecode Reflection](#reflection)**<br>
/// [reflect](Compiler::reflect)(...) - reflect over a single shader's bytecode<br>
/// [reflect_library](Compiler::reflect_library)(...) - ???<br>
///
/// &nbsp;   **[Bytecode Debugging](#debugging)**<br>
/// [disassemble](Compiler::disassemble)(...) - disassemble bytecode as human readable text<br>
/// [disassemble_region](Compiler::disassemble_region)(...) - disassemble bytecode as human readable text<br>
/// [get_trace_instruction_offsets_count](Compiler::get_trace_instruction_offsets_count)(...) - get the number of trace instruction byte offsets<br>
/// [get_trace_instruction_offsets_inplace](Compiler::get_trace_instruction_offsets_inplace)(...) - read trace instruction byte offsets<br>
/// [get_trace_instruction_offsets](Compiler::get_trace_instruction_offsets)(...) - read trace instruction byte offsets<br>
///
/// &nbsp;   **[ReadOnlyBlob Utilities](#blobs)**<br>
/// [create_read_only_blob](Compiler::create_read_only_blob)(...) - create a [ReadOnlyBlob] from memory<br>
/// [read_file_to_blob](Compiler::read_file_to_blob)(...) - read a [ReadOnlyBlob] from disk<br>
/// [write_blob_to_file](Compiler::write_blob_to_file)(...) - write a [ReadOnlyBlob] to disk<br>
///
/// &nbsp;   **[D3D11 Factories & APIs](#d3d11)**<br>
/// [create_function_linking_graph](Compiler::create_function_linking_graph)(...) - create a [d3d11::FunctionLinkingGraph]<br>
/// [create_linker](Compiler::create_linker)(...) - create a [d3d11::Linker]<br>
/// [load_module](Compiler::load_module)(...) - load a [d3d11::Module]<br>
#[allow(non_snake_case)] // fn ptrs
#[allow(clippy::type_complexity)] // no point extracting `type`s with these single use types... would just increase the chances of editing shrapnel
pub struct Compiler {
    // https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/

    // TODO: D3DDisassemble10Effect

    // Undocumented APIs not exposed:
    // TODO: D3DAssemble

    // Legacy APIs not (yet?) exposed:
    // TODO: D3DCompileFromMemory
    // TODO: D3DDisassembleCode
    // TODO: D3DDisassembleEffect
    // TODO: D3DGetCodeDebugInfo
    // TODO: D3DPreprocessFromMemory

    // d3d11shadertracing.h not (yet?) exposed:
    // TODO: D3DDisassemble11Trace

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
        println!("d3dcompiler_{}.dll: {}", nn, Compiler::load_system(nn).is_ok());
    }
}

#[test] fn d3dcompiler_47() {
    let _d3dc47 = Compiler::load_system(47).unwrap();
}

#[should_panic] #[test] fn d3dcompiler_48() {
    let _d3dc48 = Compiler::load_system(48).unwrap();
}
