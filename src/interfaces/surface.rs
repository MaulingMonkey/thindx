#![allow(dead_code)] // TODO: remove

use crate::*;

use winapi::Interface;
use winapi::shared::d3d9types::*;
use winapi::shared::windef::{RECT, HDC};

use std::convert::TryInto;
use std::ptr::{null, null_mut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3dsurface9)\]
/// (extends [Resource])
/// A dense 2-dimensional region of data, often belonging to a [Texture]
#[derive(Clone)] #[repr(transparent)]
pub struct Surface(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DSurface9>);



/// # Surfaces
/// Bind/Create [Surface]s for back buffers, render targets, depth stencil, etc.
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createdepthstencilsurface)\]
    /// IDirect3DDevice9::CreateDepthStencilSurface
    ///
    /// Creates a depth-stencil resource.
    pub fn create_depth_stencil_surface(&self, width: u32, height: u32, format: Format, multi_sample: MultiSample, multi_sample_quality: u32, discard: bool, _shared_handle: impl SharedHandleParam) -> Result<Surface, MethodError> {
        let mut surface = null_mut();
        let hr = unsafe { self.0.CreateDepthStencilSurface(width, height, format.into(), multi_sample.into(), multi_sample_quality, discard.into(), &mut surface, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateDepthStencilSurface", hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createoffscreenplainsurface)\]
    /// IDirect3DDevice9::CreateOffscreenPlainSurface
    ///
    /// Create an off-screen surface.
    pub fn create_offscreen_plain_surface(&self, width: u32, height: u32, format: Format, pool: Pool, _shared_handle: impl SharedHandleParam) -> Result<Surface, MethodError> {
        let mut surface = null_mut();
        let hr = unsafe { self.0.CreateOffscreenPlainSurface(width, height, format.into(), pool.into(), &mut surface, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateOffscreenPlainSurface", hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createrendertarget)\]
    /// IDirect3DDevice9::CreateRenderTarget
    ///
    /// Creates a render-target surface.
    pub fn create_render_target(&self, width: u32, height: u32, format: Format, multi_sample: MultiSample, multi_sample_quality: u32, lockable: bool, _shared_handle: impl SharedHandleParam) -> Result<Surface, MethodError> {
        let mut surface = null_mut();
        let hr = unsafe { self.0.CreateRenderTarget(width, height, format.into(), multi_sample.into(), multi_sample_quality, lockable.into(), &mut surface, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateRenderTarget", hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getbackbuffer)\]
    /// IDirect3DDevice9::GetBackBuffer
    ///
    /// Retrieves a back buffer from the device's swap chain.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]           if `back_buffer` >= number of back buffers
    /// *   Ok([Surface])
    pub fn get_back_buffer(&self, swap_chain: u32, back_buffer: u32, type_: BackBufferType) -> Result<Surface, MethodError> {
        let mut surface = null_mut();
        let hr = unsafe { self.0.GetBackBuffer(swap_chain, back_buffer, type_.into(), &mut surface) };
        MethodError::check("IDirect3DDevice9::GetBackBuffer", hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdepthstencilsurface)\]
    /// IDirect3DDevice9::GetDepthStencilSurface
    ///
    /// Gets the depth-stencil surface owned by the Direct3DDevice object.
    ///
    /// ### Returns
    ///
    /// * <span style="inaccurate">[D3DERR::INVALIDCALL] ...?</span>
    /// * Ok(Some([Surface]))       the render target bound to that index
    /// * Ok(None)                  no render target was bound to that index
    ///
    /// [Multiple Render Targets (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/multiple-render-targets
    pub fn get_depth_stencil_surface(&self) -> Result<Option<Surface>, MethodError> {
        // TODO: verify soundness before making pub
        let mut ds = null_mut();
        let hr = unsafe { self.0.GetDepthStencilSurface(&mut ds) };
        if hr == D3DERR::NOTFOUND {
            Ok(None)
        } else {
            MethodError::check("IDirect3DDevice9::GetDepthStencilSurface", hr)?;
            Ok(unsafe { Surface::from_raw_opt(ds) })
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertarget)\]
    /// IDirect3DDevice9::GetRenderTarget
    ///
    /// Typically, methods that return state will not work on a device that is created using D3DCREATE_PUREDEVICE. This method however, will work even on a pure device because it returns an interface.
    /// The device can now support multiple render targets. The number of render targets supported by a device is contained in the NumSimultaneousRTs member of [Caps]. See [Multiple Render Targets (Direct3D 9)].
    ///
    /// ### Returns
    ///
    /// * [D3DERR]::???             `render_target_index` > [Caps].NumSimultaneousRTs ?
    /// * Ok(Some([Surface]))       the render target bound to that index
    /// * Ok(None)                  no render target was bound to that index
    ///
    /// [Multiple Render Targets (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/multiple-render-targets
    pub fn get_render_target(&self, render_target_index: u32) -> Result<Option<Surface>, MethodError> {
        // TODO: verify soundness before making pub
        let mut rt = null_mut();
        let hr = unsafe { self.0.GetRenderTarget(render_target_index, &mut rt) };
        if hr == D3DERR::NOTFOUND {
            Ok(None)
        } else {
            MethodError::check("IDirect3DDevice9::GetRenderTarget", hr)?;
            Ok(unsafe { Surface::from_raw_opt(rt) })
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getfrontbufferdata)\]
    /// IDirect3DDevice9::GetFrontBufferData
    pub fn get_front_buffer_data(&self, swap_chain: u32, surface: &Surface) -> Result<(), MethodError> {
        // TODO: verify soundness before making pub
        let hr = unsafe { self.0.GetFrontBufferData(swap_chain, surface.as_raw()) };
        MethodError::check("IDirect3DDevice9::GetFrontBufferData", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setdepthstencilsurface)\]
    /// IDirect3DDevice9::SetDepthStencilSurface
    ///
    /// ### Returns
    ///
    /// * [D3DERR::INVALIDCALL]         if `depth_stencil_surface == Some(surface)` and `surface.usage() != Usage::DepthStencil`
    /// * `Ok(())`                      if the depth stencil was successfully (un)bound
    pub fn set_depth_stencil_surface(&self, depth_stencil_surface: Option<&Surface>) -> Result<(), MethodError> {
        let ds = depth_stencil_surface.map_or(null_mut(), |ds| ds.as_raw());
        let hr = unsafe { self.0.SetDepthStencilSurface(ds) };
        MethodError::check("IDirect3DDevice9::SetDepthStencilSurface", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setrendertarget)\]
    /// IDirect3DDevice9::SetRenderTarget
    ///
    /// ### Returns
    ///
    /// * [D3DERR::INVALIDCALL]         if `render_target_index == 0` and `render_target == None`
    /// * [D3DERR::INVALIDCALL]         if `render_target == Some(surface)` and `surface.usage() != Usage::RenderTarget`
    /// * `Ok(())`                      if the render target was successfully bound
    pub fn set_render_target(&self, render_target_index: u32, render_target: Option<&Surface>) -> Result<(), MethodError> {
        let rt = render_target.map_or(null_mut(), |rt| rt.as_raw());
        let hr = unsafe { self.0.SetRenderTarget(render_target_index, rt) };
        MethodError::check("IDirect3DDevice9::SetRenderTarget", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-clear)\]
    /// IDirect3DDevice9::Clear
    ///
    /// Clears one or more surfaces such as a render target, multiple render targets, a stencil buffer, and a depth buffer.
    ///
    /// ### Returns
    ///
    /// * [D3DERR::INVALIDCALL]     if `rects.len()` > `u32::MAX`
    /// * [D3DERR::INVALIDCALL]     if all non-`rects` parameters were `None`
    /// * [D3DERR::INVALIDCALL]     if `color`   was `Some(...)` without a render target being bound
    /// * [D3DERR::INVALIDCALL]     if `depth`   was `Some(...)` without a depth buffer being bound
    /// * [D3DERR::INVALIDCALL]     if `stencil` was `Some(...)` without a stencil buffer being bound
    /// * `Ok(())`                  otherwise
    pub fn clear(&self, rects: Option<&[Rect]>, color: Option<Color>, depth: Option<f32>, stencil: Option<u32>) -> Result<(), MethodError> {
        // TODO: more clear docs
        // TODO: conversion traits for params?

        let n : u32 = rects.map_or(0, |r| r.len()).try_into().map_err(|_| MethodError("Device::clear", D3DERR::INVALIDCALL))?;
        let rects = rects.map_or(null(), |r| r.as_ptr().cast());

        let flags =
            ((color  .is_some() as u32) * D3DCLEAR_TARGET ) |
            ((depth  .is_some() as u32) * D3DCLEAR_ZBUFFER) |
            ((stencil.is_some() as u32) * D3DCLEAR_STENCIL);

        let color   = color.unwrap_or(Color::argb(0)).into();
        let depth   = depth.unwrap_or(0.0);
        let stencil = stencil.unwrap_or(0);

        let hr = unsafe { self.0.Clear(n, rects, flags, color, depth, stencil) };
        MethodError::check("IDirect3DDevice9::Clear", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-colorfill)\]
    /// IDirect3DDevice9::ColorFill
    ///
    /// Allows an application to fill a rectangular area of a [Pool::Default] surface with a specified color.
    ///
    /// ### Returns
    ///
    /// * [D3DERR::INVALIDCALL]     if `surface` isn't from [Pool::Default] ?
    /// * [D3DERR::INVALIDCALL]     if `surface` isn't a supported format ?
    /// * [D3DERR::INVALIDCALL]     if `rect` exceeds the bounds of the surface
    /// * `Ok(())`                  on success
    pub fn color_fill(&self, surface: &Surface, rect: Option<Rect>, color: impl Into<Color>) -> Result<(), MethodError> {
        let rect = rect.map(RECT::from);
        let rect = rect.as_ref().map_or(null(), |r| r);
        let hr = unsafe { self.0.ColorFill(surface.as_raw(), rect, color.into().into()) };
        MethodError::check("IDirect3DDevice9::ColorFill", hr)
    }
}



pub trait IDirect3DSurface9Ext : private::Sealed {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dsurface9-getcontainer)\]
    /// IDirect3DSurface9::GetContainer
    ///
    /// Provides access to the parent cube texture or texture (mipmap) object, if this surface is a child level of a cube texture or a mipmap. This method can also provide access to the parent swap chain if the surface is a back-buffer child.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::NOINTERFACE]   The container doesn't implement `C`'s interface.
    /// *   Ok(`C`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// # let texture = device.create_texture(128, 128, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// # let surface = texture.get_surface_level(0).unwrap();
    /// let texture = surface.get_container::<Texture>().unwrap();
    /// assert_eq!(D3DERR::NOINTERFACE, surface.get_container::<Device>().err());
    /// ```
    fn get_container<C: Raw>(&self) -> Result<C, MethodError> where C::Raw : Interface {
        let mut container = null_mut();
        let hr = unsafe { self.as_winapi().GetContainer(&C::Raw::uuidof(), &mut container) };
        MethodError::check("IDirect3DVolume9::GetContainer", hr)?;
        Ok(unsafe { C::from_raw(container.cast()) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dsurface9-getdc)\]
    /// IDirect3DSurface9::GetDC
    ///
    /// Retrieves a device context.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If a device context was already retrieved
    /// *   Ok([HDC])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// # let texture = device.create_texture(128, 128, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// # let surface = texture.get_surface_level(0).unwrap();
    /// let dc = surface.get_dc().unwrap();
    /// assert!(!dc.is_null());
    ///
    /// assert_eq!(D3DERR::INVALIDCALL, surface.get_dc().err(), "HDC already acquired");
    /// ```
    fn get_dc(&self) -> Result<HDC, MethodError> {
        let mut hdc = null_mut();
        let hr = unsafe { self.as_winapi().GetDC(&mut hdc) };
        MethodError::check("IDirect3DSurface9::GetDC", hr)?;
        Ok(hdc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dsurface9-getdesc)\]
    /// IDirect3DSurface9::GetDesc
    ///
    /// Retrieves a description of the surface.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([SurfaceDesc])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let texture = device.create_texture(128, 128, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let surface = texture.get_surface_level(0).unwrap();
    /// let desc : SurfaceDesc = surface.get_desc().unwrap();
    /// assert_eq!(desc.width,  128);
    /// assert_eq!(desc.height, 128);
    /// // ...
    /// ```
    fn get_desc(&self) -> Result<SurfaceDesc, MethodError> {
        let mut desc = SurfaceDesc::default();
        let hr = unsafe { self.as_winapi().GetDesc(&mut *desc) };
        MethodError::check("IDirect3DSurface9::GetDesc", hr)?;
        Ok(desc)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dsurface9-lockrect)\]
    /// IDirect3DSurface9::LockRect
    ///
    /// Locks a rectangle on a surface.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([D3DLOCKED_RECT])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let texture = device.create_texture(128, 128, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let surface = texture.get_surface_level(0).unwrap();
    /// let data = [[Color::argb(0xFF112233); 128]; 128];
    /// unsafe {
    ///     let lock = surface.lock_rect_unchecked(.., Lock::None).unwrap();
    ///     for y in 0..128 {
    ///         let src = data[y].as_ptr();
    ///         let dst = (lock.pBits as *mut u8).add(y * lock.Pitch as usize);
    ///         std::ptr::copy(src, dst.cast(), 128);
    ///     }
    ///     surface.unlock_rect();
    /// }
    /// ```
    unsafe fn lock_rect_unchecked(&self, rect: impl IntoRectOrFull, flags: impl Into<Lock>) -> Result<D3DLOCKED_RECT, MethodError> {
        let rect = rect.into_rect();
        let rect = rect.as_ref().map_or(null(), |b| &**b);
        let mut locked = std::mem::zeroed::<D3DLOCKED_RECT>();
        let hr = self.as_winapi().LockRect(&mut locked, rect.cast(), flags.into().into());
        MethodError::check("IDirect3DSurface9::LockRect", hr)?;
        Ok(locked)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dsurface9-releasedc)\]
    /// IDirect3DSurface9::ReleaseDC
    ///
    /// Release a device context handle.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// # let texture = device.create_texture(128, 128, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// # let surface = texture.get_surface_level(0).unwrap();
    /// let dc = surface.get_dc().unwrap();
    /// assert!(!dc.is_null());
    ///
    /// assert_eq!(D3DERR::INVALIDCALL, surface.get_dc().err(), "HDC already acquired");
    /// assert_eq!(D3DERR::INVALIDCALL, surface.release_dc(42 as _).err(), "not the right HDC");
    ///
    /// surface.release_dc(dc).unwrap();
    /// let dc = surface.get_dc().unwrap();
    /// surface.release_dc(dc).unwrap();
    /// ```
    fn release_dc(&self, hdc: HDC) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().ReleaseDC(hdc) };
        MethodError::check("IDirect3DSurface9::ReleaseDC", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3dsurface9-unlockrect)\]
    /// IDirect3DSurface9::UnlockRect
    ///
    /// Unlocks a rectangle on a surface.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::test();
    /// let texture = device.create_texture(128, 128, 8, Usage::None, Format::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let surface = texture.get_surface_level(0).unwrap();
    /// let data = [[Color::argb(0xFF112233); 128]; 128];
    /// unsafe {
    ///     let lock = surface.lock_rect_unchecked(.., Lock::None).unwrap();
    ///     for y in 0..128 {
    ///         let src = data[y].as_ptr();
    ///         let dst = (lock.pBits as *mut u8).add(y * lock.Pitch as usize);
    ///         std::ptr::copy(src, dst.cast(), 128);
    ///     }
    ///     surface.unlock_rect();
    /// }
    /// ```
    fn unlock_rect(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().UnlockRect() };
        MethodError::check("IDirect3DSurface9::UnlockRect", hr)
    }
}

impl<T: private::Sealed> IDirect3DSurface9Ext for T {}

mod private {
    use winapi::shared::d3d9::IDirect3DSurface9;
    pub unsafe trait Sealed                             { fn as_winapi(&self) -> &IDirect3DSurface9; }
    unsafe impl Sealed for mcom::Rc<IDirect3DSurface9>  { fn as_winapi(&self) -> &IDirect3DSurface9 { &**self } }
    unsafe impl Sealed for super::Surface               { fn as_winapi(&self) -> &IDirect3DSurface9 { &*self.0 } }
}



// #[test] fn create_depth_stencil_surface() {} // TODO
// #[test] fn create_offscreen_plain_surface() {} // TODO
// #[test] fn create_render_target() {} // TODO

// #[test] fn get_back_buffer() {} // TODO

#[test] fn get_set_depth_stencil_surface() {
    let device = Device::test_pp(false, |pp, _| {
        pp.EnableAutoDepthStencil = true.into();
        pp.AutoDepthStencilFormat = Format::D24S8.into();
    }).unwrap();

    let ds = device.get_depth_stencil_surface().unwrap().unwrap();
    assert_eq!(ds.as_raw(), device.get_depth_stencil_surface().unwrap().unwrap().as_raw());
    device.set_depth_stencil_surface(Some(&ds)).unwrap();
    assert_eq!(ds.as_raw(), device.get_depth_stencil_surface().unwrap().unwrap().as_raw());
    device.set_depth_stencil_surface(None).unwrap();
    assert!(device.get_depth_stencil_surface().unwrap().is_none());

    // TODO: What happens with pure devices?
    // TODO: What happens with setting surfaces with poor formats?
    // TODO: What happens with setting surfaces with mismatched resolutions?
}

#[test] fn get_set_render_target() {
    let device = Device::test();
    //let caps = device...
    let max_rts = 4; // TODO: caps

    // attempt to muck with RT0
    let rt0 = device.get_render_target(0).expect("RT0 inaccessable").expect("RT0 null");
    device.set_render_target(0, Some(&rt0)).unwrap();
    assert_eq!(rt0.as_raw(), device.get_render_target(0).expect("RT0 inaccessable").expect("RT0 null").as_raw());
    assert_eq!(D3DERR::INVALIDCALL, device.set_render_target(0, None));
    assert_eq!(rt0.as_raw(), device.get_render_target(0).expect("RT0 inaccessable").expect("RT0 null").as_raw());
    device.set_render_target(0, Some(&rt0)).unwrap();
    assert_eq!(rt0.as_raw(), device.get_render_target(0).expect("RT0 inaccessable").expect("RT0 null").as_raw());

    // muck with probably-in-bounds RTs
    for _i in 1..max_rts {
        // TODO: create real textures to bind
        // println!("rt[{}]", i);
        // device.set_render_target(i, Some(&rt0)).unwrap();
    }

    // TODO: What happens with setting surfaces with poor formats?
    // TODO: What happens with setting surfaces with mismatched resolutions?

    // muck with probably-out-of-bounds RTs
    for i in [100, 1000, 100000, !0].iter().copied() {
        // TODO: create real textures to bind
        assert_eq!(D3DERR::INVALIDCALL, device.set_render_target(i, Some(&rt0)));
    }
}

