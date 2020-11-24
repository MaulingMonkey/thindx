use crate::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9ex)\]
/// (extends [Device])
/// Core interface used for general rendering, resource creation, etc.
#[cfg(feature = "9ex")]
#[derive(Clone)] #[repr(transparent)]
pub struct DeviceEx(pub(crate) mcom::Rc<winapi::shared::d3d9::IDirect3DDevice9Ex>);



#[cfg(test)] impl DeviceEx {
    pub fn test() -> Self { Self::test_pp(|_| {}).unwrap() }

    pub fn test_pp(ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS)) -> Result<Self, MethodError> {
        // winit's new_any_thread is unix-only, but I want/need threaded windows unit tests, so create an HWND from scratch.
        use wchar::wch_c;

        use winapi::shared::d3d9caps::*;
        use winapi::shared::d3d9types::*;
        use winapi::shared::windef::*;

        use winapi::um::libloaderapi::*;
        use winapi::um::winuser::*;

        use std::convert::*;
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

        let d3d = Direct3DEx::test();
        unsafe { d3d.create_device_ex(0, DevType::HAL, null_mut(), behavior, &mut pp, &mut [D3DDISPLAYMODEEX{
            Size:               std::mem::size_of::<D3DDISPLAYMODEEX>().try_into().unwrap(),
            Width:              0,
            Height:             0,
            Format:             Format::Unknown.into(),
            RefreshRate:        0,
            ScanLineOrdering:   0,
        }])}
    }
}
