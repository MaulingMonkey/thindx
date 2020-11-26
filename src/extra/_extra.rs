use crate::*;

use wchar::wch_c;

use winapi::shared::d3d9caps::*;
use winapi::shared::d3d9types::*;
use winapi::shared::windef::*;

use winapi::um::libloaderapi::*;
use winapi::um::winuser::*;

use std::convert::*;
use std::ptr::*;

pub fn create_window(title: &str) -> HWND {
    unsafe {
        let hinstance = GetModuleHandleW(null());
        assert!(!hinstance.is_null());

        let hcursor = LoadCursorW(null_mut(), IDC_ARROW);
        assert!(!hcursor.is_null());

        let wc = WNDCLASSW {
            lpfnWndProc:    Some(DefWindowProcW),
            hInstance:      hinstance,
            hCursor:        hcursor,
            lpszClassName:  wch_c!("Thin3D9Extra").as_ptr(),
            .. std::mem::zeroed()
        };
        RegisterClassW(&wc); // might fail if previously registered
    
        let hwnd = CreateWindowExW(
            0,
            wch_c!("Thin3D9Extra").as_ptr(),
            title.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_ptr(),
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
    }
}

#[doc(hidden)] pub fn test_window(two: bool) -> HWND {
    if two {
        HWND2.with(|hwnd| *hwnd)
    } else {
        HWND1.with(|hwnd| *hwnd)
    }
}

thread_local! {
    pub(crate) static HWND1 : HWND = create_window("thin3d9 test window #1");
    pub(crate) static HWND2 : HWND = create_window("thin3d9 test window #2");
}



#[doc(hidden)] impl Direct3D   { pub fn test() -> Self { unsafe { Direct3D  ::create(SdkVersion::default()).unwrap() } } }
#[doc(hidden)] impl Direct3DEx { pub fn test() -> Self { unsafe { Direct3DEx::create(SdkVersion::default()).unwrap() } } }

#[doc(hidden)] impl SafeDevice {
    pub fn test() -> Self { SafeDevice::new(Device::test()).unwrap() }
    pub fn pure() -> Self { SafeDevice::new(Device::pure()).unwrap() }
    pub fn test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<Self, MethodError> { Ok(SafeDevice::new(Device::test_pp(two, ppf)?)?) }
}

#[doc(hidden)] impl Device {
    pub fn test() -> Self { Self::test_pp(false, |_,_|{}).unwrap() }
    pub fn pure() -> Self { Self::test_pp(false, |_,c| *c |= Create::PureDevice).unwrap() }
    pub fn test2() -> [Self; 2] {[
        Self::test_pp(false, |_,_|{}).unwrap(),
        Self::test_pp(true,  |_,_|{}).unwrap(),
    ]}
    pub fn pure2() -> [Self; 2] {[
        Self::test_pp(false, |_,c| *c |= Create::PureDevice).unwrap(),
        Self::test_pp(true,  |_,c| *c |= Create::PureDevice).unwrap(),
    ]}

    pub fn test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<Self, MethodError> {
        let mut behavior = Create::DisablePrintScreen | Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges;
        let mut pp = D3DPRESENT_PARAMETERS {
            Windowed:               true.into(),
            hDeviceWindow:          test_window(two),
            SwapEffect:             D3DSWAPEFFECT_DISCARD,
            PresentationInterval:   D3DPRESENT_INTERVAL_IMMEDIATE,
            .. unsafe { std::mem::zeroed() }
        };

        ppf(&mut pp, &mut behavior);
        let d3d = Direct3D::test();
        unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }
    }
}

#[doc(hidden)] impl DeviceEx {
    pub fn test() -> Self { Self::test_pp(false, |_, _|{}).unwrap() }
    pub fn pure() -> Self { Self::test_pp(false, |_,c| *c |= Create::PureDevice).unwrap() }
    pub fn test2() -> [Self; 2] {[
        Self::test_pp(false, |_,_|{}).unwrap(),
        Self::test_pp(true,  |_,_|{}).unwrap(),
    ]}
    pub fn pure2() -> [Self; 2] {[
        Self::test_pp(false, |_,c| *c |= Create::PureDevice).unwrap(),
        Self::test_pp(true,  |_,c| *c |= Create::PureDevice).unwrap(),
    ]}

    pub fn test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<Self, MethodError> {
        let mut behavior = Create::DisablePrintScreen | Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges;
        let mut pp = D3DPRESENT_PARAMETERS {
            Windowed:               true.into(),
            hDeviceWindow:          test_window(two),
            SwapEffect:             D3DSWAPEFFECT_DISCARD,
            PresentationInterval:   D3DPRESENT_INTERVAL_IMMEDIATE,
            .. unsafe { std::mem::zeroed() }
        };

        ppf(&mut pp, &mut behavior);
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
