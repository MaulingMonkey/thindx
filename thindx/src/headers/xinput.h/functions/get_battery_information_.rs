use crate::xinput::*;
use bytemuck::Zeroable;
use winapi::um::xinput::XInputGetBatteryInformation;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation)\]
/// XInputGetBatteryInformation
///
/// ### Arguments
/// *   `user_index`    Identify which user's controller to get the battery information of
/// *   `dev_type`      [BatteryDevType::Gamepad] or [BatteryDevType::Headset]
pub fn get_battery_information(user_index: impl Into<User>, dev_type: impl Into<BatteryDevType>) -> Result<BatteryInformation, MethodError> {
    let mut info = BatteryInformation::zeroed();
    let code = unsafe { XInputGetBatteryInformation(user_index.into().into(), dev_type.into().into(), &mut info as *mut _ as *mut _) };
    check_error_success("XInputGetBatteryInformation", code)?;
    Ok(info)
}
