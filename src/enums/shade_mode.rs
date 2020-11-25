#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dshademode)\]
/// D3DSHADEMODE
///
/// Defines constants that describe the supported shading modes.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShadeMode(D3DSHADEMODE);
pub type Shade = ShadeMode;

enumish! { Shade => D3DSHADEMODE; Flat, Gouraud, Phong }

#[allow(non_upper_case_globals)] impl ShadeMode { // These are enum-like
    pub const Flat      : ShadeMode = ShadeMode(D3DSHADE_FLAT);
    pub const Gouraud   : ShadeMode = ShadeMode(D3DSHADE_GOURAUD);
    pub const Phong     : ShadeMode = ShadeMode(D3DSHADE_PHONG);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for ShadeMode {
    fn default() -> Self { ShadeMode::Flat } // 1
}
