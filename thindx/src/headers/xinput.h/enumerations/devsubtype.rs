#[cfg_attr(not(doc), allow(unused_imports))] use crate::xinput::*;
use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;
const XINPUT_DEVSUBTYPE_FLIGHT_STICK : u8 = 0x04; // winapi missing def



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/xinput/xinput-and-controller-subtypes)\]
/// XINPUT_DEVSUBTYPE_\*
///
/// Subtypes of [DevType::Gamepad].
///
/// **NOTE:** "Legacy" XInput (9.1.0 / Windows Vista) will always return [`DevSubType::Gamepad`], regardless of device.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)] // 0 = Unknown
#[repr(transparent)] pub struct DevSubType(u8);

enumish! { DevSubType => u8; Unknown }

#[allow(non_upper_case_globals)] impl DevSubType {
    /// An unknown style of Xbox 360 controller.
    pub const Unknown : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_UNKNOWN);

    /// A typical Xbox 360 gamepad, or fallback on XInput 9.1.0 / Windows Vista.
    pub const Gamepad : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_GAMEPAD);

    /// A wheel (such as the [Xbox 360 Wireless Racing Wheel](https://en.wikipedia.org/wiki/Xbox_360_Wireless_Racing_Wheel)?)
    pub const Wheel : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_WHEEL);

    #[allow(missing_docs)] // Would be nice to track down an actual example
    pub const ArcadeStick : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_ARCADE_STICK);

    #[allow(missing_docs)] // Would be nice to track down an actual example
    pub const FlightStick : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_FLIGHT_STICK);

    /// A dance pad (such as the one provided with [DDR](https://en.wikipedia.org/wiki/Dance_Dance_Revolution_Universe) and other dancing games?
    pub const DancePad : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_DANCE_PAD);

    /// A guitar (such as the one bundled with Guitar Hero or Rock Band?)
    pub const Guitar : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_GUITAR);

    /// A guitar (such as the one bundled with Guitar Hero or Rock Band?)
    pub const GuitarAlternate : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_GUITAR_ALTERNATE);

    /// A drum kit (such as the one bundled with Rock Band?)
    pub const DrumKit : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_DRUM_KIT);

    /// A bass guitar (such as the one bundled with Guitar Hero or Rock Band?)
    pub const GuitarBass : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_GUITAR_BASS);

    #[allow(missing_docs)] // Would be nice to track down an actual example
    pub const ArcadePad : DevSubType = DevSubType(XINPUT_DEVSUBTYPE_ARCADE_PAD);
}

#[doc(hidden)] impl DevSubType {
}
