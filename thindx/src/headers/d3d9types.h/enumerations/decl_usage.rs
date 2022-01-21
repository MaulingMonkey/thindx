#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclusage)\]
/// D3DDECLUSAGE, but 8 bit
///
/// Identifies the intended use of vertex data.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DeclUsage8(u8);

enumish! {
    DeclUsage8 => D3DDECLUSAGE;
    Position, BlendWeight, BlendIndices, Normal, PSize, TexCoord, Tangent,
    Binormal, TessFactor, PositionT, Color, Fog, Depth, Sample,
}

#[allow(non_upper_case_globals)] impl DeclUsage8 { // These are enum-like
    pub const Position          : DeclUsage8 = DeclUsage8(D3DDECLUSAGE_POSITION as u8); // 0
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

impl Default for DeclUsage8 {
    fn default() -> Self { DeclUsage8::Position } // 0
}

//#cpp2rust D3DDECLUSAGE                = d3d::DeclUsage8
//#cpp2rust D3DDECLUSAGE_POSITION       = d3d::DeclUsage8::Position
//#cpp2rust D3DDECLUSAGE_BLENDWEIGHT    = d3d::DeclUsage8::BlendWeight
//#cpp2rust D3DDECLUSAGE_BLENDINDICES   = d3d::DeclUsage8::BlendIndices
//#cpp2rust D3DDECLUSAGE_NORMAL         = d3d::DeclUsage8::Normal
//#cpp2rust D3DDECLUSAGE_PSIZE          = d3d::DeclUsage8::PSize
//#cpp2rust D3DDECLUSAGE_TEXCOORD       = d3d::DeclUsage8::TexCoord
//#cpp2rust D3DDECLUSAGE_TANGENT        = d3d::DeclUsage8::Tangent
//#cpp2rust D3DDECLUSAGE_BINORMAL       = d3d::DeclUsage8::Binormal
//#cpp2rust D3DDECLUSAGE_TESSFACTOR     = d3d::DeclUsage8::TessFactor
//#cpp2rust D3DDECLUSAGE_POSITIONT      = d3d::DeclUsage8::PositionT
//#cpp2rust D3DDECLUSAGE_COLOR          = d3d::DeclUsage8::Color
//#cpp2rust D3DDECLUSAGE_FOG            = d3d::DeclUsage8::Fog
//#cpp2rust D3DDECLUSAGE_DEPTH          = d3d::DeclUsage8::Depth
//#cpp2rust D3DDECLUSAGE_SAMPLE         = d3d::DeclUsage8::Sample
