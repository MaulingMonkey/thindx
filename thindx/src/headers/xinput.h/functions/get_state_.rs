use crate::*;
use crate::xinput::*;

use bytemuck::Zeroable;

use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)\]
/// XInputGetState
///
/// Retrieves the current state of the specified controller.
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - [`User`] gamepad not connected
pub fn get_state(user_index: impl Into<User>) -> Result<State, MethodError> {
    let mut state = State::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * tested        in `examples/d3d9-02-xinput.rs`
    //  * `user_index`  is well tested from 0 ..= 255 (but retest if the type of `user_index` expands to allow `u32`!)
    //  * `state`       is out-only, fixed size, no `cbSize` field, never null, all bit patterns sane
    let code = unsafe { XInputGetState(user_index.into().into(), &mut state as *mut _ as *mut _) };
    check_error_success("XInputGetState", code)?;
    Ok(state)
}

#[test] fn test_valid_params() {
    if let Err(err) = get_state(User::Zero ) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_state(User::Three) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(ERROR::BAD_ARGUMENTS, get_state(User::Any));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_state(User::from_unchecked(4)));
    assert_eq!(ERROR::BAD_ARGUMENTS, get_state(User::from_unchecked(254)));
}
