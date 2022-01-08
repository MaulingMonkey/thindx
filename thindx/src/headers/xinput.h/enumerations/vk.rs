use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke#remarks)\]
/// VK_* values specific to Xbox 360 controllers
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)]
#[repr(transparent)] pub struct VK(u16);

enumish! {
    VK => u16;
    None, PadA, PadB, PadX, PadY, PadRShoulder, PadLShoulder, PadRTrigger, PadLTrigger,
    PadDPadUp, PadDPadDown, PadDPadLeft, PadDPadRight, PadStart, PadBack, PadLThumbPress, PadRThumbPress,
    PadLThumbUp, PadLThumbDown, PadLThumbRight, PadLThumbLeft, PadLThumbUpLeft, PadLThumbUpRight, PadLThumbDownRight, PadLThumbDownLeft,
    PadRThumbUp, PadRThumbDown, PadRThumbRight, PadRThumbLeft, PadRThumbUpLeft, PadRThumbUpRight, PadRThumbDownRight, PadRThumbDownLeft,
}

#[allow(non_upper_case_globals)] impl VK {
    /// No virtual key corresponding to this keystroke event (input method editor input?)
    pub const None : VK = VK(0);

    /// Typically synonymous select/accept in menus on Xbox 360 style controllers/games/console.
    ///
    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Green A   | Bottom button of the right face cluster
    /// | Xbox One      | A         | Bottom button of the right face cluster
    pub const PadA : VK = VK(VK_PAD_A);

    /// Typically synonymous back/cancel in menus on Xbox 360 style controllers/games/console.
    ///
    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Red B     | Right button of the right face cluster
    /// | Xbox One      | B         | Right button of the right face cluster
    pub const PadB : VK = VK(VK_PAD_B);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Blue X    | Left button of the right face cluster
    /// | Xbox One      | X         | Left button of the right face cluster
    pub const PadX : VK = VK(VK_PAD_X);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Yellow Y  | Top button of the right face cluster
    /// | Xbox One      | Y         | Top button of the right face cluster
    pub const PadY : VK = VK(VK_PAD_Y);

    /// | Controller    | Where |
    /// | ------------- | ----- |
    /// | Xbox 360      | Top right on the controller
    /// | Xbox One      | Top right on the controller
    pub const PadRShoulder : VK = VK(VK_PAD_RSHOULDER);

    /// | Controller    | Where |
    /// | ------------- | ----- |
    /// | Xbox 360      | Top left on the controller
    /// | Xbox One      | Top left on the controller
    pub const PadLShoulder : VK = VK(VK_PAD_LSHOULDER);

    /// Analog trigger behind the right side of the gamepad.
    pub const PadRTrigger : VK = VK(VK_PAD_RTRIGGER);

    /// Analog trigger behind the left side of the gamepad.
    pub const PadLTrigger : VK = VK(VK_PAD_LTRIGGER);

    /// | Controller    | Where |
    /// | ------------- | ----- |
    /// | Xbox 360      | Directional pad on the bottom left of the face
    /// | Xbox One      | Directional pad on the bottom left of the face
    pub const PadDPadUp : VK = VK(VK_PAD_DPAD_UP);

    /// | Controller    | Where |
    /// | ------------- | ------ |
    /// | Xbox 360      | Directional pad on the bottom left of the face
    /// | Xbox One      | Directional pad on the bottom left of the face
    pub const PadDPadDown : VK = VK(VK_PAD_DPAD_DOWN);

    /// | Controller    | Where |
    /// | ------------- | ----- |
    /// | Xbox 360      | Directional pad on the bottom left of the face
    /// | Xbox One      | Directional pad on the bottom left of the face
    pub const PadDPadLeft : VK = VK(VK_PAD_DPAD_LEFT);

    /// | Controller    | Where |
    /// | ------------- | ----- |
    /// | Xbox 360      | Directional pad on the bottom left of the face
    /// | Xbox One      | Directional pad on the bottom left of the face
    pub const PadDPadRight : VK = VK(VK_PAD_DPAD_RIGHT);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Start `ᐅ` | Right button of the middle face cluster
    /// | Xbox One      | Menu `≡`  | Right button of the middle face cluster
    pub const PadStart : VK = VK(VK_PAD_START);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Back `ᐊ` | Left button of the middle face cluster
    /// | Xbox One      | View `⧉` | Left button of the middle face cluster
    pub const PadBack : VK = VK(VK_PAD_BACK);

    /// | Controller    | When  |
    /// | ------------- | ----- |
    /// | Xbox 360      | Pressing the left thumbstick into the controller
    /// | Xbox One      | Pressing the left thumbstick into the controller
    pub const PadLThumbPress : VK = VK(VK_PAD_LTHUMB_PRESS);

    /// | Controller    | When  |
    /// | ------------- | ----- |
    /// | Xbox 360      | Pressing the right thumbstick into the controller
    /// | Xbox One      | Pressing the right thumbstick into the controller
    pub const PadRThumbPress : VK = VK(VK_PAD_RTHUMB_PRESS);

    /// Moved the upper left thumbstick up.
    pub const PadLThumbUp : VK = VK(VK_PAD_LTHUMB_UP);

    /// Moved the upper left thumbstick down.
    pub const PadLThumbDown : VK = VK(VK_PAD_LTHUMB_DOWN);

    /// Moved the upper left thumbstick right.
    pub const PadLThumbRight : VK = VK(VK_PAD_LTHUMB_RIGHT);

    /// Moved the upper left thumbstick left.
    pub const PadLThumbLeft : VK = VK(VK_PAD_LTHUMB_LEFT);

    /// Moved the upper left thumbstick up and left.
    pub const PadLThumbUpLeft : VK = VK(VK_PAD_LTHUMB_UPLEFT);

    /// Moved the upper left thumbstick up and right.
    pub const PadLThumbUpRight : VK = VK(VK_PAD_LTHUMB_UPRIGHT);

    /// Moved the upper left thumbstick down and right.
    pub const PadLThumbDownRight : VK = VK(VK_PAD_LTHUMB_DOWNRIGHT);

    /// Moved the upper left thumbstick and left.
    pub const PadLThumbDownLeft : VK = VK(VK_PAD_LTHUMB_DOWNLEFT);

    /// Moved the right thumbstick up.
    pub const PadRThumbUp : VK = VK(VK_PAD_RTHUMB_UP);

    /// Moved the right thumbstick down.
    pub const PadRThumbDown : VK = VK(VK_PAD_RTHUMB_DOWN);

    /// Moved the right thumbstick right.
    pub const PadRThumbRight : VK = VK(VK_PAD_RTHUMB_RIGHT);

    /// Moved the right thumbstick left.
    pub const PadRThumbLeft : VK = VK(VK_PAD_RTHUMB_LEFT);

    /// Moved the right thumbstick up and left.
    pub const PadRThumbUpLeft : VK = VK(VK_PAD_RTHUMB_UPLEFT);

    /// Moved the right thumbstick up and right.
    pub const PadRThumbUpRight : VK = VK(VK_PAD_RTHUMB_UPRIGHT);

    /// Moved the right thumbstick down and right.
    pub const PadRThumbDownRight : VK = VK(VK_PAD_RTHUMB_DOWNRIGHT);

    /// Moved the right thumbstick down and left.
    pub const PadRThumbDownLeft : VK = VK(VK_PAD_RTHUMB_DOWNLEFT);
}
