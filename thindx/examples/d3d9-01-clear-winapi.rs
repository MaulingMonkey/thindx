//! Basic [d3d9::Device] setup with [winapi](https://docs.rs/winapi/)
#![windows_subsystem = "windows"]

use thindx::d3d9::*;

use abistr::cstr16 as wcstr;

use winapi::shared::d3d9types::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

use winapi::um::libloaderapi::*;
use winapi::um::winuser::*;

use std::cell::RefCell;
use std::ptr::*;



thread_local! {
    static D3D      : Direct3D = unsafe { Direct3D::create(SdkVersion::default()) }.unwrap();
    static DEVICE   : RefCell<Option<Device>> = RefCell::new(None);
}

/// ### ⚠️ Safety ⚠️
/// *   `hwnd` must be a valid window
/// *   `wparam` / `lparam` may be assumed to be valid pointers depending no the exact `umsg` passed
unsafe extern "system" fn window_proc(hwnd: HWND, umsg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match umsg {
        WM_DESTROY => {
            DEVICE.with(|d| *d.borrow_mut() = None);
            PostQuitMessage(0);
            0
        },
        _ => DefWindowProcW(hwnd, umsg, wparam, lparam),
    }
}

fn main() {
    dev::win32::optional_dev_init();

    let hinstance = unsafe { GetModuleHandleW(null()) };
    assert!(!hinstance.is_null());

    let hcursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
    assert!(!hcursor.is_null());

    let wc = WNDCLASSW {
        lpfnWndProc:    Some(window_proc),
        hInstance:      hinstance,
        hCursor:        hcursor,
        lpszClassName:  wcstr!("SampleWndClass").as_ptr(),
        .. unsafe { std::mem::zeroed() }
    };
    assert_ne!(0, unsafe { RegisterClassW(&wc) });

    let ex_style = 0;
    let style = WS_OVERLAPPEDWINDOW;
    let mut size = RECT { left: 0, right: 800, top: 0, bottom: 600 };
    assert_ne!(0, unsafe { AdjustWindowRectEx(&mut size, style, false.into(), ex_style) });

    let hwnd = unsafe { CreateWindowExW(
        ex_style,
        wcstr!("SampleWndClass").as_ptr(),
        wcstr!("01-clear-winapi - thindx example").as_ptr(),
        style,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        size.right - size.left,
        size.bottom - size.top,
        null_mut(),
        null_mut(),
        hinstance,
        null_mut(),
    )};
    assert!(!hwnd.is_null());

    if !dev::d3d9::hide_for_docs_gen() {
        assert_eq!(0, unsafe { ShowWindow(hwnd, SW_SHOW) });
    }

    let mut pp = D3DPRESENT_PARAMETERS {
        Windowed:               true.into(),
        hDeviceWindow:          hwnd,
        SwapEffect:             SwapEffect::Discard.into(),
        PresentationInterval:   Present::IntervalOne.into(),
        .. unsafe { std::mem::zeroed() }
    };

    let behavior =
        // Create::DisablePrintScreen | // d3d9ex
        Create::FpuPreserve |
        Create::HardwareVertexProcessing |
        Create::NoWindowChanges;

    let device = D3D.with(|d3d| unsafe { d3d.create_device(0, DevType::HAL, null_mut(), behavior, &mut pp) }).unwrap();
    DEVICE.with(|d| *d.borrow_mut() = Some(device));

    loop {
        let mut msg = MSG { message: 0, hwnd: null_mut(), time: 0, pt: POINT { x: 0, y: 0 }, lParam: 0, wParam: 0 };
        while unsafe { PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) } != 0 {
            if msg.message == WM_QUIT { return; }
            unsafe { TranslateMessage(&msg) };
            unsafe { DispatchMessageW(&msg) };
        }

        DEVICE.with(|d| {
            let device = d.borrow();
            if let Some(device) = device.as_ref() {
                device.clear(None, Some(Color::argb(0xFF664422)), None, None).unwrap();
                device.present(.., .., (), None).unwrap(); // TODO: Handle D3DERR::DEVICELOST
                dev::d3d9::screenshot_rt0_for_docs_gen(&device);
            }
        });
    }
}
