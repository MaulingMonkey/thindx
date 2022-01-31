use crate::*;
use winapi::um::xinput::*;
use bytemuck::{Pod, Zeroable};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad)\]
/// XINPUT_GAMEPAD
///
/// Describes the state of an Xbox 360 controller.
///
/// ### See Also
/// *   [examples::d3d9_02_xinput]
/// *   [Doing thumbstick dead zones right](https://web.archive.org/web/20141025190105/https://www.third-helix.com/2013/04/12/doing-thumbstick-dead-zones-right.html)
///     (**strongly** recommended reading!)
/// *   [Xbox 360 controller - Layout](https://en.wikipedia.org/wiki/Xbox_360_controller#Layout) - Wikipedia
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct Gamepad {

    /// Buttons of the gamepad that are currently held.
    pub buttons: xinput::Buttons,

    /// Analog trigger behind the left side of the gamepad.
    ///
    /// | Value | Description   |
    /// | -----:| ------------- |
    /// |     0 | Not held
    /// |    30 | [xinput::Gamepad::TRIGGER_THRESHOLD]
    /// |   255 | Fully held
    pub left_trigger: u8,

    /// Analog trigger behind the right side of the gamepad.
    ///
    /// | Value | Description   |
    /// | -----:| ------------- |
    /// |     0 | Not held
    /// |    30 | [xinput::Gamepad::TRIGGER_THRESHOLD]
    /// |   255 | Fully held
    pub right_trigger: u8,

    /// X-coordinate of the upper left thumbstick of the gamepad.
    ///
    /// | Value     | Description   |
    /// | ---------:| ------------- |
    /// |    -32768 | Fully left
    /// |     -7849 | - [xinput::Gamepad::LEFT_THUMB_DEADZONE]
    /// |         0 | Centered
    /// |     +7849 | + [xinput::Gamepad::LEFT_THUMB_DEADZONE]
    /// |    +32767 | Fully right
    pub left_thumb_x: i16,

    /// Y-coordinate of the upper left thumbstick of the gamepad.
    ///
    /// | Value     | Description   |
    /// | ---------:| ------------- |
    /// |    -32768 | Fully down
    /// |     -7849 | - [xinput::Gamepad::LEFT_THUMB_DEADZONE]
    /// |         0 | Centered
    /// |     +7849 | + [xinput::Gamepad::LEFT_THUMB_DEADZONE]
    /// |    +32767 | Fully up
    pub left_thumb_y: i16,

    /// X-coordinate of the right thumbstick of the gamepad.
    ///
    /// | Value     | Description   |
    /// | ---------:| ------------- |
    /// |    -32768 | Fully left
    /// |     -8689 | - [xinput::Gamepad::RIGHT_THUMB_DEADZONE]
    /// |         0 | Centered
    /// |     +8689 | + [xinput::Gamepad::RIGHT_THUMB_DEADZONE]
    /// |    +32767 | Fully right
    pub right_thumb_x: i16,

    /// Y-coordinate of the right thumbstick of the gamepad.
    ///
    /// | Value     | Description   |
    /// | ---------:| ------------- |
    /// |    -32768 | Fully down
    /// |     -8689 | - [xinput::Gamepad::RIGHT_THUMB_DEADZONE]
    /// |         0 | Centered
    /// |     +8689 | + [xinput::Gamepad::RIGHT_THUMB_DEADZONE]
    /// |    +32767 | Fully up
    pub right_thumb_y: i16,
}

impl Gamepad {
    /// An optional default threshhold to compare [Gamepad::left_trigger] or [Gamepad::right_trigger] to, before which you might avoid registering a trigger pull.
    ///
    /// In my experience, Xbox 360 controllers all perfectly report 0 trigger when untouched, so this is somewhat optional.
    /// On the other hand, there's no guarantee third party XInput compatible controllers behave the same, so perhaps you should use such a constant!
    pub const TRIGGER_THRESHOLD : u8 = XINPUT_GAMEPAD_TRIGGER_THRESHOLD;

    /// A default deadzone magnitude for the left thumbstick.
    ///
    /// You might ignore [Gamepad::left_thumb_x] / [Gamepad::left_thumb_y] when they're within this magnitude of (0,0).
    /// While this is generally large enough to avoid drift on new controllers, I have seen more well used controllers rest outside of this magnitude, resulting in suprising drift.
    /// For maximum flexibility, consider making the user's deadzone customizeable, and perhaps using a higher threshhold for triggering game UI navigation.
    ///
    /// I also **strongly** recommend reading [Doing thumbstick dead zones right](https://web.archive.org/web/20141025190105/https://www.third-helix.com/2013/04/12/doing-thumbstick-dead-zones-right.html).
    pub const LEFT_THUMB_DEADZONE : i16 = XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE;

    /// A default deadzone magnitude for the right thumbstick.
    ///
    /// You might ignore [Gamepad::right_thumb_x] / [Gamepad::right_thumb_y] when they're within this magnitude of (0,0).
    /// While this is generally large enough to avoid drift on new controllers, I have seen more well used controllers rest outside of this magnitude, resulting in suprising drift.
    /// For maximum flexibility, consider making the user's deadzone customizeable, and perhaps using a higher threshhold for triggering game UI navigation.
    ///
    /// I also **strongly** recommend reading [Doing thumbstick dead zones right](https://web.archive.org/web/20141025190105/https://www.third-helix.com/2013/04/12/doing-thumbstick-dead-zones-right.html).
    pub const RIGHT_THUMB_DEADZONE : i16 = XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE;
}

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

#[test] fn test_traits_for_coverage() {
    let _gamepad = Gamepad::default();
    let _gamepad = Gamepad::zeroed();
    let _gamepad = _gamepad.clone();
    dbg!(_gamepad);
}

//#cpp2rust XINPUT_GAMEPAD                      = xinput::Gamepad
//#cpp2rust XINPUT_GAMEPAD_TRIGGER_THRESHOLD    = xinput::Gamepad::TRIGGER_THRESHOLD
//#cpp2rust XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE  = xinput::Gamepad::LEFT_THUMB_DEADZONE
//#cpp2rust XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE = xinput::Gamepad::RIGHT_THUMB_DEADZONE
