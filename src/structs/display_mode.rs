use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddisplaymode)\]
/// D3DDISPLAYMODE
///
/// Describes the display mode.
#[derive(Clone, Copy, Default, Debug)]
#[repr(C)] pub struct DisplayMode {
    pub width:          u32,
    pub height:         u32,
    pub refresh_rate:   u32,
    pub format:         Format,
}

test_layout! { DisplayMode => unsafe D3DDISPLAYMODE { width => Width, height => Height, refresh_rate => RefreshRate, format => Format } }

impl Deref    for DisplayMode { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DDISPLAYMODE; }
impl DerefMut for DisplayMode { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DDISPLAYMODE> for DisplayMode { fn from(value: D3DDISPLAYMODE) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<DisplayMode> for D3DDISPLAYMODE { fn from(value: DisplayMode   ) -> Self { unsafe { std::mem::transmute(value) } } }



impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdisplaymode)\]
    /// IDirect3DDevice9::GetDisplayMode
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thin3d9?)
    /// *   Ok([DisplayMode])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let mode : DisplayMode = device.get_display_mode(0).unwrap();
    /// println!("{:#?}", mode);
    /// assert!(mode.width > 0);
    /// assert_eq!(mode.format, Format::X8R8G8B8);
    /// ```
    ///
    /// ### Outputs
    ///
    /// ```text
    /// DisplayMode {
    ///     width: 2160,
    ///     height: 3840,
    ///     refresh_rate: 60,
    ///     format: Format(D3DFMT_X8R8G8B8),
    /// }
    /// ```
    pub fn get_display_mode(&self, swap_chain: u32) -> Result<DisplayMode, MethodError> {
        let mut dm = DisplayMode::default();
        let hr = unsafe { self.0.GetDisplayMode(swap_chain, &mut *dm) };
        MethodError::check("IDirect3DDevice9::GetDisplayMode", hr)?;
        Ok(dm)
    }
}
