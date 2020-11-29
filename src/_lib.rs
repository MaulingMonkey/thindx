//! Thin [D3DCompiler](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-d3dcompiler-reference) wrappers for Rust.
//! 
//! ### Why not `winapi` directly?
//! 
//! *   This crate aims to make fns safe/sound/slightly rusty when possible
//! *   Attempts to verify API soundness through unit tests, even if this will mostly test
//!     [D3DCompiler](https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/dx-graphics-d3dcompiler-reference)'s
//!     behavior.
//! *   Doc comments for one-stop intellisense, safety documentation, etc.
//!
//! ### Why not `<other graphics crate>`?
//! 
//! *   Most other graphics crates focus on **hiding** the underlying APIs as much as possible, improving portability.
//! *   This crate **surfaces** the underlying API `1:1`ish as much as possible, improving the potential for
//!     interoperability with other graphics code, or for accomplishing incremental rewrites.
//!
//! ### Is this crate sound?
//! ❌  Lack of thorough testing coverage means undefined behavior (integer overflows? parser/deserialization bugs?) in
//! d3dcompiler_##.dll probably leads to a few soundness holes in individual `fn`s.
//!
//! ⚠️  The public API shape/design can probably be made safe with few if any changes, through the addition of extra
//! internal bounds checks and other parameter validation.  The main caveat here is bytecode deserialization - it may
//! later turn out that I need to change the API to make constructing shader blobs from untrusted sources `unsafe`,
//! unless I want to write my own bytecode validator from scratch (I don't, way out of project scope!)
//!
//! ### Core API
//!
//! <div class="interface-tree">
//!
//! [D3DCompiler] - a lazily-loaded `d3dcompiler_NN.dll` that lets you report missing DLLs in a friendly way.
//! &nbsp;    │
//! &nbsp;    │  **[D3DCompiler] constructors**
//! &nbsp;    ├─ [new](D3DCompiler::new)(version: [u32]) -> Result&lt;[D3DCompiler]&gt;
//! &nbsp;    │
//! &nbsp;    │  **Compile HLSL to Bytecode**
//! &nbsp;    ├─ [compile_from_file](D3DCompiler::compile_from_file)(...) - compile hlsl to bytecode
//! &nbsp;    ├─ [compile](D3DCompiler::compile)(...) - compile hlsl to bytecode
//! &nbsp;    ├─ [compile2](D3DCompiler::compile2)(...) - compile hlsl to bytecode
//! &nbsp;    ├─ [preprocess](D3DCompiler::preprocess)(...) - preprocess HLSL `#include`s and `#define`s
//! &nbsp;    │
//! &nbsp;    │  **Bytecode Manipulation**
//! &nbsp;    ├─ [compress_shaders](D3DCompiler::compress_shaders)(...) - compress hlsl or bytecode
//! &nbsp;    ├─ [decompress_shaders](D3DCompiler::decompress_shaders)(...) - decompress shaders
//! &nbsp;    ├─ [decompress_shaders_inplace](D3DCompiler::decompress_shaders_inplace)(...) - decompress shaders without allocating
//! &nbsp;    ├─ [decompress_shaders_count](D3DCompiler::decompress_shaders_count)(...) - get the number of shaders in a compressed archive
//! &nbsp;    │
//! &nbsp;    │  **Bytecode [BlobPart] Manipulation**
//! &nbsp;    ├─ [get_blob_part](D3DCompiler::get_blob_part)(...) - read a [BlobPart] of a shader bytecode blob
//! &nbsp;    ├─ [get_debug_info](D3DCompiler::get_debug_info)(...) - read [BlobPart::DebugInfo] of a shader bytecode blob
//! &nbsp;    ├─ [get_input_and_output_signature_blob](D3DCompiler::get_input_and_output_signature_blob)(...) - read [BlobPart::InputAndOutputSignatureBlob] of a shader bytecode blob
//! &nbsp;    ├─ [get_input_signature_blob](D3DCompiler::get_input_signature_blob)(...) - read [BlobPart::InputSignatureBlob] of a shader bytecode blob
//! &nbsp;    ├─ [get_output_signature_blob](D3DCompiler::get_output_signature_blob)(...) - read [BlobPart::OutputSignatureBlob] of a shader bytecode blob
//! &nbsp;    ├─ [set_blob_part](D3DCompiler::set_blob_part)(...) - write a [BlobPart] of a shader bytecode blob
//! &nbsp;    ├─ [strip_shader](D3DCompiler::strip_shader)(...) - strip debug information etc. from bytecode
//! &nbsp;    │
//! &nbsp;    │  **Bytecode Reflection**
//! &nbsp;    ├─ [reflect](D3DCompiler::reflect)(...) - reflect over a single shader's bytecode
//! &nbsp;    ├─ [reflect_library](D3DCompiler::reflect_library)(...) - ???
//! &nbsp;    │
//! &nbsp;    │  **Bytecode Debugging**
//! &nbsp;    ├─ [disassemble](D3DCompiler::disassemble)(...) - disassemble bytecode as human readable text
//! &nbsp;    ├─ [disassemble_region](D3DCompiler::disassemble_region)(...) - disassemble bytecode as human readable text
//! &nbsp;    ├─ [get_trace_instruction_offsets_count](D3DCompiler::get_trace_instruction_offsets_count)(...) - get the number of trace instruction byte offsets
//! &nbsp;    ├─ [get_trace_instruction_offsets_inplace](D3DCompiler::get_trace_instruction_offsets_inplace)(...) - read trace instruction byte offsets
//! &nbsp;    ├─ [get_trace_instruction_offsets](D3DCompiler::get_trace_instruction_offsets)(...) - read trace instruction byte offsets
//! &nbsp;    │
//! &nbsp;    │  **[ReadOnlyBlob] Utilities**
//! &nbsp;    ├─ [create_read_only_blob](D3DCompiler::create_read_only_blob)(...) - create a [ReadOnlyBlob] from memory
//! &nbsp;    ├─ [read_file_to_blob](D3DCompiler::read_file_to_blob)(...) - read a [ReadOnlyBlob] from disk
//! &nbsp;    ├─ [write_blob_to_file](D3DCompiler::write_blob_to_file)(...) - write a [ReadOnlyBlob] to disk
//! &nbsp;    │
//! &nbsp;    │  **Create [d3d11] Factories/APIs**
//! &nbsp;    ├─ [create_function_linking_graph](D3DCompiler::create_function_linking_graph)(...) - create a [d3d11::FunctionLinkingGraph]
//! &nbsp;    ├─ [create_linker](D3DCompiler::create_linker)(...) - create a [d3d11::Linker]
//! &nbsp;    └─ [load_module](D3DCompiler::load_module)(...) - load a [d3d11::Module]
//!
//! <span class="inaccurate">DXCompiler</span> - TODO:  a lazily-loaded `dxcompiler.dll`
//! &nbsp;    │
//! &nbsp;    └ ???
//!
//! </div>
//!
//! ### Enumerations
//!
//! | thin3dcompiler        | docs.microsoft.com    | Desc |
//! | --------------------- | --------------------- | ---- |
//! | [BlobPart]            | [D3D_BLOB_PART]       | Identifies a subsection of a bytecode blob
//! | [ErrorKind]           | HRESULT               | Error kind, used throughout the API
//!
//! [D3D_BLOB_PART]:        https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3d_blob_part
//!
//! ### Flags
//!
//! | thin3dcompiler        | docs.microsoft.com    | Desc |
//! | --------------------- | --------------------- | ---- |
//! | [Compile]             | [D3DCOMPILE_\*]           | Flags controlling how HLSL is compiled to bytecode
//! | [CompileEffect]       | [D3DCOMPILE_EFFECT_\*]    | Flags controlling how HLSL **effects** shaders are compiled
//! | [CompileSecdata]      | [D3DCOMPILE_SECDATA_\*]   | Flags controlling how HLSL is compiled to bytecode via [compile2](D3DCompiler::compile2)
//! | [CompilerStrip]       | [D3DCOMPILER_STRIP_\*]    | Flags controlling what data is stripped by [D3DCompiler::strip_shader]
//! | [CompressShader]      | [D3D_COMPRESS_SHADER_\*]  | Flags controlling the behavior of [D3DCompiler::compress_shaders]
//! | [Disasm]              | [D3D_DISASM_\*]           | Flags controlling how [disassemble_region](D3DCompiler::disassemble_region) disassembles shader data
//! | [GetInstOffsets]      | [D3D_GET_INST_OFFSETS_\*] | Flags controlling [D3DCompiler::get_trace_instruction_offsets]
//!
//! [D3DCOMPILE_EFFECT_\*]:     https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-effect-constants
//! [D3DCOMPILE_SECDATA_\*]:    https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompile2#parameters
//! [D3DCOMPILE_\*]:            https://docs.microsoft.com/en-us/windows/win32/direct3dhlsl/d3dcompile-constants
//! [D3DCOMPILER_STRIP_\*]:     https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3dcompiler_strip_flags
//! [D3D_COMPRESS_SHADER_\*]:   https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dcompressshaders#parameters
//! [D3D_DISASM_\*]:            https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3ddisassembleregion#parameters
//! [D3D_GET_INST_OFFSETS_\*]:  https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/nf-d3dcompiler-d3dgettraceinstructionoffsets#parameters
//!
//! ### Structures
//!
//! | thin3dcompiler        | docs.microsoft.com    | Desc |
//! | --------------------- | --------------------- | ---- |
//! | [ShaderData]          | [D3D_SHADER_DATA]     | &amp;\[[u8]\] equivalent that's ABI-compatible with some D3D APIs
//! | [Error]               | <span style="opacity: 33%">N/A</span>
//!
//! [D3D_SHADER_DATA]:          https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ns-d3dcompiler-d3d_shader_data
//!
//! ### Traits
//!
//! | thin3dcompiler        | docs.microsoft.com    | Desc |
//! | --------------------- | --------------------- | ---- |
//! | [AsID3DInclude]       | [ID3DInclude]         | Conversion trait for resolving `#include`s
//! | [AsShaderMacros]      | [D3D_SHADER_MACRO]\[\]| Conversion trait for resolving `#define`s
//! | [Raw]                 | <span style="opacity: 33%">N/A</span> | Conversion to/from raw [winapi] types
//!
//! [ID3DInclude]:              https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/nn-d3dcommon-id3dinclude
//! [D3D_SHADER_MACRO]:         https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ns-d3dcommon-d3d_shader_macro
//!
//!
//!
//!
//!

// TODO: fix ???s above




#![allow(broken_intra_doc_links)] // temporary
#![deny(unreachable_patterns)]

#[macro_use] mod macros;



#[path="d3dcompiler/_d3dcompiler.rs"]   mod d3dcompiler;    pub use d3dcompiler::*;
#[path="enumerations/_enumerations.rs"] mod enumerations;   pub use enumerations::*;
#[path="flags/_flags.rs"]               mod flags;          pub use flags::*;
#[path="interfaces/_interfaces.rs"]     mod interfaces;     pub use interfaces::*;
#[path="structures/_structures.rs"]     mod structures;     pub use structures::*;
#[path="traits/_traits.rs"]             mod traits;         pub use traits::*;

mod error_kind;                 pub use error_kind::*;
mod error;                      pub use error::*;
