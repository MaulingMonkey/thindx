use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;
use winapi::shared::minwindef::UINT;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplaymodefilter)\]
/// D3DDISPLAYMODEFILTER
///
/// Specifies types of display modes to filter out.
///
/// ### See Also
/// *   [d3d9::IDirect3D9ExExt::enum_adapter_modes_ex]
/// *   [d3d9::IDirect3D9ExExt::get_adapter_mode_count_ex]
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct DisplayModeFilter {
    /// Size of this structure.
    /// This should always be set to `size_of::<DisplayModeFilter>()`.
    pub size:               UINT,

    /// The display mode to filter out.
    pub format:             Format,

    /// Wheither the scanline ordering is interlaced or progressive.
    pub scanline_ordering:  ScanlineOrdering,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    DisplayModeFilter => D3DDISPLAYMODEFILTER {
        size                => Size,
        format              => Format,
        scanline_ordering   => ScanLineOrdering,
    }
}

//#cpp2rust D3DDISPLAYMODEFILTER = d3d::DisplayModeFilter
