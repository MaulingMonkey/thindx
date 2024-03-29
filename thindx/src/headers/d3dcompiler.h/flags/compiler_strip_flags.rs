#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::um::d3dcompiler::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcompiler/ne-d3dcompiler-d3dcompiler_strip_flags)\]
/// DWORD / D3DCOMPILER_STRIP_FLAGS / D3DCOMPILER_STRIP_*
///
/// Flags controlling what data is stripped by [d3d::Compiler::strip_shader]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct CompilerStripFlags(D3DCOMPILER_STRIP_FLAGS);
#[doc(hidden)] pub use CompilerStripFlags as CompilerStrip;

flags! { CompilerStrip => D3DCOMPILER_STRIP_FLAGS; None, ReflectionData, DebugInfo, TestBlobs, PrivateData, RootSignature }

#[allow(non_upper_case_globals)] impl CompilerStrip { // These are enum-like
    /// Strip nothing
    pub const None              : CompilerStrip = CompilerStrip(0);

    /// Strip reflection data (presumably disables [d3d::Compiler::reflect11] and friends?)
    pub const ReflectionData    : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_REFLECTION_DATA);

    /// Strip debug information (file/line/name info? [d3d::Blob::DebugInfo]?  Possibly [d3d::Blob::Pdb] sections as well?)
    pub const DebugInfo         : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_DEBUG_INFO);

    /// Strip test blobs (e.g. [Blob::Test*](d3d::Blob::TestAlternateShader))
    pub const TestBlobs         : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_TEST_BLOBS);

    /// Strip [d3d::Blob::PrivateData] sections
    pub const PrivateData       : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_PRIVATE_DATA);

    /// Strip the root signature ([d3d::Blob::RootSignature]). Refer to [Specifying Root Signatures in HLSL] for more information on using Direct3D12 with HLSL.
    ///
    /// [Specifying Root Signatures in HLSL]:   https://learn.microsoft.com/en-us/windows/desktop/direct3d12/specifying-root-signatures-in-hlsl
    pub const RootSignature     : CompilerStrip = CompilerStrip(D3DCOMPILER_STRIP_ROOT_SIGNATURE);
}



//#cpp2rust D3DCOMPILER_STRIP_FLAGS             = d3d::CompilerStripFlags

//#cpp2rust D3DCOMPILER_STRIP_REFLECTION_DATA   = d3d::CompilerStrip::ReflectionData
//#cpp2rust D3DCOMPILER_STRIP_DEBUG_INFO        = d3d::CompilerStrip::DebugInfo
//#cpp2rust D3DCOMPILER_STRIP_TEST_BLOBS        = d3d::CompilerStrip::TestBlobs
//#cpp2rust D3DCOMPILER_STRIP_PRIVATE_DATA      = d3d::CompilerStrip::PrivateData
//#cpp2rust D3DCOMPILER_STRIP_ROOT_SIGNATURE    = d3d::CompilerStrip::RootSignature
