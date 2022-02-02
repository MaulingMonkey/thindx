#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplayrotation)\]
/// D3DDISPLAYROTATION
///
/// Specifies how the monitor being used to display a full-screen application is rotated.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct DisplayRotation(D3DDISPLAYROTATION);

enumish! { DisplayRotation => D3DDISPLAYROTATION; default: Identity != 0; Identity, _90, _180, _270 }

#[allow(non_upper_case_globals)] impl DisplayRotation { // These are enum-like
    pub const Identity  : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_IDENTITY); // 1
    pub const _90       : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_90);
    pub const _180      : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_180);
    pub const _270      : DisplayRotation = DisplayRotation(D3DDISPLAYROTATION_270);
}

//#cpp2rust D3DDISPLAYROTATION              = d3d::DisplayRotation

//#cpp2rust D3DDISPLAYROTATION_IDENTITY     = d3d::DisplayRotation::Identity
//#cpp2rust D3DDISPLAYROTATION_90           = d3d::DisplayRotation::_90
//#cpp2rust D3DDISPLAYROTATION_180          = d3d::DisplayRotation::_180
//#cpp2rust D3DDISPLAYROTATION_270          = d3d::DisplayRotation::_270