// #[test] fn get_front_buffer_data() {} // TODO

#[test] fn clear() {
    // TODO: fuzz rects harder

    for (ds_fmt,                        has_depth,  has_sten,   require ) in [
        (None,                          false,      false,      true    ),
        (Some(Format::D24S8),           true,       true,       true    ),
        (Some(Format::D24X8),           true,       false,      true    ),
        (Some(Format::D16),             true,       false,      true    ),
        (Some(Format::D15S1),           true,       true,       false   ),
        (Some(Format::D24X4S4),         true,       true,       false   ),
        (Some(Format::D16_LOCKABLE),    true,       false,      true    ),
        (Some(Format::D24FS8),          true,       true,       false   ),
        (Some(Format::D32),             true,       false,      false   ),
        (Some(Format::D32_LOCKABLE),    true,       false,      false   ),
        (Some(Format::D32F_LOCKABLE),   true,       false,      true    ),
        (Some(Format::X8_LOCKABLE),     false,      false,      false   ),
    ].iter().copied() {
        let device = Device::test_pp(false, |pp, _| {
            if let Some(ds_fmt) = ds_fmt {
                pp.EnableAutoDepthStencil = true.into();
                pp.AutoDepthStencilFormat = ds_fmt.into();
            }
        });
        if device.is_err() && !require { continue }
        let device = device.unwrap_or_else(|err| panic!("bad depth-stencil: {:?} {:?}", ds_fmt, err));

        for remove_ds in [false, true].iter().copied() {
            let autods = device.get_depth_stencil_surface().unwrap();
            if remove_ds {
                if autods.is_none() { continue }
                device.set_depth_stencil_surface(None).unwrap();
            }

            let has_depth = has_depth && !remove_ds;
            let has_sten  = has_sten  && !remove_ds;

            for (target,                        depth,                      stencil     ) in [
                (None,                          None,                       None        ), // none
                (Some(Color::argb(0xFF112233)), Some(0.5),                  Some(0xFFFF)), // all
                // color
                (Some(Color::argb(0xFF112233)), None,                       None        ),
                // depth
                (None,                          Some(-1.0),                 None        ),
                (None,                          Some(-0.5),                 None        ),
                (None,                          Some( 0.0),                 None        ),
                (None,                          Some( 0.5),                 None        ),
                (None,                          Some( 1.0),                 None        ),
                (None,                          Some( 1.5),                 None        ),
                (None,                          Some( 2.0),                 None        ),
                (None,                          Some( 2.0),                 None        ),
                (None,                          Some(f32::MIN),             None        ),
                (None,                          Some(f32::MIN_POSITIVE),    None        ),
                (None,                          Some(f32::NAN),             None        ),
                (None,                          Some(f32::INFINITY),        None        ),
                // stencil
                (None,                          None,                       Some(0)     ),
                (None,                          None,                       Some(0xFF)  ),
                (None,                          None,                       Some(!0)    ),
            ].iter().copied() {
                println!("ds_fmt={:?} target={:?} depth={:?} stencil={:?}", ds_fmt, target, depth, stencil);
                for rects in [
                    None,
                    Some(&[][..]),
                    Some(&[Rect::from([0..0,   0..0])][..]),                                    // empty rect
                    Some(&[Rect::from([0..100, 0..100])][..]),                                  // small rect
                    Some(&[Rect::from([100..10000, 100..10000])][..]),                          // out of bounds rect
                    Some(&[Rect::from([-10000..10000, -10000..10000])][..]),                    // positive dims huge rect
                    Some(&[Rect::from([10000..-10000, 10000..-10000])][..]),                    // negative dims huge rect
                    Some(&[Rect::from([0..100, 0..100]), Rect::from([0..100, 0..100])][..]),    // overlapping rects
                ].iter().copied() {
                    let result = device.clear(rects, target, depth, stencil);
                    println!("  rects: {: <70} == {:?}", format!("{:?}", rects), result);

                    if target.is_none() && depth.is_none() && stencil.is_none() {
                        assert_eq!(D3DERR::INVALIDCALL, result, "expected failure - target=depth=stencil=None - nothing to clear");
                    } else if depth.is_some() && !has_depth {
                        assert_eq!(D3DERR::INVALIDCALL, result, "expected failure - depth={:?} was specified but no depth buffer was bound", depth);
                    } else if stencil.is_some() && !has_sten {
                        assert_eq!(D3DERR::INVALIDCALL, result, "expected failure - stencil={:?} was specified but no stencil buffer was bound", stencil);
                    } else {
                        result.expect("expected success, got");
                    }
                }
            }

            device.set_depth_stencil_surface(autods.as_ref()).unwrap();
        }
    }
}

#[test] fn color_fill() {
    let device = Device::test();
    let rt0 = device.get_render_target(0).unwrap().unwrap();
    device.color_fill(&rt0, None, Color::argb(0xFF112233)).unwrap();
    device.color_fill(&rt0, Some(Rect::from([0..1, 0..1])), Color::argb(0xFF112233)).unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.color_fill(&rt0, Some( Rect::from([-1..1, -1..1])                 ), Color::argb(0xFF112233)));
    assert_eq!(D3DERR::INVALIDCALL, device.color_fill(&rt0, Some( Rect::from([0..100000, 0..100000])         ), Color::argb(0xFF112233)));
    assert_eq!(D3DERR::INVALIDCALL, device.color_fill(&rt0, Some( Rect::from([10000..-10000, 10000..-10000]) ), Color::argb(0xFF112233)));
    assert_eq!(D3DERR::INVALIDCALL, device.color_fill(&rt0, Some( Rect::from([-10000..10000, -10000..10000]) ), Color::argb(0xFF112233)));

    // TODO: test unsupported pools
    // TODO: test unsupported formats
}
