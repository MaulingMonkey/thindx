#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclmethod)\]
/// D3DDECLMETHOD, but 8 bit
///
/// Defines the vertex declaration method which is a predefined operation performed by the tessellator (or any procedural geometry routine on the vertex data during tessellation).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DeclMethod8(u8);

impl DeclMethod8 {
    /// Convert a raw [D3DDECLMETHOD] value into a [DeclMethod8].  This is *probably* safe... probably...
    ///
    /// [D3DDECLMETHOD]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclmethod
    pub const fn from_unchecked(declmethod: D3DDECLMETHOD) -> Self { Self(declmethod as u8) }

    /// Convert a [DeclMethod8] into a raw [D3DDECLMETHOD].
    ///
    /// [D3DDECLMETHOD]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclmethod
    pub const fn into(self) -> D3DDECLMETHOD { self.0 as _ }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl DeclMethod8 {
    pub const Default           : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_DEFAULT as u8);
    pub const PartialU          : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_PARTIALU as u8);
    pub const PartialV          : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_PARTIALV as u8);
    pub const CrossUV           : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_CROSSUV as u8);
    pub const UV                : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_UV as u8);
    pub const Lookup            : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_LOOKUP as u8);
    pub const LookupPresampled  : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_LOOKUPPRESAMPLED as u8);
}

impl Debug for DeclMethod8 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DeclMethod8::Default            => write!(f, "DeclMethod8::Default"),
            DeclMethod8::PartialU           => write!(f, "DeclMethod8::PartialU"),
            DeclMethod8::PartialV           => write!(f, "DeclMethod8::PartialV"),
            DeclMethod8::CrossUV            => write!(f, "DeclMethod8::CrossUV"),
            DeclMethod8::UV                 => write!(f, "DeclMethod8::UV"),
            DeclMethod8::Lookup             => write!(f, "DeclMethod8::Lookup"),
            DeclMethod8::LookupPresampled   => write!(f, "DeclMethod8::LookupPresampled"),
            other                           => write!(f, "DeclMethod8({})", other.0),
        }
    }
}

impl Default for DeclMethod8 {
    fn default() -> Self { DeclMethod8::Default }
}

impl From<DeclMethod8> for D3DDECLMETHOD {
    fn from(value: DeclMethod8) -> Self { value.0.into() }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DDECLMETHOD> for DeclMethod8 {
    fn from(value: D3DDECLMETHOD) -> Self { Self(value) }
}
