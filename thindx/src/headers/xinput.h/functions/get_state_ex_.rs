use crate::*;
use crate::xinput::*;
use bytemuck::Zeroable;
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)\]
/// XInputGetStateEx
///
/// **⚠️ NOTE ⚠️:** This undocumented function is reserved for system software to access [Buttons::Guide].
///
/// Silently falls back on `XInputGetState` if `XInputGetStateEx` is unavailable.
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`User`] gamepad not connected
/// *   ~~[THINERR::MISSING_DLL_EXPORT]~~ - Silently falls back on [get_state] instead
#[deprecated = "This undocumented function is reserved for system software to access Buttons::Guide."]
pub fn get_state_ex(user_index: impl Into<User>) -> Result<State, MethodError> {
    let user_index = user_index.into().into();
    let mut state = State::zeroed();

    #[allow(non_snake_case)]
    if let Some(XInputGetStateEx) = Imports::get()._XInputGetStateEx {
        let code = unsafe { XInputGetStateEx(user_index, &mut state as *mut _ as *mut _) };
        check_error_success("XInputGetStateEx", code)?;
    } else {
        let code = unsafe { XInputGetState(user_index, &mut state as *mut _ as *mut _) };
        check_error_success("XInputGetState", code)?;
    };
    Ok(state)
}

#[test] #[allow(deprecated)] fn test_valid_params() {
    if let Err(err) = get_state_ex(User::Zero ) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state_ex(User::Three) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
}

#[test] #[allow(deprecated)] fn test_bad_arguments() {
    assert_eq!(ERROR::BAD_ARGUMENTS, get_state_ex(User::Any));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_state_ex(User::from_unchecked(4)));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_state_ex(User::from_unchecked(254)));
}
