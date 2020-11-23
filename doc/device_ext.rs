use thin3d9::*;

pub trait DeviceExt : Sized {
    /// Create a Device for testing/documentation purpouses.
    ///
    /// Note that the lack of `unsafe means this defeats thin3d9's global soundness - see [Device::create] for details!
    fn test() -> Self;

    /// Create a Device for testing/documentation purpouses.
    ///
    /// Note that the lack of `unsafe means this defeats thin3d9's global soundness - see [Device::create] for details!
    fn test_pp(ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS)) -> Result<Self, MethodError>;
}

impl DeviceExt for Device {
    fn test() -> Self { Self::test_pp(|_| {}).unwrap() }

    fn test_pp(ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS)) -> Result<Self, MethodError> {
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

        use crate::Direct3DExt;
        let d3d = <Direct3D as Direct3DExt>::test();
        unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }
    }
}
