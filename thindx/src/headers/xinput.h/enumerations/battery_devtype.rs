use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation)\]
/// BATTERY_DEVTYPE_\*
///
/// ### See Also
/// *   [xinput::get_battery_information](crate::xinput::get_battery_information)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)] // 0 = Gamepad
#[repr(transparent)] pub struct BatteryDevType(u8);

enumish! { BatteryDevType => u8; Gamepad, Headset }

#[allow(non_upper_case_globals)] impl BatteryDevType {
    /// Get the battery information for a gamepad
    pub const Gamepad : BatteryDevType = BatteryDevType(BATTERY_DEVTYPE_GAMEPAD);

    /// Get the battery information for a headset
    pub const Headset : BatteryDevType = BatteryDevType(BATTERY_DEVTYPE_HEADSET);
}

#[doc(hidden)] impl BatteryDevType {
    /// Get the battery information for a gamepad
    pub const GAMEPAD : BatteryDevType = BatteryDevType(BATTERY_DEVTYPE_GAMEPAD);

    /// Get the battery information for a headset
    pub const HEADSET : BatteryDevType = BatteryDevType(BATTERY_DEVTYPE_HEADSET);
}

//#cpp2rust BATTERY_DEVTYPE_GAMEPAD = xinput::BatteryDevType::Gamepad
//#cpp2rust BATTERY_DEVTYPE_HEADSET = xinput::BatteryDevType::Headset
