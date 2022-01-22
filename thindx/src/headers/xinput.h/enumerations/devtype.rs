use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities)\]
/// XINPUT_DEVTYPE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct DevType(u8);

enumish! { DevType => u8; Gamepad }

#[allow(non_upper_case_globals)] impl DevType {
    /// The device is a game controller.
    pub const Gamepad : DevType = DevType(XINPUT_DEVTYPE_GAMEPAD as _);
}

#[doc(hidden)] impl DevType {
    /// The device is a game controller.
    pub const GAMEPAD : DevType = DevType(XINPUT_DEVTYPE_GAMEPAD as _);
}

//#cpp2rust XINPUT_DEVTYPE_GAMEPAD = xinput::DevType::Gamepad
