#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddeclmethod)\]
/// D3DDECLMETHOD, but 8 bit
///
/// Defines the vertex declaration method which is a predefined operation performed by the tessellator (or any procedural geometry routine on the vertex data during tessellation).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DeclMethod8(u8);

enumish! { DeclMethod8 => D3DDECLMETHOD; Default, PartialU, PartialV, CrossUV, UV, Lookup, LookupPresampled }

#[allow(non_upper_case_globals)] impl DeclMethod8 { // These are enum-like
    pub const Default           : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_DEFAULT as u8); // 0
    pub const PartialU          : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_PARTIALU as u8);
    pub const PartialV          : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_PARTIALV as u8);
    pub const CrossUV           : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_CROSSUV as u8);
    pub const UV                : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_UV as u8);
    pub const Lookup            : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_LOOKUP as u8);
    pub const LookupPresampled  : DeclMethod8 = DeclMethod8(D3DDECLMETHOD_LOOKUPPRESAMPLED as u8);
}

impl Default for DeclMethod8 {
    fn default() -> Self { DeclMethod8::Default } // 0
}
