#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsampler-texture-type)\]
/// D3DSAMPLER_TEXTURE_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SamplerTextureType(D3DSAMPLER_TEXTURE_TYPE);
pub type STT = SamplerTextureType;

impl SamplerTextureType {
    /// Convert a raw [D3DSAMPLER_TEXTURE_TYPE] value into a [SamplerTextureType].  This is *probably* safe... probably....
    ///
    /// [D3DSAMPLER_TEXTURE_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsampler-texture-type
    pub const fn from_unchecked(samplertexturetype: D3DSAMPLER_TEXTURE_TYPE) -> Self { Self(samplertexturetype) }

    /// Convert a [SamplerTextureType] into a raw [D3DSAMPLER_TEXTURE_TYPE].
    ///
    /// [D3DSAMPLER_TEXTURE_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsampler-texture-type
    pub const fn into(self) -> D3DSAMPLER_TEXTURE_TYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl SamplerTextureType {
    pub const Unknown       : SamplerTextureType = SamplerTextureType(D3DSTT_UNKNOWN);
    pub const _2D           : SamplerTextureType = SamplerTextureType(D3DSTT_2D);
    pub const Regular       : SamplerTextureType = SamplerTextureType(D3DSTT_2D);
    pub const Cube          : SamplerTextureType = SamplerTextureType(D3DSTT_CUBE);
    pub const Volume        : SamplerTextureType = SamplerTextureType(D3DSTT_VOLUME);
}

#[doc(hidden)] impl SamplerTextureType {
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for SamplerTextureType {
    fn default() -> Self { SamplerTextureType::Unknown }
}

impl Debug for SamplerTextureType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SamplerTextureType::Unknown => write!(f, "SamplerTextureType::Unknown"),
            SamplerTextureType::_2D     => write!(f, "SamplerTextureType::_2D"),
            SamplerTextureType::Cube    => write!(f, "SamplerTextureType::Cube"),
            SamplerTextureType::Volume  => write!(f, "SamplerTextureType::Volume"),
            other                       => write!(f, "SamplerTextureType({})", other.0),
        }
    }
}

impl From<SamplerTextureType> for D3DSAMPLER_TEXTURE_TYPE {
    fn from(value: SamplerTextureType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DSAMPLER_TEXTURE_TYPE> for SamplerTextureType {
    fn from(value: D3DSAMPLER_TEXTURE_TYPE) -> Self { Self(value) }
}
