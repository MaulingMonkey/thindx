use thindx::abistr::cstr16 as wcstr;
use thindx::SafeHWND;

use winapi::um::debugapi::*;
use winapi::um::libloaderapi::*;
use winapi::um::wincon::*;
use winapi::um::winuser::*;

use std::ptr::*;

pub fn create_window(title: &str) -> SafeHWND<'static> {
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

        SafeHWND::assert_unbounded(hwnd).unwrap()
    }
}

#[doc(hidden)] pub fn test_window(two: bool) -> SafeHWND<'static> {
    if two {
        HWND2.with(|hwnd| hwnd.clone())
    } else {
        HWND1.with(|hwnd| hwnd.clone())
    }
}

thread_local! {
    pub static HWND1 : SafeHWND<'static> = create_window("thindx test window #1");
    pub static HWND2 : SafeHWND<'static> = create_window("thindx test window #2");
}

pub fn optional_dev_init() {
    if std::env::var_os("CARGO").is_some() {
        // reattach for panic logs/spam
        let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
        // might fail if somehow already attached, or triggered despite not having a parent console
    }

    if unsafe { IsDebuggerPresent() } != 0 {
        std::panic::set_hook(std::boxed::Box::new(|pi| unsafe {
            eprintln!("example paniced: {}", pi);
            DebugBreak();
            std::process::exit(1);
        }));
    }
}
