#[allow(unused_imports)] use crate::*;

use winapi::shared::minwindef::UINT;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_cbuffer_flags)\]
/// UINT / D3D_SHADER_CBUFFER_FLAGS_\*
///
/// Flags controlling how [D3DCompiler::disassemble_region] disassembles the compiled shader data.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderCbufferFlags(UINT);
use ShaderCbufferFlags as CBF;

flags! { CBF => UINT; None, UserPacked }

#[allow(non_upper_case_globals)] impl CBF { // These are enum-like
    pub const None          : CBF = CBF(0);

    /// Bind the constant buffer to an input slot defined in HLSL code (instead of letting the compiler choose the input slot).
    pub const UserPacked    : CBF = CBF(D3D_CBF_USERPACKED);
}

#[doc(hidden)] impl CBF { // Ctrl+C Ctrl+V support
    /// Bind the constant buffer to an input slot defined in HLSL code (instead of letting the compiler choose the input slot).
    pub const USERPACKED    : CBF = CBF(D3D_CBF_USERPACKED);
}

impl Default for CBF {
    fn default() -> Self { CBF::None }
}
