#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddecltype)\]
/// D3DDECLTYPE, but 8 bit
///
/// Defines a vertex declaration data type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DeclType8(u8);

impl DeclType8 {
    /// Convert a raw [D3DDECLTYPE] value into a [DeclType8].  This is *probably* safe... probably...
    ///
    /// [D3DDECLTYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddecltype
    pub const fn from_unchecked(decltype: D3DDECLTYPE) -> Self { Self(decltype as u8) }

    /// Convert a [DeclType8] into a raw [D3DDECLTYPE].
    ///
    /// [D3DDECLTYPE]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddecltype
    pub const fn into(self) -> D3DDECLTYPE { self.0 as _ }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl DeclType8 {
    pub const Float1            : DeclType8 = DeclType8(D3DDECLTYPE_FLOAT1 as u8);
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

impl Debug for DeclType8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DeclType8::Float1       => write!(f, "DeclType8::Float1"),
            DeclType8::Float2       => write!(f, "DeclType8::Float2"),
            DeclType8::Float3       => write!(f, "DeclType8::Float3"),
            DeclType8::Float4       => write!(f, "DeclType8::Float4"),
            DeclType8::Color        => write!(f, "DeclType8::Color"),
            DeclType8::UByte4       => write!(f, "DeclType8::UByte4"),
            DeclType8::Short2       => write!(f, "DeclType8::Short2"),
            DeclType8::Short4       => write!(f, "DeclType8::Short4"),
            DeclType8::UByte4N      => write!(f, "DeclType8::UByte4N"),
            DeclType8::Short2N      => write!(f, "DeclType8::Short2N"),
            DeclType8::Short4N      => write!(f, "DeclType8::Short4N"),
            DeclType8::UShort2N     => write!(f, "DeclType8::UShort2N"),
            DeclType8::UShort4N     => write!(f, "DeclType8::UShort4N"),
            DeclType8::UDec3        => write!(f, "DeclType8::UDec3"),
            DeclType8::Dec3N        => write!(f, "DeclType8::Dec3N"),
            DeclType8::Float16_2    => write!(f, "DeclType8::Float16_2"),
            DeclType8::Float16_4    => write!(f, "DeclType8::Float16_4"),
            DeclType8::Unused       => write!(f, "DeclType8::Unused"),
            other                   => write!(f, "DeclType8({})", other.0),
        }
    }
}

impl Default for DeclType8 {
    fn default() -> Self { DeclType8::Unused }
}

impl From<DeclType8> for D3DDECLTYPE {
    fn from(value: DeclType8) -> Self { value.0.into() }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DDECLTYPE> for DeclType8 {
    fn from(value: D3DDECLTYPE) -> Self { Self(value) }
}
