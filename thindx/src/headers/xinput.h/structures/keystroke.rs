use crate::xinput::*;
use bytemuck::{Pod, Zeroable};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke)\]
/// XINPUT_KEYSTROKE
///
/// Describes a gamepad-provided keystroke.
#[derive(Clone, Copy, Debug)]
#[derive(Pod, Zeroable)]
#[repr(C)] pub struct Keystroke {
    /// Virtual key code of the key/button/stick movement.
    pub virtual_key:    VK,

    /// Documented as being unused?
    pub unicode:        u16,

    /// [Keystroke::KeyDown] | [Keystroke::KeyUp] | [Keystroke::Repeat]
    pub flags:          KeystrokeFlags,

    /// Index of the signed-in gamer associated with the device. Can be a value in the range 0–3.
    pub user_index:     User,

    /// HID code corresponding to the input. If there is no corresponding HID code, this value is zero.
    pub hid_code:       u8,
}

test_layout! {
    Keystroke => unsafe winapi::um::xinput::XINPUT_KEYSTROKE {
        virtual_key     => VirtualKey,
        unicode         => Unicode,
        flags           => Flags,
        user_index      => UserIndex,
        hid_code        => HidCode,
    }
}
