#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclusage)\]
/// D3DDECLUSAGE, but 8 bit
///
/// Identifies the intended use of vertex data.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DeclUsage8(u8);

impl DeclUsage8 {
    /// Convert a raw [D3DDECLUSAGE] value into a [DeclUsage8].  This is *probably* safe... probably...
    ///
    /// [D3DDECLUSAGE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclusage
    pub const fn from_unchecked(declusage: D3DDECLUSAGE) -> Self { Self(declusage as u8) }

    /// Convert a [DeclUsage8] into a raw [D3DDECLUSAGE].
    ///
    /// [D3DDECLUSAGE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclusage
    pub const fn into(self) -> D3DDECLUSAGE { self.0 as _ }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl DeclUsage8 {
    pub const Position          : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_POSITION as u8);
    pub const BlendWeight       : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_BLENDWEIGHT as u8);
    pub const BlendIndices      : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_BLENDINDICES as u8);
    pub const Normal            : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_NORMAL as u8);
    pub const PSize             : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_PSIZE as u8);
    pub const TexCoord          : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_TEXCOORD as u8);
    pub const Tangent           : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_TANGENT as u8);
    pub const Binormal          : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_BINORMAL as u8);
    pub const TessFactor        : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_TESSFACTOR as u8);
    pub const PositionT         : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_POSITIONT as u8);
    pub const Color             : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_COLOR as u8);
    pub const Fog               : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_FOG as u8);
    pub const Depth             : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_DEPTH as u8);
    pub const Sample            : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_SAMPLE as u8);
}

impl Debug for DeclUsage8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DeclUsage8::Position        => write!(f, "DeclUsage8::Position"),
            DeclUsage8::BlendWeight     => write!(f, "DeclUsage8::BlendWeight"),
            DeclUsage8::BlendIndices    => write!(f, "DeclUsage8::BlendIndices"),
            DeclUsage8::Normal          => write!(f, "DeclUsage8::Normal"),
            DeclUsage8::PSize           => write!(f, "DeclUsage8::PSize"),
            DeclUsage8::TexCoord        => write!(f, "DeclUsage8::TexCoord"),
            DeclUsage8::Tangent         => write!(f, "DeclUsage8::Tangent"),
            DeclUsage8::Binormal        => write!(f, "DeclUsage8::Binormal"),
            DeclUsage8::TessFactor      => write!(f, "DeclUsage8::TessFactor"),
            DeclUsage8::PositionT       => write!(f, "DeclUsage8::PositionT"),
            DeclUsage8::Color           => write!(f, "DeclUsage8::Color"),
            DeclUsage8::Fog             => write!(f, "DeclUsage8::Fog"),
            DeclUsage8::Depth           => write!(f, "DeclUsage8::Depth"),
            DeclUsage8::Sample          => write!(f, "DeclUsage8::Sample"),
            other                       => write!(f, "DeclUsage8({})", other.0),
        }
    }
}

impl Default for DeclUsage8 {
    fn default() -> Self { DeclUsage8::Position }
}

impl From<DeclUsage8> for D3DDECLUSAGE {
    fn from(value: DeclUsage8) -> Self { value.0.into() }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DDECLUSAGE> for DeclUsage8 {
    fn from(value: D3DDECLUSAGE) -> Self { Self(value) }
}
