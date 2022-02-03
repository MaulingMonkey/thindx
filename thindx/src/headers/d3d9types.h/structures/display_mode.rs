use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplaymode)\]
/// D3DDISPLAYMODE
///
/// Describes the display mode.
///
/// ### See Also
/// *   [IDirect3DDevice9Ext::get_display_mode]
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct DisplayMode {
    /// Screen width, in pixels.
    pub width:          u32,

    /// Screen height, in pixels.
    pub height:         u32,

    /// Refresh rate, in refreshes per seconds (hz).
    /// The value of 0 indicates an adapter default.
    pub refresh_rate:   u32,

    /// Surface format of the display mode / screen.
    pub format:         Format,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    DisplayMode => D3DDISPLAYMODE {
        width           => Width,
        height          => Height,
        refresh_rate    => RefreshRate,
        format          => Format
    }
}

//#cpp2rust D3DDISPLAYMODE = d3d::DisplayMode
