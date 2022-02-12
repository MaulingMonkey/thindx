//! ### ⚠️ Safety ⚠️
//! `HWND`s are pretty funky.
//! On the one hand, they're supposedly pointers, and in the single threaded days of 16-bit windows, they perhaps were.
//! Modern 32-bit windows has turned these into generational indicies of sorts, and has sunken a lot of time into making Win32 safe/sound despite userspace's abuse.
//! Windows may be owned by another thread, and thus destroyed at any time.
//! Windows may be owned by another process, and thus destroyed at any time.
//! Windows may be owned by another user, probably?
//! HWNDs should generally be treated as weak references without proper validity checking.

use crate::*;
use crate::d3d::Rect; // ...?

use winapi::um::winuser::*;



macro_rules! fn_err_get_last_error { () => { fn_err!(ErrorKind(get_last_error() as _)) } }

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)\]
/// HWND
///
/// A window handle.
/// Note that Window's definition of a "window" is [pretty expansive](https://docs.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-).
/// There's a reason it's named "Windows"!
///
/// Windows are "windows".
/// Buttons are "windows".
/// The desktop is a "window".
/// ~~You're a "window".~~
/// Windows belonging to other threads are windows.
/// Windows belonging to other processes are windows.
/// Windows belonging to other users... *maybe* you're isolated from those?  But probably not.
pub type HWND = winapi::shared::windef::HWND;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)\]
/// DestroyWindow
///
/// Destroys the specified window.
///
/// ### ⚠️ Safety ⚠️
/// *   Destroying a window out from underneath a 3D rendering API such as Direct3D is generally unsound.
/// *   Destroying a "destroyed" window is unlikely to be sound.  While windows itself can handle it,
///     it can result in multiple WM_DESTROY and WM_NCDESTROY events, which the underlying wndprocs likely can't handle.
/// *   Destroying a window belonging to another thread or process is an incredibly bad idea, if it even works.
/// *   Honestly, you should probably only destroy windows you created, in your own process, on your own thread, and even then be careful!
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # use std::ptr::*;
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, unsafe { win32::destroy_window(null_mut()) });
/// ```
pub unsafe fn destroy_window(hwnd: impl Into<HWND>) -> Result<(), Error> {
    fn_context!(win32::destroy_window => DestroyWindow);
    let hwnd = hwnd.into();
    let succeeded = unsafe { DestroyWindow(hwnd) != 0 };
    if succeeded { Ok(()) } else { fn_err_get_last_error!() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)\]
/// GetClientRect
///
/// Retrieves the coordinates of a window's client area.
/// The client coordinates specify the upper-left and lower-right corners of the client area.
/// Because client coordinates are relative to the upper-left corner of a window's client area, the coordinates of the upper-left corner are (0,0).
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # use std::ptr::*;
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, win32::get_client_rect(null_mut()));
/// let rect = win32::get_client_rect(win32::get_desktop_window()).unwrap();
/// assert_eq!(0, rect.left());
/// assert_eq!(0, rect.top());
/// assert!(0 != rect.width());
/// assert!(0 != rect.height());
/// # for p in 0 .. 8 * std::mem::size_of::<win32::HWND>() {
/// #   let e = win32::get_client_rect((1usize << p) as win32::HWND); // shouldn't crash
/// #   if e.is_err() { assert_eq!(ERROR::INVALID_WINDOW_HANDLE, e) }
/// # }
/// ```
pub fn get_client_rect(hwnd: impl TryInto<HWND>) -> Result<Rect, Error> {
    fn_context!(win32::get_client_rect => GetClientRect);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    let mut rect = Rect::zeroed();
    let succeeded = unsafe { GetClientRect(hwnd, rect.as_mut()) != 0 };
    if succeeded { Ok(rect) } else { fn_err_get_last_error!() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)\]
/// GetDesktopWindow
///
/// Retrieves a handle to the desktop window.
/// The desktop window covers the entire screen.
/// The desktop window is the area on top of which other windows are painted.
///
/// ### Example
/// ```rust
/// # use thindx::win32;
/// let hwnd = win32::get_desktop_window();
/// # assert!(win32::is_window(hwnd));
/// ```
#[must_use] pub fn get_desktop_window() -> HWND {
    fn_context!(win32::get_desktop_window => GetDesktopWindow);
    unsafe { GetDesktopWindow() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptra)\]
