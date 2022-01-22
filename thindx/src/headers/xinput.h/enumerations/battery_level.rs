use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/XInput/ns-xinput-xinput_battery_information)\]
/// BATTERY_LEVEL_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)] // 0 = Empty
#[repr(transparent)] pub struct BatteryLevel(u8);

enumish! { BatteryLevel => u8; Empty, Low, Medium, Full }

#[allow(non_upper_case_globals)] #[allow(missing_docs)] impl BatteryLevel {
    pub const Empty     : BatteryLevel = BatteryLevel(BATTERY_LEVEL_EMPTY as _);
    pub const Low       : BatteryLevel = BatteryLevel(BATTERY_LEVEL_LOW as _);
    pub const Medium    : BatteryLevel = BatteryLevel(BATTERY_LEVEL_MEDIUM as _);
    pub const Full      : BatteryLevel = BatteryLevel(BATTERY_LEVEL_FULL as _);
}

#[doc(hidden)] impl BatteryLevel {
    pub const EMPTY     : BatteryLevel = BatteryLevel(BATTERY_LEVEL_EMPTY as _);
    pub const LOW       : BatteryLevel = BatteryLevel(BATTERY_LEVEL_LOW as _);
    pub const MEDIUM    : BatteryLevel = BatteryLevel(BATTERY_LEVEL_MEDIUM as _);
    pub const FULL      : BatteryLevel = BatteryLevel(BATTERY_LEVEL_FULL as _);
}

//#cpp2rust BATTERY_LEVEL_EMPTY     = xinput::BatteryLevel::Empty
//#cpp2rust BATTERY_LEVEL_LOW       = xinput::BatteryLevel::Low
//#cpp2rust BATTERY_LEVEL_MEDIUM    = xinput::BatteryLevel::Medium
//#cpp2rust BATTERY_LEVEL_FULL      = xinput::BatteryLevel::Full
