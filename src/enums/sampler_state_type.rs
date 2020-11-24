#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsamplerstatetype)\]
/// D3DSAMPLERSTATETYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SamplerStateType(D3DSAMPLERSTATETYPE);
pub type Samp = SamplerStateType;

impl SamplerStateType {
    /// Convert a raw [D3DSAMPLERSTATETYPE] value into a [SamplerStateType].  This is *probably* safe... probably....
    ///
    /// [D3DSAMPLERSTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsamplerstatetype
    pub const fn from_unchecked(samplerstatetype: D3DSAMPLERSTATETYPE) -> Self { Self(samplerstatetype) }

    /// Convert a [SamplerStateType] into a raw [D3DSAMPLERSTATETYPE].
    ///
    /// [D3DSAMPLERSTATETYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsamplerstatetype
    pub const fn into(self) -> D3DSAMPLERSTATETYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl SamplerStateType {
    pub const AddressU          : SamplerStateType = SamplerStateType(D3DSAMP_ADDRESSU);
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

impl Debug for SamplerStateType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SamplerStateType::AddressU          => write!(f, "SamplerStateType::AddressU"),
            SamplerStateType::AddressV          => write!(f, "SamplerStateType::AddressV"),
            SamplerStateType::AddressW          => write!(f, "SamplerStateType::AddressW"),
            SamplerStateType::BorderColor       => write!(f, "SamplerStateType::BorderColor"),
            SamplerStateType::MagFilter         => write!(f, "SamplerStateType::MagFilter"),
            SamplerStateType::MinFilter         => write!(f, "SamplerStateType::MinFilter"),
            SamplerStateType::MipFilter         => write!(f, "SamplerStateType::MipFilter"),
            SamplerStateType::MipMapLODBias     => write!(f, "SamplerStateType::MipMapLODBias"),
            SamplerStateType::MaxMipLevel       => write!(f, "SamplerStateType::MaxMipLevel"),
            SamplerStateType::MaxAnisotropy     => write!(f, "SamplerStateType::MaxAnisotropy"),
            SamplerStateType::SRGBTexture       => write!(f, "SamplerStateType::SRGBTexture"),
            SamplerStateType::ElementIndex      => write!(f, "SamplerStateType::ElementIndex"),
            SamplerStateType::DMapOffset        => write!(f, "SamplerStateType::DMapOffset"),
            other                               => write!(f, "SamplerStateType({})", other.0),
        }
    }
}

impl From<SamplerStateType> for D3DSAMPLERSTATETYPE {
    fn from(value: SamplerStateType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DSAMPLERSTATETYPE> for SamplerStateType {
    fn from(value: D3DSAMPLERSTATETYPE) -> Self { Self(value) }
}
