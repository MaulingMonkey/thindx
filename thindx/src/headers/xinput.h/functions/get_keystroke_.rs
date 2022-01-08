use crate::xinput::*;
use bytemuck::Zeroable;
use winapi::shared::winerror::ERROR_EMPTY;
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke)\]
/// XInputGetKeystroke
///
/// Retrieves the current state of the specified controller.
///
/// ### Errors
/// *   [ERROR::DEVICE_NOT_CONNECTED]
pub fn get_keystroke(user_index: impl Into<User>, _reserved: ()) -> Result<Option<Keystroke>, MethodError> {
    let mut keystroke = Keystroke::zeroed();
    let code = unsafe { XInputGetKeystroke(user_index.into().into(), 0, &mut keystroke as *mut _ as *mut _) };
    if code == ERROR_EMPTY { return Ok(None) }
    check_error_success("XInputGetKeystroke", code)?;
    Ok(Some(keystroke))
}
