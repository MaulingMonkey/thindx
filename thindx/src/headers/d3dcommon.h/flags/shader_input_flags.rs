#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_shader_input_flags)\]
/// D3D_SHADER_INPUT_FLAGS / D3D_SIF_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShaderInputFlags(D3D_SHADER_INPUT_FLAGS);
#[doc(hidden)] pub use ShaderInputFlags as SIF;

flags! { SIF => D3D_SHADER_INPUT_FLAGS; None, UserPacked, ComparisonSampler, TextureComponent0, TextureComponent1, TextureComponents, Unused }

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl SIF { // These are enum-like
    pub const None                  : SIF = SIF(0);
    pub const UserPacked            : SIF = SIF(D3D_SIF_USERPACKED);
    pub const ComparisonSampler     : SIF = SIF(D3D_SIF_COMPARISON_SAMPLER);
    pub const TextureComponent0     : SIF = SIF(D3D_SIF_TEXTURE_COMPONENT_0);
    pub const TextureComponent1     : SIF = SIF(D3D_SIF_TEXTURE_COMPONENT_1);
    pub const TextureComponents     : SIF = SIF(D3D_SIF_TEXTURE_COMPONENTS);
    pub const Unused                : SIF = SIF(D3D_SIF_UNUSED);
}

#[doc(hidden)] impl SIF { // Ctrl+C Ctrl+V support
    pub const USERPACKED            : SIF = SIF(D3D_SIF_USERPACKED);
    pub const COMPARISON_SAMPLER    : SIF = SIF(D3D_SIF_COMPARISON_SAMPLER);
    pub const TEXTURE_COMPONENT_0   : SIF = SIF(D3D_SIF_TEXTURE_COMPONENT_0);
    pub const TEXTURE_COMPONENT_1   : SIF = SIF(D3D_SIF_TEXTURE_COMPONENT_1);
    pub const TEXTURE_COMPONENTS    : SIF = SIF(D3D_SIF_TEXTURE_COMPONENTS);
    pub const UNUSED                : SIF = SIF(D3D_SIF_UNUSED);
}

impl Default for SIF {
    fn default() -> Self { SIF::None }
}
