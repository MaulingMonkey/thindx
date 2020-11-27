#![allow(dead_code)] // TODO: remove

use crate::*;

use winapi::um::wingdi::*;

use std::ptr::*;

// TODO: support for Device s in doc comment examples (via dev crate?)
// TODO: fuzz / torture-test Device operations in randomized combinations for odd interactions


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9)\]
/// Core interface used for general rendering, resource creation, etc.
///
/// # Table of Contents
/// | Topic                                         | Overview |
/// | --------------------------------------------- | -------- |
/// | [Common](#common)                             | ...
/// | [Drawing](#drawing)                           | Draw primitives
/// | [Buffers](#buffers)                           | Bind/Create/Update [IndexBuffer]s and [VertexBuffer]s
/// | [Queries](#queries)                           | Create/Check Occlusion and other [Query]s
/// | [Shaders](#shaders)                           | Bind/Create [PixelShader]s and [VertexShader]s
/// | [StateBlocks](#stateblocks)                   | Create/Capture/Replay Direct3D states via [StateBlock]s
/// | [Surfaces](#surfaces)                         | Bind/Create [Surface]s for back buffers, render targets, depth stencil, etc.
/// | [SwapChains](#swapchains)                     | Create [SwapChain]s / [SwapChainEx]s for multi-window rendering
/// | [Textures](#textures)                         | Bind/Create/Update [Texture]s, [CubeTexture]s, and [VolumeTexture]s
/// | [VertexDeclarations](#vertexdeclarations)     | Bind/Create [VertexDeclaration]s for describing [VertexBuffer] layouts
/// | [Miscellanious](#miscellanious)               | Metadata, etc.
/// | [Lighting](#lighting-16-bit)                  | Configure (and query) [Light]ing
/// | [Viewports](#viewports)                       | Configure (and query) the [Viewport]
#[derive(Clone)] #[repr(transparent)]
pub struct Device(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DDevice9>);



