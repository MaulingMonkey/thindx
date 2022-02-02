#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::basetsd::UINT64;
use winapi::um::d3d11shader::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d11shader/nf-d3d11shader-id3d11shaderreflection-getrequiresflags#return-value)\]
/// UINT64 / D3D_SHADER_REQUIRES_\*
///
/// ### See Also
/// *   [d3d11::ShaderReflection::get_requires_flags]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ShaderRequires(UINT64);

flags! {
    ShaderRequires => UINT64;
    None, Doubles, EarlyDepthStencil, UavsAtEveryStage, _64_Uavs, MinimumPrecision, _11_1_DoubleExtensions,
    _11_1_ShaderExtensions, Level9ComparisonFiltering
}

#[allow(missing_docs)]
#[allow(non_upper_case_globals)] impl ShaderRequires { // These are enum-like
    #[doc=""]
    pub const None                          : ShaderRequires = ShaderRequires(0);
    pub const Doubles                       : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_DOUBLES);
    pub const EarlyDepthStencil             : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL);
    pub const UavsAtEveryStage              : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE);
    pub const _64_Uavs                      : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_64_UAVS);
    pub const MinimumPrecision              : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_MINIMUM_PRECISION);
    pub const _11_1_DoubleExtensions        : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS);
    pub const _11_1_ShaderExtensions        : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS);
    pub const Level9ComparisonFiltering     : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING);
    pub const TiledResources                : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_TILED_RESOURCES);
}

#[doc(hidden)] impl ShaderRequires { // Ctrl+C Ctrl+V support
    pub const DOUBLES                       : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_DOUBLES);
    pub const EARLY_DEPTH_STENCIL           : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL);
    pub const UAVS_AT_EVERY_STAGE           : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE);
    pub const _64_UAVS                      : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_64_UAVS);
    pub const MINIMUM_PRECISION             : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_MINIMUM_PRECISION);
    pub const _11_1_DOUBLE_EXTENSIONS       : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS);
    pub const _11_1_SHADER_EXTENSIONS       : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS);
    pub const LEVEL_9_COMPARISON_FILTERING  : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING);
    pub const TILED_RESOURCES               : ShaderRequires = ShaderRequires(D3D_SHADER_REQUIRES_TILED_RESOURCES);
}

impl Default for ShaderRequires {
    fn default() -> Self { ShaderRequires::None }
}

//#cpp2rust D3D_SHADER_REQUIRES_DOUBLES                         = d3d::ShaderRequires::Doubles
//#cpp2rust D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL             = d3d::ShaderRequires::EarlyDepthStencil
//#cpp2rust D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE             = d3d::ShaderRequires::UavsAtEveryStage
//#cpp2rust D3D_SHADER_REQUIRES_64_UAVS                         = d3d::ShaderRequires::_64_Uavs
//#cpp2rust D3D_SHADER_REQUIRES_MINIMUM_PRECISION               = d3d::ShaderRequires::MinimumPrecision
//#cpp2rust D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS          = d3d::ShaderRequires::_11_1_DoubleExtensions
//#cpp2rust D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS          = d3d::ShaderRequires::_11_1_ShaderExtensions
//#cpp2rust D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING    = d3d::ShaderRequires::Level9ComparisonFiltering
//#cpp2rust D3D_SHADER_REQUIRES_TILED_RESOURCES                 = d3d::ShaderRequires::TiledResources
