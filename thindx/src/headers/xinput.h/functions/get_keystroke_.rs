use crate::*;
use crate::xinput::*;

use bytemuck::Zeroable;

use winapi::shared::winerror::ERROR_EMPTY;
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke)\]
/// XInputGetKeystroke
///
/// Retrieves gamepad input events.
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - Disconnected [`User`]
pub fn get_keystroke(user_index: impl Into<u32>, _reserved: ()) -> Result<Option<Keystroke>, MethodError> {
    let mut keystroke = Keystroke::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * tested        in `examples/xinput-exercise-all.rs`
    //  * `user_index`  is well tested from 0 ..= 255 (but retest if the type of `user_index` expands to allow `u32`!)
    let code = unsafe { XInputGetKeystroke(user_index.into().into(), 0, &mut keystroke as *mut _ as *mut _) };
    if code == ERROR_EMPTY { return Ok(None) }
    check_error_success("XInputGetKeystroke", code)?;
    Ok(Some(keystroke))
}

#[test] fn test_valid_args() {
    if let Err(err) = get_keystroke(0u32,       ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_keystroke(1u32,       ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_keystroke(2u32,       ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_keystroke(3u32,       ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_keystroke(User::Any,  ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
}

#[test] fn test_invalid_args() {
    assert_eq!(ERROR::BAD_ARGUMENTS, get_keystroke(4u32, ()));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_keystroke(99u32, ()));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_keystroke(254u32, ()));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_keystroke(9001u32, ()));
}

//#cpp2rust XInputGetKeystroke  = xinput::get_keystroke
