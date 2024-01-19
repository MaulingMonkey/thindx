use bytemuck::*;
use winapi::shared::d3d9types::*;

use std::os::raw::c_int as int;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dlocked-box)\]
/// D3DLOCKED_BOX
///
/// A locked 3-dimensional box/volume.
#[derive(Clone, Copy, Debug)]
#[derive(Zeroable)]
#[repr(C)] pub struct LockedBox {
    /// Byte offset from the left edge of one row, to the left edge of the next row.
    ///
    /// For block formats like DXT, this the pitch between rows of *blocks*.
    pub row_pitch:      int,

    /// Byte offset from the top-left of one slice, to the top-left of the next deepest slice.
    pub slice_pitch:    int,

    /// Pointer to the beginning of the volume/box.
    /// If a [d3d::Box] was provided to the lock_box\[_unchecked\] call, bits will be appropriately offset from the start of the volume.
    pub bits:           *mut u8,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    LockedBox => D3DLOCKED_BOX {
        row_pitch   => RowPitch,
        slice_pitch => SlicePitch,
        bits        => pBits,
    }
}

//#cpp2rust D3DLOCKED_BOX   = d3d::LockedBox
