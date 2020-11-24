#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturefiltertype)\]
/// D3DTEXTUREFILTERTYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TextureFilterType(D3DTEXTUREFILTERTYPE);
pub type TexF = TextureFilterType;

impl TextureFilterType {
    /// Convert a raw [D3DTEXTUREFILTERTYPE] value into a [TextureFilterType].  This is *probably* safe... probably....
    ///
    /// [D3DTEXTUREFILTERTYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturefiltertype
    pub const fn from_unchecked(texturefiltertype: D3DTEXTUREFILTERTYPE) -> Self { Self(texturefiltertype) }

    /// Convert a [TextureFilterType] into a raw [D3DTEXTUREFILTERTYPE].
    ///
    /// [D3DTEXTUREFILTERTYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dtexturefiltertype
    pub const fn into(self) -> D3DTEXTUREFILTERTYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl TextureFilterType {
    pub const None              : TextureFilterType = TextureFilterType(D3DTEXF_NONE);
    pub const Point             : TextureFilterType = TextureFilterType(D3DTEXF_POINT);
    pub const Linear            : TextureFilterType = TextureFilterType(D3DTEXF_LINEAR);
    pub const Anisotropic       : TextureFilterType = TextureFilterType(D3DTEXF_ANISOTROPIC);
    pub const PyramidalQuad     : TextureFilterType = TextureFilterType(D3DTEXF_PYRAMIDALQUAD);
    pub const GaussianQuad      : TextureFilterType = TextureFilterType(D3DTEXF_GAUSSIANQUAD);
    pub const ConvolutionMono   : TextureFilterType = TextureFilterType(D3DTEXF_CONVOLUTIONMONO);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for TextureFilterType {
    fn default() -> Self { TextureFilterType::None }
}

impl Debug for TextureFilterType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            TextureFilterType::None             => write!(f, "TextureFilterType::None"),
            TextureFilterType::Point            => write!(f, "TextureFilterType::Point"),
            TextureFilterType::Linear           => write!(f, "TextureFilterType::Linear"),
            TextureFilterType::Anisotropic      => write!(f, "TextureFilterType::Anisotropic"),
            TextureFilterType::PyramidalQuad    => write!(f, "TextureFilterType::PyramidalQuad"),
            TextureFilterType::GaussianQuad     => write!(f, "TextureFilterType::GaussianQuad"),
            TextureFilterType::ConvolutionMono  => write!(f, "TextureFilterType::ConvolutionMono"),
            other                               => write!(f, "TextureFilterType({})", other.0),
        }
    }
}

impl From<TextureFilterType> for D3DTEXTUREFILTERTYPE {
    fn from(value: TextureFilterType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DTEXTUREFILTERTYPE> for TextureFilterType {
    fn from(value: D3DTEXTUREFILTERTYPE) -> Self { Self(value) }
}
