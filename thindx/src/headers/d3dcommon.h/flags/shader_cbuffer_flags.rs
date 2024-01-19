#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::minwindef::UINT;
use winapi::um::d3dcommon::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_cbuffer_flags)\]
/// UINT / D3D_SHADER_CBUFFER_FLAGS_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ShaderCbufferFlags(UINT);
#[doc(hidden)] pub use ShaderCbufferFlags as CBF;

flags! { CBF => UINT; None, UserPacked }

#[allow(non_upper_case_globals)] impl CBF { // These are enum-like
    #[allow(missing_docs)]
    pub const None          : CBF = CBF(0);

    /// Bind the constant buffer to an input slot defined in HLSL code (instead of letting the compiler choose the input slot).
    pub const UserPacked    : CBF = CBF(D3D_CBF_USERPACKED);
}



//#cpp2rust D3D_SHADER_CBUFFER_FLAGS    = d3d::ShaderCbufferFlags

//#cpp2rust D3D_CBF_USERPACKED          = d3d::CBF::UserPacked
//#cpp2rust D3D10_CBF_USERPACKED        = d3d::CBF::UserPacked
