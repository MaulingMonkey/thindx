#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplayrotation)\]
/// D3DDISPLAYROTATION
///
/// Specifies how the monitor being used to display a full-screen application is rotated.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DisplayRotation(D3DDISPLAYROTATION);

enumish! { DisplayRotation => D3DDISPLAYROTATION; Identity, _90, _180, _270 }

#[allow(non_upper_case_globals)] impl DisplayRotation { // These are enum-like
    pub const Identity  : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_IDENTITY); // 1
    pub const _90       : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_90);
    pub const _180      : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_180);
    pub const _270      : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_270);
}

impl Default for DisplayRotation {
    fn default() -> Self { DisplayRotation::Identity } // 1
}
