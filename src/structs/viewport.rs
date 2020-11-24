use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dviewport9)\]
/// D3DVIEWPORT9
///
/// Defines Viewport properties.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct Viewport {
    /// Pixel coordinate of the upper-left corner of the viewport on the render-target surface.
    /// Unless you want to render to a subset of the surface, this member can be set to 0.
    pub x:      u32,

    /// Pixel coordinate of the upper-left corner of the viewport on the render-target surface.
    /// Unless you want to render to a subset of the surface, this member can be set to 0.
    pub y:      u32,

    /// Width dimension of the clip volume, in pixels.
    /// Unless you are rendering only to a subset of the surface, this member should be set to the width dimension of the render-target surface.
    pub width:  u32,

    /// Height dimension of the clip volume, in pixels.
    /// Unless you are rendering only to a subset of the surface, this member should be set to the height dimension of the render-target surface.
    pub height: u32,

    /// Together with [max_z], value describing the range of depth values into which a scene is to be rendered, the minimum and maximum values of the clip volume.
    /// Most applications set this value to 0.0.
    /// Clipping is performed after applying the projection matrix.
    ///
    /// [max_z]:        #structfield.max_z
    pub min_z:  f32,

    /// Together with [min_z], value describing the range of depth values into which a scene is to be rendered, the minimum and maximum values of the clip volume.
    /// Most applications set this value to 1.0. Clipping is performed after applying the projection matrix.
    ///
    /// [min_z]:        #structfield.min_z
    pub max_z:  f32,
}

impl Deref    for Viewport { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DVIEWPORT9; }
impl DerefMut for Viewport { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DVIEWPORT9> for Viewport { fn from(value: D3DVIEWPORT9) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Viewport> for D3DVIEWPORT9 { fn from(value: Viewport    ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { Viewport => unsafe D3DVIEWPORT9 { x => X, y => Y, width => Width, height => Height, min_z => MinZ, max_z => MaxZ } }



/// # Viewports
/// Configure (and query) the [Viewport]
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setviewport)\]
    /// IDirect3DDevice9::SetViewport
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the viewport is invalid / describes a region that cannot exist within the render target surface.
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// device.set_viewport(Viewport{ x: 0, y: 0, width: 100, height: 100, min_z: 0.0, max_z: 1.0 }).unwrap();
    /// ```
    pub fn set_viewport(&self, viewport: impl Into<Viewport>) -> Result<(), MethodError> {
        let viewport = viewport.into();
        let hr = unsafe { self.0.SetViewport(&*viewport) };
        MethodError::check("IDirect3DDevice9::SetViewport", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getviewport)\]
    /// IDirect3DDevice9::GetViewport
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If this is a pure device?
    /// *   [D3DERR::INVALIDCALL]   If the viewport is invalid
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let viewport : Viewport = device.get_viewport().unwrap();
    /// ```
    pub fn get_viewport(&self) -> Result<Viewport, MethodError> {
        let mut viewport = Viewport::default();
        let hr = unsafe { self.0.GetViewport(&mut *viewport) };
        MethodError::check("IDirect3DDevice9::GetViewport", hr)?;
        Ok(viewport)
    }
}
