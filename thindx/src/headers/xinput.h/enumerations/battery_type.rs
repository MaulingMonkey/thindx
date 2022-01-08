#![allow(deprecated)]
use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/XInput/ns-xinput-xinput_battery_information)\]
/// BATTERY_TYPE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)] // 0 = Disconnected
#[repr(transparent)] pub struct BatteryType(u8);

enumish! { BatteryType => u8; Disconnected, Wired, Alkaline, NiMH, Unknown }

#[allow(non_upper_case_globals)] impl BatteryType {
    /// The device is not connected.
    pub const Disconnected : BatteryType = BatteryType(BATTERY_TYPE_DISCONNECTED as _);

    /// The device is a wired device, and does not have a battery.
    pub const Wired : BatteryType = BatteryType(BATTERY_TYPE_WIRED as _);

    /// The device has an alkaline battery.
    pub const Alkaline : BatteryType = BatteryType(BATTERY_TYPE_ALKALINE as _);

    /// The device has a **ni**ckle **m**etal **h**ydride battery.
    pub const NiMH : BatteryType = BatteryType(BATTERY_TYPE_NIMH as _);

    /// The device has an unknown battery type.
    #[deprecated = "Are you sure you want to use this?  New BATTERY_TYPE_* enumerations may be added at a later date..."]
    pub const Unknown : BatteryType = BatteryType(BATTERY_TYPE_UNKNOWN as _);
}

#[doc(hidden)] impl BatteryType {
    /// The device is not connected.
    pub const DISCONNECTED : BatteryType = BatteryType(BATTERY_TYPE_DISCONNECTED as _);

    /// The device is a wired device, and does not have a battery.
    pub const WIRED : BatteryType = BatteryType(BATTERY_TYPE_WIRED as _);

    /// The device has an alkaline battery.
    pub const ALKALINE : BatteryType = BatteryType(BATTERY_TYPE_ALKALINE as _);

    /// The device has a **ni**ckle **m**etal **h**ydride battery.
    pub const NIMH : BatteryType = BatteryType(BATTERY_TYPE_NIMH as _);

    /// The device has an unknown battery type.
    #[deprecated = "Are you sure you want to use this?  New BATTERY_TYPE_* enumerations may be added at a later date..."]
    pub const UNKNOWN : BatteryType = BatteryType(BATTERY_TYPE_UNKNOWN as _);
}
