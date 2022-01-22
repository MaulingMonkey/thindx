use crate::*;
use crate::xinput::*;
use bytemuck::Zeroable;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)\]
/// XInputGetStateEx
///
/// ⚠️ **NOTE** ⚠️ This undocumented function is reserved for system software to access [Buttons::Guide].
///
/// Silently falls back on `XInputGetState` if `XInputGetStateEx` is unavailable.
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`User`] gamepad not connected
/// *   ~~[THINERR::MISSING_DLL_EXPORT]~~ - Silently falls back on [get_state] instead
#[deprecated = "This undocumented function is reserved for system software to access Buttons::Guide."]
pub fn get_state_ex(user_index: impl Into<User>) -> Result<State, MethodError> {
    #[allow(non_snake_case)]
    if let Some(XInputGetStateEx) = Imports::get()._XInputGetStateEx {
        let mut state = State::zeroed();
        // SAFETY: ✔️
        //  * fuzzed        in `tests/fuzz-xinput.rs`
        //  * tested        in `examples/xinput-exercise-all.rs` (Guide button works)
        //  * `user_index`  is well tested from 0 ..= 255 (but retest if the type of `user_index` expands to allow `u32`!)
        //  * `state`       is out-only, fixed size, no `cbSize` field, never null, all bit patterns sane
        //  * `fn`          should be `None` or valid if returned by `Imports::get()`
        let code = unsafe { XInputGetStateEx(user_index.into().into(), &mut state as *mut _ as *mut _) };
        check_error_success("XInputGetStateEx", code)?;
        Ok(state)
    } else {
        get_state(user_index)
    }
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

//#cpp2rust XInputGetStateEx    = xinput::get_state_ex
