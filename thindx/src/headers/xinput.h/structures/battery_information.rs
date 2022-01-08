use crate::xinput::*;
use bytemuck::{Pod, Zeroable};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/XInput/ns-xinput-xinput_battery_information)\]
/// XINPUT_BATTERY_INFORMATION
///
/// Battery type and charge.
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct BatteryInformation {
    /// The type of battery.
    pub battery_type:   BatteryType,

    /// The charge state of the battery.
    /// This value is only valid for wireless devices with a known battery type.
    pub battery_level:  BatteryLevel,
}

test_layout! {
    BatteryInformation => unsafe winapi::um::xinput::XINPUT_BATTERY_INFORMATION {
        battery_type    => BatteryType,
        battery_level   => BatteryLevel,
    }
}
