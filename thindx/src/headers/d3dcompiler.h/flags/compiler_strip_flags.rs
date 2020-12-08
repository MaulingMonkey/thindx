#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcompiler::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3dcompiler_strip_flags)\]
/// DWORD / D3DCOMPILER_STRIP_FLAGS / D3DCOMPILER_STRIP_*
///
/// Flags controlling what data is stripped by [d3d::Compiler::strip_shader]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct CompilerStripFlags(D3DCOMPILER_STRIP_FLAGS);
#[doc(hidden)] pub use CompilerStripFlags as CompilerStrip;

flags! { CompilerStrip => D3DCOMPILER_STRIP_FLAGS; None, ReflectionData, DebugInfo, TestBlobs, PrivateData, RootSignature }

#[allow(non_upper_case_globals)] impl CompilerStrip { // These are enum-like
    /// Strip nothing
    pub const None              : CompilerStrip = CompilerStrip(0);

    /// Strip reflection data (presumably disables [d3d::Compiler::reflect11] and friends?)
    pub const ReflectionData    : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_REFLECTION_DATA);

    /// Strip debug information (file/line/name info? [d3d::Blob::DebugInfo]?  Possibly [d3d::Blob::PDB] sections as well?)
    pub const DebugInfo         : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_DEBUG_INFO);

    /// Strip test blobs (e.g. [Blob::Test*](d3d::Blob::TestAlternateShader))
    pub const TestBlobs         : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_TEST_BLOBS);

    /// Strip [d3d::Blob::PrivateData] sections
    pub const PrivateData       : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_PRIVATE_DATA);

    /// Strip the root signature ([d3d::Blob::RootSignature]). Refer to [Specifying Root Signatures in HLSL] for more information on using Direct3D12 with HLSL.
    ///
    /// [Specifying Root Signatures in HLSL]:   https://docs.microsoft.com/en-us/windows/desktop/direct3d12/specifying-root-signatures-in-hlsl
    pub const RootSignature     : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_ROOT_SIGNATURE);
}

#[doc(hidden)] impl CompilerStrip { // Ctrl+C Ctrl+V support
    pub const NONE              : CompilerStrip = CompilerStrip(0);
    pub const REFLECTION_DATA   : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_REFLECTION_DATA);
    pub const DEBUG_INFO        : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_DEBUG_INFO);
    pub const TEST_BLOBS        : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_TEST_BLOBS);
    pub const PRIVATE_DATA      : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_PRIVATE_DATA);
    pub const ROOT_SIGNATURE    : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_ROOT_SIGNATURE);
}

impl Default for CompilerStripFlags {
    fn default() -> Self { CompilerStrip::None }
}
