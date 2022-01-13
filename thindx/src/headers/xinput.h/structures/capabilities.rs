use crate::xinput::*;
use bytemuck::{Pod, Zeroable};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities)\]
/// XINPUT_CAPABILITIES
///
/// Battery type and charge.
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct Capabilities {
    /// Device type (generally always [DevType::Gamepad]?)
    pub ty:         DevType,

    /// Device "sub"type.
    ///
    /// **NOTE:** "Legacy" XInput (9.1.0 / Windows Vista) will always return [`DevSubType::Gamepad`], regardless of device.
    pub sub_type:   DevSubType,

    /// Capability flags.
    pub flags:      Caps,

    /// Describes available features and control resolutions.
    pub gamepad:    Gamepad,

    /// Describes available functionality and resolutions.
    pub vibration:  Vibration,
}

test_layout! {
    Capabilities => winapi::um::xinput::XINPUT_CAPABILITIES {
        ty          => Type,
        sub_type    => SubType,
        flags       => Flags,
        gamepad     => Gamepad,
        vibration   => Vibration,
    }
}
