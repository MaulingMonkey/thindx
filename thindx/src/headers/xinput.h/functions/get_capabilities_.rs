use crate::xinput::*;
use bytemuck::Zeroable;
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)\]
/// XInputGetCapabilities
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`Flag`]
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`Flag::None`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`User`] in bounds, but without a gamepad
pub fn get_capabilities(user_index: impl Into<User>, flags: Flag) -> Result<Capabilities, MethodError> {
    let mut caps = Capabilities::zeroed();
    let code = unsafe { XInputGetCapabilities(user_index.into().into(), flags.into(), &mut caps as *mut _ as *mut _) };
    check_error_success("XInputGetCapabilities", code)?;
    Ok(caps)
}

#[test] fn test_valid_params() {
    if let Err(err) = get_capabilities(User::Zero,  Flag::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(User::Three, Flag::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(User::Zero,  Flag::None   ) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_capabilities(User::Three, Flag::None   ) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_capabilities(User::Any,                 Flag::Gamepad));            // Bad User (any)
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_capabilities(User::from_unchecked(4),   Flag::Gamepad));            // Bad User (obb)
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_capabilities(User::Zero,                Flag::from_unchecked(42))); // Bad Flag (obb)
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_capabilities(User::Zero,                Flag::from_unchecked(!0))); // Bad Flag (obb)
}
