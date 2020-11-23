use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};
use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplaymode)\]
/// D3DDISPLAYMODE
///
/// Describes the display mode.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct DisplayMode(D3DDISPLAYMODE);

impl DisplayMode {
    pub fn width(&self) -> u32 { self.0.Width }
    pub fn height(&self) -> u32 { self.0.Width }
    pub fn refresh_rate(&self) -> u32 { self.0.Width }
    pub fn format(&self) -> Format { Format::from_unchecked(self.0.Format) }
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self(D3DDISPLAYMODE {
            // reduce usage of the danger keyword by manually initializing
            Width:          0,
            Height:         0,
            RefreshRate:    0,
            Format:         D3DFMT_UNKNOWN,
        })
    }
}

impl Deref for DisplayMode {
    type Target = D3DDISPLAYMODE;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for DisplayMode {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Debug for DisplayMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("DisplayMode")
            .field("Width",         &self.0.Width)
            .field("Height",        &self.0.Height)
            .field("RefreshRate",   &self.0.RefreshRate)
            .field("Format",        &Format::from_unchecked(self.0.Format))
            .finish()
    }
}

impl From<D3DDISPLAYMODE> for DisplayMode {
    fn from(value: D3DDISPLAYMODE) -> Self { Self(value) }
}

impl From<DisplayMode> for D3DDISPLAYMODE {
    fn from(value: DisplayMode) -> Self { value.0 }
}
