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
pub fn get_keystroke(user_index: impl Into<User>, _reserved: ()) -> Result<Option<Keystroke>, MethodError> {
    let mut keystroke = Keystroke::zeroed();
    let code = unsafe { XInputGetKeystroke(user_index.into().into(), 0, &mut keystroke as *mut _ as *mut _) };
    if code == ERROR_EMPTY { return Ok(None) }
    check_error_success("XInputGetKeystroke", code)?;
    Ok(Some(keystroke))
}

#[test] fn test_valid_args() {
    if let Err(err) = get_keystroke(User::Zero,  ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_keystroke(User::One,   ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_keystroke(User::Two,   ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_keystroke(User::Three, ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_keystroke(User::Any,   ()) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
}

#[test] fn test_invalid_args() {
    assert_eq!(ERROR::BAD_ARGUMENTS, get_keystroke(User::from_unchecked(4), ()));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_keystroke(User::from_unchecked(99), ()));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_keystroke(User::from_unchecked(254), ()));
}
