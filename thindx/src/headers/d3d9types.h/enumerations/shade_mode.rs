#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dshademode)\]
/// D3DSHADEMODE
///
/// Defines constants that describe the supported shading modes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ShadeMode(D3DSHADEMODE);
pub use ShadeMode as Shade;

enumish! { Shade => D3DSHADEMODE; Flat, Gouraud, Phong }

#[allow(non_upper_case_globals)] impl ShadeMode { // These are enum-like
    pub const Flat      : ShadeMode = ShadeMode(D3DSHADE_FLAT);
    pub const Gouraud   : ShadeMode = ShadeMode(D3DSHADE_GOURAUD);
    pub const Phong     : ShadeMode = ShadeMode(D3DSHADE_PHONG);
}

impl ShadeMode {
    pub const fn zeroed() -> Self { Self(0) }
}

//#cpp2rust D3DSHADEMODE        = d3d::ShadeMode
//#cpp2rust D3DSHADE_FLAT       = d3d::Shade::Flat
//#cpp2rust D3DSHADE_GOURAUD    = d3d::Shade::Gouraud
//#cpp2rust D3DSHADE_PHONG      = d3d::Shade::Phong
