#![allow(dead_code)] // TODO: remove

use crate::*;

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

        use winapi::shared::d3d9::*;
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
            //D3DCREATE_DISABLE_PRINTSCREEN | // not implemented by winapi?
            D3DCREATE_FPU_PRESERVE |
            D3DCREATE_HARDWARE_VERTEXPROCESSING |
            D3DCREATE_NOWINDOWCHANGES;

        let d3d = Direct3D::test();
        unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }
    }
}
