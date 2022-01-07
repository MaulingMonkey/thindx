pub fn testfast() -> bool { std::env::var_os("TESTFAST").is_some() }

pub mod d3d9 {
    #[doc(no_inline)] pub use crate::testfast;
    #[doc(no_inline)] pub use crate::win32::*;

    #[doc(no_inline)] pub use thindx::*;
    #[doc(no_inline)] pub use thindx::d3d::*;
    #[doc(no_inline)] pub use thindx::d3d9::*;

    use winapi::shared::d3d9caps::*;
    use winapi::shared::d3d9types::*;

    // XXX: temporary?

    #[doc(no_inline)] pub use winapi::shared::d3d9caps::{
        D3DCAPS9,
    };
    #[doc(no_inline)] pub use winapi::shared::d3d9types::{
        D3DDISPLAYMODE,
        D3DDISPLAYMODEEX,
        D3DPRESENT_PARAMETERS,
    };
    #[doc(no_inline)] pub use winapi::shared::windef::{
        HWND,
        HMONITOR,
    };
    pub type AdapterIndex = u32;
    pub const D3DADAPTER_DEFAULT : AdapterIndex = 0;



    // Direct3D{,Ex}

    #[cfg(feature = "9ex")]
    pub fn d3dex_test() -> Direct3DEx { unsafe { Direct3DEx::create(SdkVersion::default()).unwrap() } }
    pub fn d3d_test()   -> Direct3D   { unsafe { Direct3D  ::create(SdkVersion::default()).unwrap() } }



    // SafeDevice

    pub fn safe_device_test() -> SafeDevice { SafeDevice::new(device_test()).unwrap() }
    pub fn safe_device_pure() -> SafeDevice { SafeDevice::new(device_pure()).unwrap() }
    pub fn safe_device_test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<SafeDevice, MethodError> { Ok(SafeDevice::new(device_test_pp(two, ppf)?)?) }



    // Device

    pub fn device_test() -> Device { device_test_pp(false, |_,_|{}).unwrap() }
    pub fn device_pure() -> Device { device_test_pp(false, |_,c| *c |= Create::PureDevice).unwrap() }
    pub fn device_test2() -> [Device; 2] {[
        device_test_pp(false, |_,_|{}).unwrap(),
        device_test_pp(true,  |_,_|{}).unwrap(),
    ]}
    pub fn device_pure2() -> [Device; 2] {[
        device_test_pp(false, |_,c| *c |= Create::PureDevice).unwrap(),
        device_test_pp(true,  |_,c| *c |= Create::PureDevice).unwrap(),
    ]}

    pub fn device_test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<Device, MethodError> {
        let mut behavior = /* Create::DisablePrintScreen | */ Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges;
        let mut pp = D3DPRESENT_PARAMETERS {
            Windowed:               true.into(),
            hDeviceWindow:          test_window(two),
            SwapEffect:             D3DSWAPEFFECT_DISCARD,
            PresentationInterval:   D3DPRESENT_INTERVAL_IMMEDIATE,
            .. unsafe { std::mem::zeroed() }
        };

        ppf(&mut pp, &mut behavior);
        let d3d = d3d_test();
        unsafe { d3d.create_device(0, DevType::HAL, std::ptr::null_mut(), behavior, &mut pp) }
    }



    // DeviceEx

    #[cfg(feature = "9ex")] pub fn device_ex_test() -> DeviceEx { device_ex_test_pp(false, |_, _|{}).unwrap() }
    #[cfg(feature = "9ex")] pub fn device_ex_pure() -> DeviceEx { device_ex_test_pp(false, |_,c| *c |= Create::PureDevice).unwrap() }

    #[cfg(feature = "9ex")] pub fn device_ex_test2() -> [DeviceEx; 2] {[
        device_ex_test_pp(false, |_,_|{}).unwrap(),
        device_ex_test_pp(true,  |_,_|{}).unwrap(),
    ]}

    #[cfg(feature = "9ex")] pub fn device_ex_pure2() -> [DeviceEx; 2] {[
        device_ex_test_pp(false, |_,c| *c |= Create::PureDevice).unwrap(),
        device_ex_test_pp(true,  |_,c| *c |= Create::PureDevice).unwrap(),
    ]}

    #[cfg(feature = "9ex")] pub fn device_ex_test_pp(two: bool, ppf: impl FnOnce(&mut D3DPRESENT_PARAMETERS, &mut Create)) -> Result<DeviceEx, MethodError> {
        let mut behavior = Create::DisablePrintScreen | Create::FpuPreserve | Create::HardwareVertexProcessing | Create::NoWindowChanges;
        let mut pp = D3DPRESENT_PARAMETERS {
            Windowed:               true.into(),
            hDeviceWindow:          test_window(two),
            SwapEffect:             D3DSWAPEFFECT_DISCARD,
            PresentationInterval:   D3DPRESENT_INTERVAL_IMMEDIATE,
            .. unsafe { std::mem::zeroed() }
        };

        ppf(&mut pp, &mut behavior);
        let d3d = d3dex_test();
        unsafe { d3d.create_device_ex(0, DevType::HAL, std::ptr::null_mut(), behavior, &mut pp, &mut []) }
    }



    pub struct Invalid;
    impl From<Invalid> for DevType         { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
    impl From<Invalid> for Format          { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
    impl From<Invalid> for FVF             { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
    impl From<Invalid> for MultiSample     { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
    impl From<Invalid> for Pool            { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
    impl From<Invalid> for ResourceType    { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
    impl From<Invalid> for Usage           { fn from(_: Invalid) -> Self { Self::from_unchecked(!0) } }
}

pub mod win32 {
    use thindx::abistr::cstr16 as wcstr;

    use winapi::shared::windef::*;

    use winapi::um::libloaderapi::*;
    use winapi::um::winuser::*;

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
                lpszClassName:  wcstr!("Thin3D9Extra").as_ptr(),
                .. std::mem::zeroed()
            };
            RegisterClassW(&wc); // might fail if previously registered

            let hwnd = CreateWindowExW(
                0,
                wcstr!("Thin3D9Extra").as_ptr(),
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
        pub static HWND1 : HWND = create_window("thin3d9 test window #1");
        pub static HWND2 : HWND = create_window("thin3d9 test window #2");
    }
}
