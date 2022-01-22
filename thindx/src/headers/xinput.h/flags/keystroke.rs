use crate::xinput::Keystroke as Struct;
use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke#members)\]
/// XINPUT_KEYSTROKE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)] // 0 = No flags
#[repr(transparent)] pub struct Keystroke(u16);

flags! { Keystroke => u16; None, KeyDown, KeyUp, Repeat }

#[allow(non_upper_case_globals)] impl Keystroke {
    /// No flags set
    pub const None : Keystroke = Keystroke(0);

    /// The key was pressed.
    pub const KeyDown : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYDOWN);

    /// The key was released.
    pub const KeyUp : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYUP);

    /// This was a repeated key event.
    pub const Repeat : Keystroke = Keystroke(XINPUT_KEYSTROKE_REPEAT);
}

#[allow(non_upper_case_globals)] impl Struct {
    /// No flags set
    pub const None : Keystroke = Keystroke(0);

    /// The key was pressed.
    pub const KeyDown : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYDOWN);

    /// The key was released.
    pub const KeyUp : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYUP);

    /// This was a repeated key event.
    pub const Repeat : Keystroke = Keystroke(XINPUT_KEYSTROKE_REPEAT);
}

#[doc(hidden)] impl Struct {
    /// No flags set
    pub const NONE : Keystroke = Keystroke(0);

    /// The key was pressed.
    pub const KEYDOWN : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYDOWN);

    /// The key was released.
    pub const KEYUP : Keystroke = Keystroke(XINPUT_KEYSTROKE_KEYUP);

    /// This was a repeated key event.
    pub const REPEAT : Keystroke = Keystroke(XINPUT_KEYSTROKE_REPEAT);
}

//#cpp2rust XINPUT_KEYSTROKE_KEYDOWN    = xinput::Keystroke::KeyDown
//#cpp2rust XINPUT_KEYSTROKE_KEYUP      = xinput::Keystroke::KeyUp
//#cpp2rust XINPUT_KEYSTROKE_REPEAT     = xinput::Keystroke::Repeat
