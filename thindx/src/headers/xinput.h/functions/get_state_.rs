use crate::xinput::*;
use bytemuck::Zeroable;
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)\]
/// XInputGetState
///
/// Retrieves the current state of the specified controller.
///
/// ### Errors
/// *   [ERROR::DEVICE_NOT_CONNECTED]
pub fn get_state(user_index: impl Into<User>) -> Result<State, MethodError> {
    let mut state = State::zeroed();
    let code = unsafe { XInputGetState(user_index.into().into(), &mut state as *mut _ as *mut _) };
    check_error_success("XInputGetState", code)?;
    Ok(state)
}