/// GetWindowLongPtrA
///
/// Retrieves information about the specified window. The function also retrieves the value at a specified offset into the extra window memory.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          If `index` is e.g. `GWLP_WNDPROC` for a `hwnd` belonging to a different process.
/// *   [ERROR::INVALID_INDEX]          If `index` isn't valid for `hwnd`
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` isn't valid
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # use winapi::um::winuser::{GWL_STYLE, GWLP_WNDPROC}; // FIXME: replace
/// # use std::ptr::*;
/// # let desktop = win32::get_desktop_window();
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, win32::get_window_long_ptr_a(null_mut(), 0));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, win32::get_window_long_ptr_a(!42usize as win32::HWND, 0));
/// assert_eq!(ERROR::INVALID_INDEX,         win32::get_window_long_ptr_a(desktop, -9001));
/// assert_eq!(ERROR::ACCESS_DENIED,         win32::get_window_long_ptr_a(desktop, GWLP_WNDPROC));
/// let desktop_style = win32::get_window_long_ptr_a(desktop, GWL_STYLE).unwrap();
/// # for i in [i32::MIN, i32::MIN/2, -9001, 0, 9001, i32::MAX/2, i32::MAX].iter().copied().chain(-64 ..= 64).chain((0..30).map(|p| 1<<p)).chain((0..31).map(|p| -(1<<p))) {
/// #   if let Err(err) = win32::get_window_long_ptr_a(desktop, i) {
/// #       match err.kind() {
/// #           ERROR::ACCESS_DENIED => {},
/// #           ERROR::INVALID_INDEX => {},
/// #           kind                 => panic!("get_window_long_ptr_a(desktop, {i}) == {kind:?}"),
/// #       }
/// #   }
/// # }
/// ```
pub fn get_window_long_ptr_a(hwnd: impl Into<HWND>, index: i32) -> Result<isize, Error> {
    fn_context!(win32::get_window_long_ptr_a => GetWindowLongPtrA);
    let hwnd = hwnd.into();
    let r = unsafe { GetWindowLongPtrA(hwnd, index) };
    match (r == 0).then(|| get_last_error()).unwrap_or(0) {
        0   => Ok(r as _), // i32 -> isize on 32-bit windows
        err => fn_err!(ErrorKind(err as _)),
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)\]
/// GetWindowLongPtrW
///
/// Retrieves information about the specified window. The function also retrieves the value at a specified offset into the extra window memory.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          If `index` is e.g. `GWLP_WNDPROC` for a `hwnd` belonging to a different process.
/// *   [ERROR::INVALID_INDEX]          If `index` isn't valid for `hwnd`
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` isn't valid
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # use winapi::um::winuser::{GWL_STYLE, GWLP_WNDPROC}; // FIXME: replace
/// # use std::ptr::*;
/// # let desktop = win32::get_desktop_window();
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, win32::get_window_long_ptr_w(null_mut(), 0));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, win32::get_window_long_ptr_w(!42usize as win32::HWND, 0));
/// assert_eq!(ERROR::INVALID_INDEX,         win32::get_window_long_ptr_w(desktop, -9001));
/// assert_eq!(ERROR::ACCESS_DENIED,         win32::get_window_long_ptr_w(desktop, GWLP_WNDPROC));
/// let desktop_style = win32::get_window_long_ptr_w(desktop, GWL_STYLE).unwrap();
/// # for i in [i32::MIN, i32::MIN/2, -9001, 0, 9001, i32::MAX/2, i32::MAX].iter().copied().chain(-64 ..= 64).chain((0..30).map(|p| 1<<p)).chain((0..31).map(|p| -(1<<p))) {
/// #   if let Err(err) = win32::get_window_long_ptr_w(desktop, i) {
/// #       match err.kind() {
/// #           ERROR::ACCESS_DENIED => {},
/// #           ERROR::INVALID_INDEX => {},
/// #           kind                 => panic!("get_window_long_ptr_w(desktop, {i}) == {kind:?}"),
/// #       }
/// #   }
/// # }
/// ```
pub fn get_window_long_ptr_w(hwnd: impl Into<HWND>, index: i32) -> Result<isize, Error> {
    fn_context!(win32::get_window_long_ptr_w => GetWindowLongPtrW);
    let hwnd = hwnd.into();
    let r = unsafe { GetWindowLongPtrW(hwnd, index) };
    match (r == 0).then(|| get_last_error()).unwrap_or(0) {
        0   => Ok(r as _), // i32 -> isize on 32-bit windows
        err => fn_err!(ErrorKind(err as _)),
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)\]
/// GetWindowThreadProcessId
///
/// Retrieves the identifier of the thread that created the specified window.
///
/// ### Returns
/// *   Ok(thread_id)
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # use std::ptr::*;
/// # let hwnd = win32::get_desktop_window();
/// let thread = win32::get_window_thread_id(hwnd).unwrap();
/// let hwnd_belongs_to_this_thread = thread == win32::get_current_thread_id();
/// # assert!(!hwnd_belongs_to_this_thread, "desktop doesn't belong to us!");
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, win32::get_window_thread_id(null_mut()));
/// # for p in 0 .. 8 * std::mem::size_of::<win32::HWND>() {
/// #   let e = win32::get_window_thread_id((1usize << p) as win32::HWND); // shouldn't crash
/// #   if e.is_err() { assert_eq!(ERROR::INVALID_WINDOW_HANDLE, e) }
/// # }
/// ```
///
/// ### See Also
/// *   [win32::get_window_thread_process_id]
#[must_use] pub fn get_window_thread_id(hwnd: impl Into<HWND>) -> Result<u32, Error> {
    fn_context!(win32::get_window_thread_id => GetWindowThreadProcessId);
    let hwnd = hwnd.into();
    let tid = unsafe { GetWindowThreadProcessId(hwnd, std::ptr::null_mut()) };
    if tid != 0 { Ok(tid) } else { fn_err_get_last_error!() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)\]
