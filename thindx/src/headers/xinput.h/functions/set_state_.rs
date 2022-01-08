use crate::xinput::*;
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate)\]
/// XInputSetState
///
/// Control the vibration of a controller.
///
/// ### Errors
/// *   [ERROR::DEVICE_NOT_CONNECTED]
pub fn set_state(user_index: impl Into<User>, mut vibration: Vibration) -> Result<(), MethodError> {
    let code = unsafe { XInputSetState(user_index.into().into(), &mut vibration as *mut _ as *mut _) };
    check_error_success("XInputSetState", code)
}
