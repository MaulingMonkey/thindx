#[allow(unused_imports)] use crate::*;
use crate::d3d9::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsamplerstatetype)\]
/// D3DSAMPLERSTATETYPE + Value
///
/// Sampler states define texture sampling operations such as texture addressing and texture filtering.
/// Some sampler states set-up vertex processing, and some set-up pixel processing.
/// Sampler states can be saved and restored using stateblocks (see [State Blocks Save and Restore State (Direct3D 9)]).
///
/// [State Blocks Save and Restore State (Direct3D 9)]:     https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SamplerStateValue {
    AddressU(TextureAddress),
    AddressV(TextureAddress),
    AddressW(TextureAddress),
    BorderColor(d3d::Color),
    MagFilter(TextureFilterType),
    MinFilter(TextureFilterType),
    MipFilter(TextureFilterType),
    MipMapLODBias(u32),
    MaxMipLevel(u32),
    MaxAnisotropy(u32),
    SRGBTexture(bool),
    ElementIndex(u32),
    DMapOffset(u32),
}

pub use SamplerStateValue as SampV;

impl SamplerStateValue {
    pub fn ty(&self) -> SamplerStateType { self.ty_value().0 }
    pub fn value(&self) -> u32 { self.ty_value().1 }
    pub fn ty_value(&self) -> (SamplerStateType, u32) {
        match self {
            &Self::AddressU(v)      => (SamplerStateType::AddressU,         v.into()),
            &Self::AddressV(v)      => (SamplerStateType::AddressV,         v.into()),
            &Self::AddressW(v)      => (SamplerStateType::AddressW,         v.into()),
            &Self::BorderColor(v)   => (SamplerStateType::BorderColor,      v.into()),
            &Self::MagFilter(v)     => (SamplerStateType::MagFilter,        v.into()),
            &Self::MinFilter(v)     => (SamplerStateType::MinFilter,        v.into()),
            &Self::MipFilter(v)     => (SamplerStateType::MipFilter,        v.into()),
            &Self::MipMapLODBias(v) => (SamplerStateType::MipMapLODBias,    v.into()),
            &Self::MaxMipLevel(v)   => (SamplerStateType::MaxMipLevel,      v.into()),
            &Self::MaxAnisotropy(v) => (SamplerStateType::MaxAnisotropy,    v.into()),
            &Self::SRGBTexture(v)   => (SamplerStateType::SRGBTexture,      v.into()),
            &Self::ElementIndex(v)  => (SamplerStateType::ElementIndex,     v.into()),
            &Self::DMapOffset(v)    => (SamplerStateType::DMapOffset,       v.into()),
        }
    }
}
