#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dshademode)\]
/// D3DSHADEMODE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ShadeMode(D3DSHADEMODE);
pub type Shade = ShadeMode;

impl ShadeMode {
    /// Convert a raw [D3DSHADEMODE] value into a [ShadeMode].  This is *probably* safe... probably....
    ///
    /// [D3DSHADEMODE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dshademode
    pub const fn from_unchecked(shademode: D3DSHADEMODE) -> Self { Self(shademode) }

    /// Convert a [ShadeMode] into a raw [D3DSHADEMODE].
    ///
    /// [D3DSHADEMODE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dshademode
    pub const fn into(self) -> D3DSHADEMODE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl ShadeMode {
    pub const Flat      : ShadeMode = ShadeMode(D3DSHADE_FLAT);
    pub const Gouraud   : ShadeMode = ShadeMode(D3DSHADE_GOURAUD);
    pub const Phong     : ShadeMode = ShadeMode(D3DSHADE_PHONG);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for ShadeMode {
    fn default() -> Self { ShadeMode::Flat }
}

impl Debug for ShadeMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ShadeMode::Flat     => write!(f, "ShadeMode::Flat"),
            ShadeMode::Gouraud  => write!(f, "ShadeMode::Gouraud"),
            ShadeMode::Phong    => write!(f, "ShadeMode::Phong"),
            other               => write!(f, "ShadeMode({})", other.0),
        }
    }
}

impl From<ShadeMode> for D3DSHADEMODE {
    fn from(value: ShadeMode) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DSHADEMODE> for ShadeMode {
    fn from(value: D3DSHADEMODE) -> Self { Self(value) }
}
