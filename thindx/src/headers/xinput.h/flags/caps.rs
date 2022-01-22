use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities#members)\]
/// XINPUT_CAPS_\*
///
/// Bitmask of the device digital caps of an Xbox 360 style gamepad.
///
/// ### See Also
/// *   [Xbox 360 controller: Layout](https://en.wikipedia.org/wiki/Xbox_360_controller#Layout) (Wikipedia)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)] // 0 = No caps
#[repr(transparent)] pub struct Caps(u16);

flags! { Caps => u16; None, VoiceSupported, FfbSupported, Wireless, PmdSupported, NoNavigation }

#[allow(non_upper_case_globals)] impl Caps {
    /// No capabilities are held.
    pub const None              : Caps = Caps(0);

    /// Device has an integrated voice device.
    pub const VoiceSupported    : Caps = Caps(XINPUT_CAPS_VOICE_SUPPORTED);

    /// **F**orce **F**eed**b**ack is supported.
    pub const FfbSupported      : Caps = Caps(XINPUT_CAPS_FFB_SUPPORTED);

    /// The device is wireless.
    pub const Wireless          : Caps = Caps(XINPUT_CAPS_WIRELESS);

    /// **P**lug-in **M**o**d**ules are supported.
    ///
    /// **NOTE:** Plug-in modules like the text input device (TID) may not be supported on Windows.
    pub const PmdSupported      : Caps = Caps(XINPUT_CAPS_PMD_SUPPORTED);

    /// Device lacks menu navigation buttons (START, BACK, DPAD).
    pub const NoNavigation      : Caps = Caps(XINPUT_CAPS_NO_NAVIGATION);
}

#[doc(hidden)] impl Caps {
    /// No capabilities are held.
    pub const NONE              : Caps = Caps(0);

    /// Device has an integrated voice device.
    pub const VOICE_SUPPORTED   : Caps = Caps(XINPUT_CAPS_VOICE_SUPPORTED);

    /// **F**orce **F**eed**b**ack is supported.
    pub const FFB_SUPPORTED     : Caps = Caps(XINPUT_CAPS_FFB_SUPPORTED);

    /// The device is wireless.
    pub const WIRELESS          : Caps = Caps(XINPUT_CAPS_WIRELESS);

    /// **P**lug-in **M**o**d**ules are supported.
    ///
    /// **NOTE:** Plug-in modules like the text input device (TID) may not be supported on Windows.
    pub const PMD_SUPPORTED     : Caps = Caps(XINPUT_CAPS_PMD_SUPPORTED);

    /// Device lacks menu navigation buttons (START, BACK, DPAD).
    pub const NO_NAVIGATION     : Caps = Caps(XINPUT_CAPS_NO_NAVIGATION);
}


//#cpp2rust XINPUT_CAPS_VOICE_SUPPORTED     = xinput::Caps::VoiceSupported
//#cpp2rust XINPUT_CAPS_FFB_SUPPORTED       = xinput::Caps::FfbSupported
//#cpp2rust XINPUT_CAPS_WIRELESS            = xinput::Caps::Wireless
//#cpp2rust XINPUT_CAPS_PMD_SUPPORTED       = xinput::Caps::PmdSupported
//#cpp2rust XINPUT_CAPS_NO_NAVIGATION       = xinput::Caps::NoNavigation
