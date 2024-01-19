#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DPTEXTURECAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct PTextureCaps(DWORD);

flags! {
    PTextureCaps => DWORD;
    None, Alpha, AlphaPalette, CubeMap, CubeMapPow2, MipCubeMap, MipMap, MipVolumeMap, NonPow2Conditional,
    NoProjectedBumpEnv, Perspective, Pow2, Projected, SquareOnly, TexRepeatNotScaledBySize, VolumeMap, VolumeMapPow2,
}

#[allow(non_upper_case_globals)] impl PTextureCaps {
    pub const None                          : PTextureCaps = PTextureCaps(0);
    pub const Alpha                         : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_ALPHA);
    pub const AlphaPalette                  : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_ALPHAPALETTE);
    pub const CubeMap                       : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_CUBEMAP);
    pub const CubeMapPow2                   : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_CUBEMAP_POW2);
    pub const MipCubeMap                    : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_MIPCUBEMAP);
    pub const MipMap                        : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_MIPMAP);
    pub const MipVolumeMap                  : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_MIPVOLUMEMAP);
    pub const NonPow2Conditional            : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_NONPOW2CONDITIONAL);
    pub const NoProjectedBumpEnv            : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_NOPROJECTEDBUMPENV);
    pub const Perspective                   : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_PERSPECTIVE);
    pub const Pow2                          : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_POW2);
    pub const Projected                     : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_PROJECTED);
    pub const SquareOnly                    : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_SQUAREONLY);
    pub const TexRepeatNotScaledBySize      : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_TEXREPEATNOTSCALEDBYSIZE);
    pub const VolumeMap                     : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_VOLUMEMAP);
    pub const VolumeMapPow2                 : PTextureCaps = PTextureCaps(D3DPTEXTURECAPS_VOLUMEMAP_POW2);
}

//#cpp2rust D3DPTEXTURECAPS_ALPHA                       = d3d::PTextureCaps::Alpha
//#cpp2rust D3DPTEXTURECAPS_ALPHAPALETTE                = d3d::PTextureCaps::AlphaPalette
//#cpp2rust D3DPTEXTURECAPS_CUBEMAP                     = d3d::PTextureCaps::CubeMap
//#cpp2rust D3DPTEXTURECAPS_CUBEMAP_POW2                = d3d::PTextureCaps::CubeMapPow2
//#cpp2rust D3DPTEXTURECAPS_MIPCUBEMAP                  = d3d::PTextureCaps::MipCubeMap
//#cpp2rust D3DPTEXTURECAPS_MIPMAP                      = d3d::PTextureCaps::MipMap
//#cpp2rust D3DPTEXTURECAPS_MIPVOLUMEMAP                = d3d::PTextureCaps::MipVolumeMap
//#cpp2rust D3DPTEXTURECAPS_NONPOW2CONDITIONAL          = d3d::PTextureCaps::NonPow2Conditional
//#cpp2rust D3DPTEXTURECAPS_NOPROJECTEDBUMPENV          = d3d::PTextureCaps::NoProjectedBumpEnv
//#cpp2rust D3DPTEXTURECAPS_PERSPECTIVE                 = d3d::PTextureCaps::Perspective
//#cpp2rust D3DPTEXTURECAPS_POW2                        = d3d::PTextureCaps::Pow2
//#cpp2rust D3DPTEXTURECAPS_PROJECTED                   = d3d::PTextureCaps::Projected
//#cpp2rust D3DPTEXTURECAPS_SQUAREONLY                  = d3d::PTextureCaps::SquareOnly
//#cpp2rust D3DPTEXTURECAPS_TEXREPEATNOTSCALEDBYSIZE    = d3d::PTextureCaps::TexRepeatNotScaledBySize
//#cpp2rust D3DPTEXTURECAPS_VOLUMEMAP                   = d3d::PTextureCaps::VolumeMap
//#cpp2rust D3DPTEXTURECAPS_VOLUMEMAP_POW2              = d3d::PTextureCaps::VolumeMapPow2
