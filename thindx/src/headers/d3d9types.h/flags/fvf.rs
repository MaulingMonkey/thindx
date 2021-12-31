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
