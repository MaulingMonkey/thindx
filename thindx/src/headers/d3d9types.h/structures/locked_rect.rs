use bytemuck::*;
use winapi::shared::d3d9types::*;

use std::os::raw::c_int as int;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dlocked-box)\]
/// D3DLOCKED_RECT
///
/// A locked 3-dimensional box/volume.
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)]
#[repr(C)] pub struct LockedRect {
    /// Number of bytes in one row of the surface.
    ///
    /// For block formats like DXT, this the pitch between rows of *blocks*.
    pub pitch:          int,

    /// Pointer to the beginning of the rectangle.
    /// If a [d3d::Rect] was provided to the lock_rect\[_unchecked\] call, bits will be appropriately offset from the start of the rect.
    pub bits:           *mut u8,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    LockedRect => D3DLOCKED_RECT {
        pitch       => Pitch,
        bits        => pBits,
    }
}

//#cpp2rust D3DLOCKED_RECT  = d3d::LockedRect
