#![allow(deprecated)]
use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/XInput/ns-xinput-xinput_battery_information)\]
/// BATTERY_TYPE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct BatteryType(u8);

enumish! { BatteryType => u8; default: Disconnected == 0; Disconnected, Wired, Alkaline, NiMH, Unknown }

#[allow(non_upper_case_globals)] impl BatteryType {
    /// The device is not connected.
    pub const Disconnected : BatteryType = BatteryType(BATTERY_TYPE_DISCONNECTED as _); // 0

    /// The device is a wired device, and does not have a battery.
    pub const Wired : BatteryType = BatteryType(BATTERY_TYPE_WIRED as _);

    /// The device has an alkaline battery.
    pub const Alkaline : BatteryType = BatteryType(BATTERY_TYPE_ALKALINE as _);

    /// The device has a **ni**ckle **m**etal **h**ydride battery.
    pub const NiMH : BatteryType = BatteryType(BATTERY_TYPE_NIMH as _);

    /// The device has an unknown battery type.
    #[deprecated = "Are you sure you want to use this?  New BATTERY_TYPE_* enumerations may be added at a later date..."]
    pub const Unknown : BatteryType = BatteryType(BATTERY_TYPE_UNKNOWN as _); // 255
}

//#cpp2rust BATTERY_TYPE_DISCONNECTED   = xinput::BatteryType::Disconnected
//#cpp2rust BATTERY_TYPE_WIRED          = xinput::BatteryType::Wired
//#cpp2rust BATTERY_TYPE_ALKALINE       = xinput::BatteryType::Alkaline
//#cpp2rust BATTERY_TYPE_NIMH           = xinput::BatteryType::NiMH
//#cpp2rust BATTERY_TYPE_UNKNOWN        = xinput::BatteryType::Unknown
