use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;

use std::mem::size_of;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplaymodeex)\]
/// D3DDISPLAYMODEEX
///
/// Describes the display mode.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::get_display_mode]
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct DisplayModeEx {
    /// The size of this structure. This should always be set to `size_of::<DisplayModeEx>()`.
    pub size:               u32,

    /// Screen width, in pixels.
    pub width:              u32,

    /// Screen height, in pixels.
    pub height:             u32,

    /// Refresh rate, in refreshes per seconds (hz).
    /// The value of 0 indicates an adapter default.
    pub refresh_rate:       u32,

    /// Surface format of the display mode / screen.
    pub format:             Format,

    /// Indicates whether the scanline order is progressive or interlaced.
    pub scanline_ordering:  ScanlineOrdering,
}

impl Default for DisplayModeEx {
    fn default() -> Self {
        Self {
            size:               size_of::<Self>() as _,
            width:              0,
            height:             0,
            refresh_rate:       0,
            format:             Format::Unknown,
            scanline_ordering:  ScanlineOrdering::Progressive,
        }
    }
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    DisplayModeEx => D3DDISPLAYMODEEX {
        size                => Size,
        width               => Width,
        height              => Height,
        refresh_rate        => RefreshRate,
        format              => Format,
        scanline_ordering   => ScanLineOrdering
    }
}

//#cpp2rust D3DDISPLAYMODEEX = d3d::DisplayModeEx
