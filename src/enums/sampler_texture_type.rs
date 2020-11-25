#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsampler-texture-type)\]
/// D3DSAMPLER_TEXTURE_TYPE
///
/// Defines the sampler texture types for vertex shaders.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SamplerTextureType(D3DSAMPLER_TEXTURE_TYPE);
pub type STT = SamplerTextureType;

enumish! { STT => D3DSAMPLER_TEXTURE_TYPE; Unknown, _2D, Cube, Volume }

#[allow(non_upper_case_globals)] impl SamplerTextureType { // These are enum-like
    pub const Unknown       : SamplerTextureType = SamplerTextureType(D3DSTT_UNKNOWN); // Supposedly D3DSP_TEXTURETYPE_SHIFT
    pub const _2D           : SamplerTextureType = SamplerTextureType(D3DSTT_2D);
    pub const Regular       : SamplerTextureType = SamplerTextureType(D3DSTT_2D);
    pub const Cube          : SamplerTextureType = SamplerTextureType(D3DSTT_CUBE);
    pub const Volume        : SamplerTextureType = SamplerTextureType(D3DSTT_VOLUME);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for SamplerTextureType {
    fn default() -> Self { SamplerTextureType::Unknown }
}
