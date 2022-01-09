use crate::xinput::*;
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate)\]
/// XInputSetState
///
/// Control the vibration of a controller.
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`User`] is not connected
pub fn set_state(user_index: impl Into<User>, mut vibration: Vibration) -> Result<(), MethodError> {
    let code = unsafe { XInputSetState(user_index.into().into(), &mut vibration as *mut _ as *mut _) };
    check_error_success("XInputSetState", code)
}

#[test] fn test_valid_params() {
    let v = Vibration::default();
    if let Err(err) = set_state(User::Zero,  v) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(User::Three, v) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(User::Zero,  v) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = set_state(User::Three, v) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    let v = Vibration::default();
    assert_eq!(ERROR::BAD_ARGUMENTS, set_state(User::Any,               v));
    assert_eq!(ERROR::BAD_ARGUMENTS, set_state(User::from_unchecked(4), v));
}
