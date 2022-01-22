use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities#parameters)\]
/// XINPUT_FLAG_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)] // 0 = No flags
#[repr(transparent)] pub struct Flag(u32);

flags! { Flag => u32; None, Gamepad }

#[allow(non_upper_case_globals)] impl Flag {
    /// No flags set
    pub const None : Flag = Flag(0);

    /// [XINPUT_FLAG_GAMEPAD](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)
    ///
    /// Limit [xinput::get_capabilities](crate::xinput::get_capabilities) to Xbox 360 controllers.
    pub const Gamepad : Flag = Flag(XINPUT_FLAG_GAMEPAD);
}

#[doc(hidden)] impl Flag {
    /// [XINPUT_FLAG_GAMEPAD](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities)
    ///
    /// Limit [xinput::get_capabilities](crate::xinput::get_capabilities) to Xbox 360 controllers.
    pub const GAMEPAD : Flag = Flag(XINPUT_FLAG_GAMEPAD);
}

//#cpp2rust XINPUT_FLAG_GAMEPAD     = xinput::Flag::Gamepad
