use crate::*;
use crate::xinput::*;

use bytemuck::Zeroable;

use winapi::um::xinput::XInputGetBatteryInformation;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation)\]
/// XInputGetBatteryInformation
///
/// ### Arguments
/// *   `user_index`    Identify which user's controller to get the battery information of
/// *   `dev_type`      [BatteryDevType::Gamepad] or [BatteryDevType::Headset]
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - Disconnected [`User`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - Invalid [`BatteryDevType`]
pub fn get_battery_information(user_index: impl Into<User>, dev_type: impl Into<BatteryDevType>) -> Result<BatteryInformation, MethodError> {
    let mut info = BatteryInformation::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  is well tested from 0 ..= 255 (but retest if the type of `user_index` expands to allow `u32`!)
    //  * `dev_type`    is decently tested (0, 1, 2 (OOB), 42, 255 all result in defined behavior)
    //  * `info`        is out-only, no cbSize field, fixed size, sane
    let code = unsafe { XInputGetBatteryInformation(user_index.into().into(), dev_type.into().into(), &mut info as *mut _ as *mut _) };
    check_error_success("XInputGetBatteryInformation", code)?;
    Ok(info)
}

#[test] fn test_valid_params() {
    if let Err(err) = get_battery_information(User::Zero,  BatteryDevType::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(User::Three, BatteryDevType::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(User::Zero,  BatteryDevType::Headset) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(User::Three, BatteryDevType::Headset) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_battery_information(User::Any,                  BatteryDevType::Gamepad));
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_battery_information(User::from_unchecked(4),    BatteryDevType::Gamepad));
    assert_eq!(ERROR::DEVICE_NOT_CONNECTED, get_battery_information(User::Zero,                 BatteryDevType::from_unchecked(42)));
}
