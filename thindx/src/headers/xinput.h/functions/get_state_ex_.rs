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
/// *   [ERROR::DEVICE_NOT_CONNECTED]
#[deprecated = "This undocumented function is reserved for system software to access [Buttons::Guide]."]
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
