#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddecltype)\]
/// D3DDECLTYPE, but 8 bit
///
/// Defines a vertex declaration data type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DeclType8(u8);

enumish! {
    DeclType8 => D3DDECLTYPE;
    Float1, Float2, Float3, Float4, Color, UByte4, Short2, Short4,
    UByte4N, Short2N, Short4N, UShort2N, UShort4N, UDec3, Dec3N, Float16_2, Float16_4, Unused,
}

#[allow(non_upper_case_globals)] impl DeclType8 { // These are enum-like
    pub const Float1            : DeclType8 = DeclType8(D3DDECLTYPE_FLOAT1 as u8); // 0
    pub const Float2            : DeclType8 = DeclType8(D3DDECLTYPE_FLOAT2 as u8);
    pub const Float3            : DeclType8 = DeclType8(D3DDECLTYPE_FLOAT3 as u8);
    pub const Float4            : DeclType8 = DeclType8(D3DDECLTYPE_FLOAT4 as u8);
    pub const Color             : DeclType8 = DeclType8(D3DDECLTYPE_D3DCOLOR as u8);
    pub const UByte4            : DeclType8 = DeclType8(D3DDECLTYPE_UBYTE4 as u8);
    pub const Short2            : DeclType8 = DeclType8(D3DDECLTYPE_SHORT2 as u8);
    pub const Short4            : DeclType8 = DeclType8(D3DDECLTYPE_SHORT4 as u8);
    pub const UByte4N           : DeclType8 = DeclType8(D3DDECLTYPE_UBYTE4N as u8);
    pub const Short2N           : DeclType8 = DeclType8(D3DDECLTYPE_SHORT2N as u8);
    pub const Short4N           : DeclType8 = DeclType8(D3DDECLTYPE_SHORT4N as u8);
    pub const UShort2N          : DeclType8 = DeclType8(D3DDECLTYPE_USHORT2N as u8);
    pub const UShort4N          : DeclType8 = DeclType8(D3DDECLTYPE_USHORT4N as u8);
    pub const UDec3             : DeclType8 = DeclType8(D3DDECLTYPE_UDEC3 as u8);
    pub const Dec3N             : DeclType8 = DeclType8(D3DDECLTYPE_DEC3N as u8);
    pub const Float16_2         : DeclType8 = DeclType8(D3DDECLTYPE_FLOAT16_2 as u8);
    pub const Float16_4         : DeclType8 = DeclType8(D3DDECLTYPE_FLOAT16_4 as u8);
    pub const Unused            : DeclType8 = DeclType8(D3DDECLTYPE_UNUSED as u8);
}

impl Default for DeclType8 {
    fn default() -> Self { DeclType8::Unused } // 17 - and Float1==0 is unused!
}

//#cpp2rust D3DDECLTYPE             = d3d::DeclType8
//#cpp2rust D3DDECLTYPE_FLOAT1      = d3d::DeclType8::Float1
//#cpp2rust D3DDECLTYPE_FLOAT2      = d3d::DeclType8::Float2
//#cpp2rust D3DDECLTYPE_FLOAT3      = d3d::DeclType8::Float3
//#cpp2rust D3DDECLTYPE_FLOAT4      = d3d::DeclType8::Float4
//#cpp2rust D3DDECLTYPE_D3DCOLOR    = d3d::DeclType8::Color
//#cpp2rust D3DDECLTYPE_UBYTE4      = d3d::DeclType8::UByte4
//#cpp2rust D3DDECLTYPE_SHORT2      = d3d::DeclType8::Short2
//#cpp2rust D3DDECLTYPE_SHORT4      = d3d::DeclType8::Short4
//#cpp2rust D3DDECLTYPE_UBYTE4N     = d3d::DeclType8::UByte4N
//#cpp2rust D3DDECLTYPE_SHORT2N     = d3d::DeclType8::Short2N
//#cpp2rust D3DDECLTYPE_SHORT4N     = d3d::DeclType8::Short4N
//#cpp2rust D3DDECLTYPE_USHORT2N    = d3d::DeclType8::UShort2N
//#cpp2rust D3DDECLTYPE_USHORT4N    = d3d::DeclType8::UShort4N
//#cpp2rust D3DDECLTYPE_UDEC3       = d3d::DeclType8::UDec3
//#cpp2rust D3DDECLTYPE_DEC3N       = d3d::DeclType8::Dec3N
//#cpp2rust D3DDECLTYPE_FLOAT16_2   = d3d::DeclType8::Float16_2
//#cpp2rust D3DDECLTYPE_FLOAT16_4   = d3d::DeclType8::Float16_4
//#cpp2rust D3DDECLTYPE_UNUSED      = d3d::DeclType8::Unused
