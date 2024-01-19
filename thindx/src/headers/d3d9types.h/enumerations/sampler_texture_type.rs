#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dsampler-texture-type)\]
/// D3DSAMPLER_TEXTURE_TYPE
///
/// Defines the sampler texture types for vertex shaders.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct SamplerTextureType(D3DSAMPLER_TEXTURE_TYPE);
pub use SamplerTextureType as STT;

enumish! { STT => D3DSAMPLER_TEXTURE_TYPE; Unknown, _2D, Cube, Volume }

#[allow(non_upper_case_globals)] impl SamplerTextureType { // These are enum-like
    pub const Unknown       : SamplerTextureType = SamplerTextureType(D3DSTT_UNKNOWN); // 0
    pub const _2D           : SamplerTextureType = SamplerTextureType(D3DSTT_2D);
    pub const Regular       : SamplerTextureType = SamplerTextureType(D3DSTT_2D);
    pub const Cube          : SamplerTextureType = SamplerTextureType(D3DSTT_CUBE);
    pub const Volume        : SamplerTextureType = SamplerTextureType(D3DSTT_VOLUME);
}

//#cpp2rust D3DSAMPLER_TEXTURE_TYPE = d3d::SamplerTextureType

//#cpp2rust D3DSTT_UNKNOWN          = d3d::STT::Unknown
//#cpp2rust D3DSTT_2D               = d3d::STT::_2D
//#cpp2rust D3DSTT_2D               = d3d::STT::Regular
//#cpp2rust D3DSTT_CUBE             = d3d::STT::Cube
//#cpp2rust D3DSTT_VOLUME           = d3d::STT::Volume
