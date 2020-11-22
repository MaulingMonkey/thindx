#![windows_subsystem = "windows"]

use thin3d9::*;

use wchar::wch_c;

use winapi::shared::d3d9::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::d3d9types::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::winerror::SUCCEEDED;

use winapi::um::debugapi::*;
use winapi::um::libloaderapi::*;
use winapi::um::winuser::*;

use std::cell::RefCell;
use std::ptr::*;



thread_local! {
    static D3D      : Direct3D = unsafe { Direct3D::create(SdkVersion::default()) }.unwrap();
    static DEVICE   : RefCell<Option<Device>> = RefCell::new(None);
}

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
    std::panic::set_hook(Box::new(|pi| unsafe {
        eprintln!("{}", pi);
        if IsDebuggerPresent() != 0 { DebugBreak(); }
        std::process::exit(1);
    }));

    let hinstance = unsafe { GetModuleHandleW(null()) };
    assert!(!hinstance.is_null());

    let hcursor = unsafe { LoadCursorW(null_mut(), IDC_ARROW) };
    assert!(!hcursor.is_null());

    let wc = WNDCLASSW {
        lpfnWndProc:    Some(window_proc),
        hInstance:      hinstance,
        hCursor:        hcursor,
        lpszClassName:  wch_c!("SampleWndClass").as_ptr(),
        .. unsafe { std::mem::zeroed() }
    };
    assert_ne!(0, unsafe { RegisterClassW(&wc) });

    let hwnd = unsafe { CreateWindowExW(
        0,
        wch_c!("SampleWndClass").as_ptr(),
        wch_c!("00-clear-winapi - thin3d9 example").as_ptr(),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        800,
        600,
        null_mut(),
        null_mut(),
        hinstance,
        null_mut(),
    )};
    assert!(!hwnd.is_null());
    assert_eq!(0, unsafe { ShowWindow(hwnd, SW_SHOW) });

    let mut pp = D3DPRESENT_PARAMETERS {
        Windowed:               true.into(),
        hDeviceWindow:          hwnd,
        SwapEffect:             D3DSWAPEFFECT_DISCARD,
        PresentationInterval:   D3DPRESENT_INTERVAL_ONE,
        .. unsafe { std::mem::zeroed() }
    };

    // https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcreate
    let behavior =
        //D3DCREATE_DISABLE_PRINTSCREEN | // not implemented by winapi?
        D3DCREATE_FPU_PRESERVE |
        D3DCREATE_HARDWARE_VERTEXPROCESSING |
        D3DCREATE_NOWINDOWCHANGES;

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
                let device = unsafe { &*device.as_raw() };
                assert!(SUCCEEDED(unsafe { device.Present(null(), null(), null_mut(), null()) }));
            }
        });
    }
}
