#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;
type D3DFVF = u32; // there's no actual type

use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfvf)\]
/// DWORD / D3DFVF_*
///
/// Flexible Vertex Format Constants, or FVF codes, are used to describe the contents of vertices interleaved in a single data stream that will be processed by the fixed-function pipeline.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FVF(D3DFVF);

flags! {
    FVF => D3DFVF;
    LastBetaUByte4, LastBetaD3DColor,
    Reserved2, Reserved0,
    Tex8, Tex7, Tex6, Tex5, Tex4, Tex3, Tex2, Tex1,// Tex0,
    XYZW, XYZB5, XYZB4, XYZB3, XYZB2, XYZB1, XYZRHW, XYZ,
    Specular, PSize, Normal, Diffuse, None,
}

#[allow(non_upper_case_globals)] impl FVF { // These are enum-like
    pub const None              : FVF = FVF(0);

    // Vertex Data Flags

    pub const Diffuse           : FVF = FVF(D3DFVF_DIFFUSE);
    pub const Normal            : FVF = FVF(D3DFVF_NORMAL);
    pub const PSize             : FVF = FVF(D3DFVF_PSIZE);
    pub const Specular          : FVF = FVF(D3DFVF_SPECULAR);
    pub const XYZ               : FVF = FVF(D3DFVF_XYZ);
    pub const XYZRHW            : FVF = FVF(D3DFVF_XYZRHW);
    pub const XYZB1             : FVF = FVF(D3DFVF_XYZB1);
    pub const XYZB2             : FVF = FVF(D3DFVF_XYZB2);
    pub const XYZB3             : FVF = FVF(D3DFVF_XYZB3);
    pub const XYZB4             : FVF = FVF(D3DFVF_XYZB4);
    pub const XYZB5             : FVF = FVF(D3DFVF_XYZB5);
    pub const XYZW              : FVF = FVF(D3DFVF_XYZW);

    // Texture Flags

    pub const Tex0              : FVF = FVF(D3DFVF_TEX0);
    pub const Tex1              : FVF = FVF(D3DFVF_TEX1);
    pub const Tex2              : FVF = FVF(D3DFVF_TEX2);
    pub const Tex3              : FVF = FVF(D3DFVF_TEX3);
    pub const Tex4              : FVF = FVF(D3DFVF_TEX4);
    pub const Tex5              : FVF = FVF(D3DFVF_TEX5);
    pub const Tex6              : FVF = FVF(D3DFVF_TEX6);
    pub const Tex7              : FVF = FVF(D3DFVF_TEX7);
    pub const Tex8              : FVF = FVF(D3DFVF_TEX8);
    // D3DFVF_TEXCOORDSIZEN(coordIndex)

    // Mask Flags

    pub const PositionMask      : FVF = FVF(D3DFVF_POSITION_MASK);
    pub const Reserved0         : FVF = FVF(D3DFVF_RESERVED0);
    pub const Reserved2         : FVF = FVF(D3DFVF_RESERVED2);
    pub const TexCountMask      : FVF = FVF(D3DFVF_TEXCOUNT_MASK);

    // Miscellanieous Flags

    pub const LastBetaD3DColor  : FVF = FVF(D3DFVF_LASTBETA_D3DCOLOR);
    pub const LastBetaUByte4    : FVF = FVF(D3DFVF_LASTBETA_UBYTE4);
    pub const TexCountShift     : u32 = D3DFVF_TEXCOUNT_SHIFT;
}

impl Default for FVF {
    fn default() -> Self { FVF::None }
}

impl Deref for FVF {
    type Target = D3DFVF;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for FVF {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

//#cpp2rust D3DFVF                      = d3d::FVF
//#cpp2rust D3DFVF_DIFFUSE              = d3d::FVF::Diffuse
//#cpp2rust D3DFVF_NORMAL               = d3d::FVF::Normal
//#cpp2rust D3DFVF_PSIZE                = d3d::FVF::PSize
//#cpp2rust D3DFVF_SPECULAR             = d3d::FVF::Specular
//#cpp2rust D3DFVF_XYZ                  = d3d::FVF::XYZ
//#cpp2rust D3DFVF_XYZRHW               = d3d::FVF::XYZRHW
//#cpp2rust D3DFVF_XYZB1                = d3d::FVF::XYZB1
//#cpp2rust D3DFVF_XYZB2                = d3d::FVF::XYZB2
//#cpp2rust D3DFVF_XYZB3                = d3d::FVF::XYZB3
//#cpp2rust D3DFVF_XYZB4                = d3d::FVF::XYZB4
//#cpp2rust D3DFVF_XYZB5                = d3d::FVF::XYZB5
//#cpp2rust D3DFVF_XYZW                 = d3d::FVF::XYZW
//#cpp2rust D3DFVF_TEX0                 = d3d::FVF::Tex0
//#cpp2rust D3DFVF_TEX1                 = d3d::FVF::Tex1
//#cpp2rust D3DFVF_TEX2                 = d3d::FVF::Tex2
//#cpp2rust D3DFVF_TEX3                 = d3d::FVF::Tex3
//#cpp2rust D3DFVF_TEX4                 = d3d::FVF::Tex4
//#cpp2rust D3DFVF_TEX5                 = d3d::FVF::Tex5
//#cpp2rust D3DFVF_TEX6                 = d3d::FVF::Tex6
//#cpp2rust D3DFVF_TEX7                 = d3d::FVF::Tex7
//#cpp2rust D3DFVF_TEX8                 = d3d::FVF::Tex8
//#cpp2rust D3DFVF_POSITION_MASK        = d3d::FVF::PositionMask
//#cpp2rust D3DFVF_RESERVED0            = d3d::FVF::Reserved0
//#cpp2rust D3DFVF_RESERVED2            = d3d::FVF::Reserved2
//#cpp2rust D3DFVF_TEXCOUNT_MASK        = d3d::FVF::TexCountMask
//#cpp2rust D3DFVF_LASTBETA_D3DCOLOR    = d3d::FVF::LastBetaD3DColor
//#cpp2rust D3DFVF_LASTBETA_UBYTE4      = d3d::FVF::LastBetaUByte4
//#cpp2rust D3DFVF_TEXCOUNT_SHIFT       = d3d::FVF::TexCountShift
