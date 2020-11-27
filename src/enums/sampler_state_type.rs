#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsamplerstatetype)\]
/// D3DSAMPLERSTATETYPE
///
/// Sampler states define texture sampling operations such as texture addressing and texture filtering.
/// Some sampler states set-up vertex processing, and some set-up pixel processing.
/// Sampler states can be saved and restored using stateblocks (see [State Blocks Save and Restore State (Direct3D 9)]).
///
/// [State Blocks Save and Restore State (Direct3D 9)]:     https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SamplerStateType(D3DSAMPLERSTATETYPE);
pub use SamplerStateType as Samp;

enumish! {
    Samp => D3DSAMPLERSTATETYPE;
    AddressU, AddressV, AddressW, BorderColor, MagFilter, MinFilter, MipFilter,
    MipMapLODBias, MaxMipLevel, MaxAnisotropy, SRGBTexture, ElementIndex, DMapOffset,
}

#[allow(non_upper_case_globals)] impl SamplerStateType { // These are enum-like
    pub const AddressU          : SamplerStateType = SamplerStateType(D3DSAMP_ADDRESSU); // 1
    pub const AddressV          : SamplerStateType = SamplerStateType(D3DSAMP_ADDRESSV);
    pub const AddressW          : SamplerStateType = SamplerStateType(D3DSAMP_ADDRESSW);
    pub const BorderColor       : SamplerStateType = SamplerStateType(D3DSAMP_BORDERCOLOR);
    pub const MagFilter         : SamplerStateType = SamplerStateType(D3DSAMP_MAGFILTER);
    pub const MinFilter         : SamplerStateType = SamplerStateType(D3DSAMP_MINFILTER);
    pub const MipFilter         : SamplerStateType = SamplerStateType(D3DSAMP_MIPFILTER);
    pub const MipMapLODBias     : SamplerStateType = SamplerStateType(D3DSAMP_MIPMAPLODBIAS);
    pub const MaxMipLevel       : SamplerStateType = SamplerStateType(D3DSAMP_MAXMIPLEVEL);
    pub const MaxAnisotropy     : SamplerStateType = SamplerStateType(D3DSAMP_MAXANISOTROPY);
    pub const SRGBTexture       : SamplerStateType = SamplerStateType(D3DSAMP_SRGBTEXTURE);
    pub const ElementIndex      : SamplerStateType = SamplerStateType(D3DSAMP_ELEMENTINDEX);
    pub const DMapOffset        : SamplerStateType = SamplerStateType(D3DSAMP_DMAPOFFSET);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for SamplerStateType {
    fn default() -> Self { SamplerStateType(0) }
}
