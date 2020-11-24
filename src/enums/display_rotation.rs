#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplayrotation)\]
/// D3DDISPLAYROTATION
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DisplayRotation(D3DDISPLAYROTATION);

impl DisplayRotation {
    /// Convert a raw [D3DDISPLAYROTATION] value into a [DisplayRotation].  This is *probably* safe... probably....
    ///
    /// [D3DDISPLAYROTATION]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplayrotation
    pub const fn from_unchecked(displayrotation: D3DDISPLAYROTATION) -> Self { Self(displayrotation) }

    /// Convert a [DisplayRotation] into a raw [D3DDISPLAYROTATION].
    ///
    /// [D3DDISPLAYROTATION]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplayrotation
    pub const fn into(self) -> D3DDISPLAYROTATION { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl DisplayRotation {
    pub const Identity  : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_IDENTITY);
    pub const _90       : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_90);
    pub const _180      : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_180);
    pub const _270      : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_270);
}

impl Default for DisplayRotation {
    fn default() -> Self { DisplayRotation::Identity }
}

impl Debug for DisplayRotation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DisplayRotation::Identity   => write!(f, "DisplayRotation::Identity"),
            DisplayRotation::_90        => write!(f, "DisplayRotation::_90"),
            DisplayRotation::_180       => write!(f, "DisplayRotation::_180"),
            DisplayRotation::_270       => write!(f, "DisplayRotation::_270"),
            other                       => write!(f, "DisplayRotation({})", other.0),
        }
    }
}

impl From<DisplayRotation> for D3DDISPLAYROTATION {
    fn from(value: DisplayRotation) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DDISPLAYROTATION> for DisplayRotation {
    fn from(value: D3DDISPLAYROTATION) -> Self { Self(value) }
}
