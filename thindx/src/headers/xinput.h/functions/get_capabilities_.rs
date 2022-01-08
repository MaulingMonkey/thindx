use crate::xinput::*;
use bytemuck::Zeroable;
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)\]
/// XInputGetCapabilities
///
/// ### Errors
/// *   [ERROR::DEVICE_NOT_CONNECTED]
pub fn get_capabilities(user_index: impl Into<User>, flags: Flag) -> Result<Capabilities, MethodError> {
    let mut caps = Capabilities::zeroed();
    let code = unsafe { XInputGetCapabilities(user_index.into().into(), flags.into(), &mut caps as *mut _ as *mut _) };
    check_error_success("XInputGetCapabilities", code)?;
    Ok(caps)
}
