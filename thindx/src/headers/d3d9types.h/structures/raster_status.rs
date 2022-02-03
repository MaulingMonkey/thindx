use crate::ctypes::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;
use winapi::shared::minwindef::UINT;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3draster-status)\]
/// D3DRASTER_STATUS
///
/// Describes the raster status.
#[derive(Clone, Copy, Debug, Default)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct RasterStatus {
    /// `true` if the raster is in the vertical blank period.
    /// `false` if the raster is not in the vertical blank period.
    pub in_vblank:  BOOL,

    /// If [in_vblank](Self::in_vblank) is `false`, then this value is an integer roughly corresponding to the current scan line painted by the raster.
    /// Scan lines are numbered in the same way as Direct3D surface coordinates: `0` is the top of the primary surface, extending to the value (`height of the surface - 1`) at the bottom of the display.
    ///
    /// If [in_vblank](Self::in_vblank) is `true`, then this value is set to zero and can be ignored.
    pub scan_line:  UINT,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    RasterStatus => D3DRASTER_STATUS {
        in_vblank => InVBlank,
        scan_line => ScanLine,
    }
}

//#cpp2rust D3DRASTER_STATUS = d3d::RasterStatus
