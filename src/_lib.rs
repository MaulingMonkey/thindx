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
//! ### Quickstart
//!
//! ```rust
//! use thin3dcompiler::*;
//!
//! let compiler = D3DCompiler::new(47).unwrap(); // Loads d3dcompiler_47.dll
//!
//! // Compile a pixel shader to shader model 4.0 bytecode
//! let result = compiler.compile_from_file(
//!     r"test\data\basic.hlsl", None, None, "ps_main", "ps_4_0",
//!     Compile::Debug, CompileEffect::None,
//! );
//!
//! // You want diagnostics?  You got diagnostics!
//! let pixel_shader : ReadOnlyBlob = match result {
//!     Ok(CompileResult { shader, errors: None }) => shader,
//!     Ok(CompileResult { shader, errors: Some(errors) }) => {
//!         println!("Shader built OK with warnings:");
//!         println!("{}", String::from_utf8_lossy(errors.get_buffer()));
//!         shader
//!     },
//!     Err(error) => {
//!         println!("Shader failed to build:");
//!         println!("{}", error);
//!         return;
//!     },
//! };
//!
//! // Get the raw bytes out of the ReadOnlyBlob
//! let bytes : &[u8] = pixel_shader.get_buffer();
//! ```
//!
//! See [D3DCompiler] for a *lot* more methods you can use!
//!
//! ### Factories
//!
//! | thin3dcompiler                                | docs.microsoft.com    | Desc |
//! | --------------------------------------------- | --------------------- | ---- |
//! | [D3DCompiler]                                 | `d3dcompiler_##.dll`  | The main factory for FXC bytecode (shader model 5 and earlier)
//! | <span class="inaccurate">DXCompiler</span>    | `dxcompiler.dll`      | The main factory for DXC bytecode (shader model 6+)
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



#![allow(broken_intra_doc_links)] // temporary
#![deny(unreachable_patterns)]

#[macro_use] mod macros; #[cfg(test)] use macros::*;



#[path="d3dcompiler/_d3dcompiler.rs"]   mod d3dcompiler;    pub use d3dcompiler::*;
#[path="d3d11/_d3d11.rs"]               pub mod d3d11;
#[path="enumerations/_enumerations.rs"] mod enumerations;   pub use enumerations::*;
#[path="flags/_flags.rs"]               mod flags;          pub use flags::*;
#[path="interfaces/_interfaces.rs"]     mod interfaces;     pub use interfaces::*;
#[path="structures/_structures.rs"]     mod structures;     pub use structures::*;
#[path="traits/_traits.rs"]             mod traits;         pub use traits::*;
#[path="values/_values.rs"]             mod values;         pub use values::*;

mod cstr_ptr;                   pub use cstr_ptr::*;
mod error_kind;                 pub use error_kind::*;
mod error;                      pub use error::*;