/// GetWindowThreadProcessId
///
/// Retrieves the identifier of the thread that created the specified window and the identifier of the process that created the window.
///
/// ### Returns
/// *   Ok((thread_id, process_id))
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```rust
/// # use thindx::*;
/// # use std::ptr::*;
/// # let hwnd = win32::get_desktop_window();
/// let (thread, process) = win32::get_window_thread_process_id(hwnd).unwrap();
/// let hwnd_belongs_to_this_thread = thread == win32::get_current_thread_id();
/// # assert!(!hwnd_belongs_to_this_thread, "desktop doesn't belong to us!");
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, win32::get_window_thread_process_id(null_mut()));
/// # for p in 0 .. 8 * std::mem::size_of::<win32::HWND>() {
/// #   let e = win32::get_window_thread_process_id((1usize << p) as win32::HWND); // shouldn't crash
/// #   if e.is_err() { assert_eq!(ERROR::INVALID_WINDOW_HANDLE, e) }
/// # }
/// ```
///
/// ### See Also
/// *   [win32::get_window_thread_id]
#[must_use] pub fn get_window_thread_process_id(hwnd: impl Into<HWND>) -> Result<(u32, u32), Error> {
    fn_context!(win32::get_window_thread_process_id => GetWindowThreadProcessId);
    let hwnd = hwnd.into();
    let mut pid = 0;
    let tid = unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    if tid != 0 { Ok((tid, pid)) } else { fn_err_get_last_error!() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)\]
/// IsWindow
///
/// Determines whether the specified window handle identifies an existing window.
///
/// Valid uses of this function are few and far between - windows belonging to another thread
/// or process could be destroyed immediately after this returns true, and invalidated handles
/// might suddenly spring back to life as the handle value is reused.
///
/// ### Example
/// ```rust
/// # use thindx::win32;
/// # let valid_hwnd = win32::get_desktop_window();
/// # let invalid_hwnd : win32::HWND = !42 as _;
/// assert!( win32::is_window(  valid_hwnd));
/// assert!(!win32::is_window(invalid_hwnd));
/// assert!(!win32::is_window(std::ptr::null_mut()));
/// # for p in 0 .. 8 * std::mem::size_of::<win32::HWND>() {
/// #   let _ = win32::is_window((1usize << p) as win32::HWND); // shouldn't crash
/// # }
/// ```
#[must_use] pub fn is_window(hwnd: impl TryInto<HWND>) -> bool {
    fn_context!(win32::is_window => IsWindow);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsWindow(hwnd) != 0 },
        Err(_) => false,
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)\]
/// IsWindowUnicode
///
/// Determines whether the specified window is a native Unicode window.
///
/// ### Returns
/// *   `true`  if the window's class was registered with `RegisterClassW`
/// *   `false` if the window's class was registered with `RegisterClassA`
/// *   `false` if the window isn't valid, probably?  (TryInto failed, HWND null/dangling/destroyed, ...)
///
/// ### Example
/// ```rust
/// # use thindx::win32;
/// # let unicode_hwnd = win32::get_desktop_window(); // TODO: replace with an explicitly created unicode hwnd
/// assert!( win32::is_window_unicode(unicode_hwnd));
/// assert!(!win32::is_window_unicode(std::ptr::null_mut()));
/// # for p in 0 .. 8 * std::mem::size_of::<win32::HWND>() {
/// #   let _ = win32::is_window_unicode((1usize << p) as win32::HWND); // shouldn't crash
/// # }
/// ```
#[must_use] pub fn is_window_unicode(hwnd: impl TryInto<HWND>) -> bool {
    fn_context!(win32::is_window_unicode => IsWindowUnicode);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsWindowUnicode(hwnd) != 0 },
        Err(_) => false,
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)\]
/// IsWindowVisible
///
/// Determines whether the specified window is WS_VISIBLE.
/// May return `true` even if the window is totally obscured by other windows, clipped out-of-bounds, etc.
///
/// ### Example
/// ```rust
/// # use thindx::win32;
/// # let hwnd = win32::get_desktop_window(); // TODO: replace with an explicitly created unicode hwnd
/// assert!( win32::is_window_visible(hwnd));
/// assert!(!win32::is_window_visible(std::ptr::null_mut()));
/// # for p in 0 .. 8 * std::mem::size_of::<win32::HWND>() {
/// #   let _ = win32::is_window_visible((1usize << p) as win32::HWND); // shouldn't crash
/// # }
/// ```
#[must_use] pub fn is_window_visible(hwnd: impl TryInto<HWND>) -> bool {
    fn_context!(win32::is_window_unicode => IsWindowVisible);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsWindowVisible(hwnd) != 0 },
        Err(_) => false,
    }
}



#[must_use] fn get_last_error() -> u32 {
    unsafe { winapi::um::errhandlingapi::GetLastError() }
}
