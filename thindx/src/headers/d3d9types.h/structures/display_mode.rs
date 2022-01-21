use crate::d3d9::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



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

test_layout! { DisplayMode => D3DDISPLAYMODE { width => Width, height => Height, refresh_rate => RefreshRate, format => Format } }

impl Deref    for DisplayMode { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DDISPLAYMODE; }
impl DerefMut for DisplayMode { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DDISPLAYMODE> for DisplayMode { fn from(value: D3DDISPLAYMODE) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<DisplayMode> for D3DDISPLAYMODE { fn from(value: DisplayMode   ) -> Self { unsafe { std::mem::transmute(value) } } }
