use crate::xinput::*;
use bytemuck::{Pod, Zeroable};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad)\]
/// XINPUT_GAMEPAD
///
/// Describes the state of an Xbox 360 controller.
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct Gamepad {

    /// Buttons of the gamepad that are currently held.
    pub buttons: Buttons,

    /// Analog trigger behind the left side of the gamepad.
    ///
    /// | Value | Description   |
    /// | -----:| ------------- |
    /// |     0 | Not held
    /// |    30 | `XINPUT_GAMEPAD_TRIGGER_THRESHOLD`
    /// |   255 | Fully held
    pub left_trigger: u8,

    /// Analog trigger behind the right side of the gamepad.
    ///
    /// | Value | Description   |
    /// | -----:| ------------- |
    /// |     0 | Not held
    /// |    30 | `XINPUT_GAMEPAD_TRIGGER_THRESHOLD`
    /// |   255 | Fully held
    pub right_trigger: u8,

    /// X-coordinate of the upper left thumbstick of the gamepad.
    ///
    /// | Value     | Description   |
    /// | ---------:| ------------- |
    /// |    -32768 | Fully left
    /// |     -7849 | `-XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE`
    /// |         0 | Horizontally centered
    /// |     +7849 | `+XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE`
    /// |    +32767 | Fully right
    pub left_thumb_x: i16,

    /// Y-coordinate of the upper left thumbstick of the gamepad.
    ///
    /// | Value     | Description   |
    /// | ---------:| ------------- |
    /// |    -32768 | Fully down
    /// |     -7849 | `-XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE`
    /// |         0 | Horizontally centered
    /// |     +7849 | `+XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE`
    /// |    +32767 | Fully up
    pub left_thumb_y: i16,

    /// X-coordinate of the right thumbstick of the gamepad.
    ///
    /// | Value     | Description   |
    /// | ---------:| ------------- |
    /// |    -32768 | Fully left
    /// |     -8689 | `-XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE`
    /// |         0 | Horizontally centered
    /// |     +8689 | `+XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE`
    /// |    +32767 | Fully right
    pub right_thumb_x: i16,

    /// Y-coordinate of the right thumbstick of the gamepad.
    ///
    /// | Value     | Description   |
    /// | ---------:| ------------- |
    /// |    -32768 | Fully down
    /// |     -8689 | `-XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE`
    /// |         0 | Horizontally centered
    /// |     +8689 | `+XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE`
    /// |    +32767 | Fully up
    pub right_thumb_y: i16,
}

// TODO: deadzone fns?
// TODO: pairing fns?

test_layout_only! {
    Gamepad => winapi::um::xinput::XINPUT_GAMEPAD {
        buttons         => wButtons,
        left_trigger    => bLeftTrigger,
        right_trigger   => bRightTrigger,
        left_thumb_x    => sThumbLX,
        left_thumb_y    => sThumbLY,
        right_thumb_x   => sThumbRX,
        right_thumb_y   => sThumbRY,
    }
}
