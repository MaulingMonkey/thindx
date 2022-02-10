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
pub fn get_battery_information(user_index: impl Into<u32>, dev_type: impl Into<BatteryDevType>) -> Result<BatteryInformation, MethodError> {
    fn_context!(xinput::get_battery_information => XInputGetBatteryInformation);
    let mut info = BatteryInformation::zeroed();
    // SAFETY: ✔️
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  is well tested
    //  * `dev_type`    is decently tested (0, 1, 2 (OOB), 42, 255 all result in defined behavior)
    //  * `info`        is out-only, no cbSize field, fixed size, sane
    let code = unsafe { XInputGetBatteryInformation(user_index.into(), dev_type.into().into(), info.as_mut()) };
    check_success!(code)?;
    Ok(info)
}

#[test] fn test_valid_params() {
    if let Err(err) = get_battery_information(0u32, BatteryDevType::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(1u32, BatteryDevType::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(2u32, BatteryDevType::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(3u32, BatteryDevType::Gamepad) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }

    if let Err(err) = get_battery_information(0u32, BatteryDevType::Headset) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(1u32, BatteryDevType::Headset) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(2u32, BatteryDevType::Headset) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
    if let Err(err) = get_battery_information(3u32, BatteryDevType::Headset) { assert_eq!(err.kind(), ERROR::DEVICE_NOT_CONNECTED); }
}

#[test] fn test_bad_arguments() {
    assert_eq!(ERROR::DEVICE_NOT_CONNECTED, get_battery_information(User::Zero, BatteryDevType::from_unchecked(42))); // bad devtype
    assert_eq!(ERROR::BAD_ARGUMENTS,        get_battery_information(User::Any,  BatteryDevType::Gamepad));
    for u in User::iter_invalid() {
        assert_eq!(ERROR::BAD_ARGUMENTS, get_battery_information(u, BatteryDevType::Gamepad));
    }
}
