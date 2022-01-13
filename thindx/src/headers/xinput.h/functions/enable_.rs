use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputenable)\]
/// XInputEnable
///
/// Meant to be called when an application gains or loses focus
/// (such as via [WM_ACTIVATEAPP](https://docs.microsoft.com/en-us/windows/desktop/winmsg/wm-activateapp)\),
/// to enable or disable XInput for this app.
///
/// "Disabling" xinput with `xinput::enable(false)` will:
/// *   Stop all vibration
/// *   Cause [xinput::get_state](crate::xinput::get_state) to retrieve neutral data (no buttons held, 0 axises)
///
/// ### Arguments
/// *   `enable` - `true` to accept input and allow vibration, `false` to block input and vibration
///
/// ### Example
/// ```rust
/// # use thindx::xinput;
/// # use winapi::shared::minwindef::*;
/// # use winapi::shared::windef::*;
/// # use winapi::um::winuser::*;
/// unsafe extern "system"
/// fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
///     match msg {
///         WM_ACTIVATEAPP => {
///             // assumes this is the only window of this app accepting xinput
///             xinput::enable(wparam as BOOL != FALSE);
///             0
///         },
///         _ => DefWindowProcW(hwnd, msg, wparam, lparam),
///     }
/// }
/// ```
pub fn enable(enable: bool) {
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `enable`      can be true or false.  Pretty easy to have exhaustive test coverage.
    unsafe { XInputEnable(enable.into()) }
}

#[test] fn spam_xinput_enable() {
    enable(true);
    enable(true);
    enable(true);
    enable(false);
    enable(false);
    enable(false);
    enable(true);
    enable(false);
    enable(true);
    enable(false);
    enable(true);
    enable(false);
    enable(true);
}
