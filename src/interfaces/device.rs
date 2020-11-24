#![allow(dead_code)] // TODO: remove

use crate::*;

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
    // TODO: examples

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
    /// *   `source_rect`   - "Must be [None]" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be [None] even then (the entire source surface is presented.)
    /// *   `dest_rect`     - "Must be [None]" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be [None] even then (the entire client area is filled.)
    /// *   `hwnd`
    /// *   `dirty_region`  - "Must be [None]" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be [None] even then (the entire region will be considered dirty.)  The implementation is free to copy more than the exact dirty region.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::DEVICEREMOVED]     When you least expect it
    /// *   [D3DERR::INVALIDCALL]       If called within a [Device::begin_scene] .. [Device::end_scene] section, if the render target is the current render target.
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use std::ptr::null_mut;   let hwnd = null_mut();
    /// # use doc::*;               let device = Device::test();
    /// // Present the entire back buffer (should work with all swap chains, probably:)
    /// unsafe { device.present(None, None, null_mut(), None) }.unwrap();
    /// // TODO: Handle D3DERR::DEVICEREMOVED
    ///
    /// // Or, with a SwapEffect::Copy swap chain, this should succeed (might succeed by simply ignoring the args, even for other SwapEffect s:)
    /// let _ = unsafe { device.present(Rect::from((0,0)..(100,100)), Rect::from((0,0)..(100,100)), hwnd, None) };
    /// ```
    pub unsafe fn present<'r>(&self, source_rect: impl Into<Option<Rect>>, dest_rect: impl Into<Option<Rect>>, hwnd: impl Into<HWND>, dirty_region: impl Into<Option<&'r RgnData>>) -> Result<(), MethodError> {
        let source_rect     = source_rect.into();
        let dest_rect       = dest_rect.into();
        let hwnd            = hwnd.into();
        let _dirty_region   = dirty_region;

        let source_rect     = source_rect   .map_or(null(), |r| &*r).cast();
        let dest_rect       = dest_rect     .map_or(null(), |r| &*r).cast();

        let hr = self.0.Present(source_rect, dest_rect, hwnd, null());
        MethodError::check("IDirect3DDevice9::Present", hr)
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgndata)\]
/// RGNDATA placeholder
///
/// RGNDATA is a header-prefixed array.  While constructable in Rust, they're slightly awkward at best.
#[repr(C)]
pub struct RgnData {
    rdh:    winapi::um::wingdi::RGNDATAHEADER,
    buffer: [Rect],
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



#[cfg(test)] impl Device {
    pub fn test() -> Self { Self::test_pp(|_| {}).unwrap() }

    pub fn test_pp(ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS)) -> Result<Self, MethodError> {
        // winit's new_any_thread is unix-only, but I want/need threaded windows unit tests, so create an HWND from scratch.
        use wchar::wch_c;

        use winapi::shared::d3d9caps::*;
        use winapi::shared::d3d9types::*;
        use winapi::shared::windef::*;
        
        use winapi::um::libloaderapi::*;
        use winapi::um::winuser::*;

        use std::ptr::*;

        thread_local! {
            static HWND : HWND = unsafe {
                let hinstance = GetModuleHandleW(null());
                assert!(!hinstance.is_null());

                let hcursor = LoadCursorW(null_mut(), IDC_ARROW);
                assert!(!hcursor.is_null());

                let wc = WNDCLASSW {
                    lpfnWndProc:    Some(DefWindowProcW),
                    hInstance:      hinstance,
                    hCursor:        hcursor,
                    lpszClassName:  wch_c!("UnitTestWndClass").as_ptr(),
                    .. std::mem::zeroed()
                };
                RegisterClassW(&wc); // might fail if previously registered
            
                let hwnd = CreateWindowExW(
                    0,
                    wch_c!("UnitTestWndClass").as_ptr(),
                    wch_c!("thin3d9 unit test").as_ptr(),
                    0, //WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    800,
                    600,
                    null_mut(),
                    null_mut(),
                    hinstance,
                    null_mut(),
                );
                assert!(!hwnd.is_null());

                hwnd
            };
        }

        let mut pp = D3DPRESENT_PARAMETERS {
            Windowed:               true.into(),
            hDeviceWindow:          HWND.with(|hwnd| *hwnd),
            SwapEffect:             D3DSWAPEFFECT_DISCARD,
            PresentationInterval:   D3DPRESENT_INTERVAL_IMMEDIATE,
            .. unsafe { std::mem::zeroed() }
        };
        ppf(&mut pp);

        let behavior =
            Create::DisablePrintScreen |
            Create::FpuPreserve |
            Create::HardwareVertexProcessing |
            Create::NoWindowChanges;

        let d3d = Direct3D::test();
        unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }
    }
}