/// # Common
/// ...
impl Device {
    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginscene)\]
    /// IDirect3DDevice9::BeginScene
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// device.begin_scene().unwrap();
    /// // ...issue draw calls and stuff...
    /// device.end_scene().unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device was already within a scene (e.g. [begin_scene] was called twice without an intervening [end_scene])
    /// *   `Ok(())`                if the device was not already within a scene, and now is
    ///
    /// [begin_scene]:          #method.begin_scene
    /// [end_scene]:            #method.end_scene
    pub fn begin_scene(&self) -> Result<(), MethodError> {
        // TODO: examples
        let hr = unsafe { self.0.BeginScene() };
        MethodError::check("IDirect3DDevice9::BeginScene", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endscene)\]
    /// IDirect3DDevice9::EndScene
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use doc::*; let device = Device::pure();
    /// device.begin_scene().unwrap();
    /// // ...issue draw calls and stuff...
    /// device.end_scene().unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device was not within a scene (e.g. [end_scene] was called without a [begin_scene], or was called a second time)
    /// *   `Ok(())`                if the device was within a scene that has now ended
    ///
    /// [begin_scene]:          #method.begin_scene
    /// [end_scene]:            #method.end_scene
    pub fn end_scene(&self) -> Result<(), MethodError> {
        // TODO: examples
        let hr = unsafe { self.0.EndScene() };
        MethodError::check("IDirect3DDevice9::EndScene", hr)
    }

    // TODO: fn scene(&self) with sane error handling / drop behavior?

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-present)\]
    /// IDirect3DDevice9::Present
    ///
    /// Presents the contents of the next buffer in the sequence of back buffers owned by the device.
    ///
    /// ### Safety
    ///
    /// *   It's likely unsound to use an invalid, non-null `hwnd`
    /// *   It's likely unsound to use a null `hwnd` if the original `presentation_parameters.hDeviceWindow` is an invalid, non-null HWND
    /// *   Out of bounds rects might also be an issue IDK?
    ///
    /// ### Arguments
    ///
    /// *   `source_rect`           - "Must be `..`" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be `..` even then (the entire source surface is presented.)
    /// *   `dest_rect`             - "Must be `..`" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be `..` even then (the entire client area is filled.)
    /// *   `dest_window_override`  - The destination window to render to.  If null / `()`, the runtime uses the `hDeviceWindow` member of D3DPRESENT_PARAMETERS for the presentation.
    /// *   `dirty_region`          - "Must be [None]" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be [None] even then (the entire region will be considered dirty.)  The implementation is free to copy more than the exact dirty region.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::DEVICEREMOVED]     When you least expect it
    /// *   [D3DERR::DEVICELOST]        When switching into/out-of fullscreen, or when invoking `C:\Windows\System32\DXCap.exe -forcetdr`
    /// *   [D3DERR::INVALIDCALL]       If called within a [Device::begin_scene] .. [Device::end_scene] section, if the render target is the current render target.
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use std::ptr::null_mut;   let hwnd = null_mut();
    /// # use doc::*;               let device = Device::test();
    /// // Present the entire back buffer (should work with all swap chains, probably:)
    /// device.present(.., .., (), None).unwrap();
    /// // TODO: Handle D3DERR::DEVICEREMOVED
    ///
    /// // Or, with a SwapEffect::Copy swap chain, this should succeed (might succeed by simply ignoring the args, even for other SwapEffect s:)
    /// let hwnd = unsafe { SafeHWND::assert(&hwnd) };
    /// match device.present((0,0)..(100,100), Rect::from((0,0)..(100,100)), hwnd, None).map_err(|e| e.d3derr()) {
    ///     Ok(()) => {}, // Huzzah!
    ///     Err(D3DERR::DEVICEREMOVED   ) => { /* oooh, a removable GPU?  Nifty!  Might switch to the laptop's built-in Device (might have different caps!) */ },
    ///     Err(D3DERR::DEVICELOST      ) => { /* switching fullscreen modes? GPU driver crashed? might prompt the user before recreating the device to avoid hang loops */ },
    ///     Err(D3DERR::DEVICEHUNG      ) => { /* ...is it your fault? crazy shaders? might prompt the user before recreating the device to avoid hang loops */ },
    ///     Err(other                   ) => panic!("oh no something bad happened: {}", other),
    /// }
    /// ```
    pub fn present<'r>(&self, source_rect: impl IntoRectOrFull, dest_rect: impl IntoRectOrFull, dest_window_override: impl AsHWND, dirty_region: impl Into<Option<&'r RgnData>>) -> Result<(), MethodError> {
        let source_rect     = source_rect.into_rect();
        let dest_rect       = dest_rect.into_rect();
        let hwnd            = dest_window_override.as_hwnd();
        let dirty_region    = dirty_region.into();

        let source_rect     = source_rect   .map_or(null(), |r| &*r).cast();
        let dest_rect       = dest_rect     .map_or(null(), |r| &*r).cast();
        let dirty_region    = match dirty_region {
            None => null::<RGNDATA>(),
            Some(dr) => {
                if dr.rdh.dwSize as usize   != std::mem::size_of::<RGNDATAHEADER>() { return Err(MethodError("Device::present", D3DERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.iType             != RDH_RECTANGLES                       { return Err(MethodError("Device::present", D3DERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.nCount as usize   > dr.buffer.len()                       { return Err(MethodError("Device::present", D3DERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.nRgnSize as usize > std::mem::size_of_val(dr)             { return Err(MethodError("Device::present", D3DERR::INVALID_STRUCT_FIELD)); }
                let dr : *const RgnData = dr;
                dr.cast()
            },
        };

        let hr = unsafe { self.0.Present(source_rect, dest_rect, hwnd, dirty_region) };
        MethodError::check("IDirect3DDevice9::Present", hr)
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgndata)\]
/// RGNDATA placeholder
///
/// RGNDATA is a header-prefixed array.  While constructable in Rust, they're slightly awkward at best.
#[repr(C)]
pub struct RgnData {
    pub(crate) rdh:    RGNDATAHEADER,
    pub(crate) buffer: [Rect],
}

#[test] fn begin_end_scene() {
    let device = Device::test();
    assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

    device.begin_scene().unwrap();
    device.end_scene().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

    device.begin_scene().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.begin_scene());
    device.end_scene().unwrap();
    assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

    device.begin_scene().unwrap();
    for _ in 0..1000 { assert_eq!(D3DERR::INVALIDCALL, device.begin_scene()); }
    device.end_scene().unwrap();
    for _ in 0..1000 { assert_eq!(D3DERR::INVALIDCALL, device.end_scene()); }
}

#[test] fn present() {
    let device = Device::test_pp(false, |pp, _| pp.SwapEffect = SwapEffect::Copy.into()).unwrap();
    device.present(.., .., (), None).unwrap();

    for rect in [
        (0, 0) .. (1, 1),
        (-100, -100) .. (100, 100),
        (100, 100) .. (-100, -100),
        (-1000, -1000) .. (1000, 1000),
        (1000, 1000) .. (-1000, -1000),
        (-100000, -100000) .. (100000, 100000),
        (0, 0) .. (100000, 100000),
        (0, 0) .. (i32::MAX, i32::MAX),
        (i32::MIN, i32::MIN) .. (i32::MAX, i32::MAX),
        (i32::MAX, i32::MAX) .. (i32::MIN, i32::MIN),
    ].iter().cloned() {
        let rect = Rect::from(rect);
        device.present(rect, rect, (), None).unwrap();
    }
}
