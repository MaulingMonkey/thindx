#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9caps/ns-d3d9caps-d3dcaps9)\]
/// D3DPTEXTURECAPS_*
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Pod, Zeroable)] #[repr(transparent)] pub struct PTextureCaps(DWORD);

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
