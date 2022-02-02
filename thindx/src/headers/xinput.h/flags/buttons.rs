#![allow(deprecated)] // Guide button flags! impl

use bytemuck::{Pod, Zeroable};
use winapi::um::xinput::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#members)\]
/// XINPUT_GAMEPAD_\*
///
/// Bitmask of the device digital buttons of an Xbox 360 style gamepad.
///
/// ### See Also
/// *   [Xbox 360 controller: Layout](https://en.wikipedia.org/wiki/Xbox_360_controller#Layout) (Wikipedia)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct Buttons(u16);

flags! { Buttons => u16; None, DPadUp, DPadDown, DPadLeft, DPadRight, Start, Guide, Back, LeftThumb, RightThumb, LeftShoulder, RightShoulder, A, B, X, Y }

impl Buttons {
    /// Returns `true` if any button from `buttons` is set in `self`
    pub fn any_held(&self, buttons: Buttons) -> bool { self.0 & buttons.0 != 0 }

    /// Returns `true` if **all** buttons from `buttons` are set in `self`
    pub fn all_held(&self, buttons: Buttons) -> bool { self.0 & buttons.0 == buttons.0 }
}

#[allow(non_upper_case_globals)] impl Buttons {
    /// No buttons are held.
    pub const None          : Buttons = Buttons(0);

    /// | Controller    | Where |
    /// | ------------- | ----- |
    /// | Xbox 360      | Directional pad on the bottom left of the face
    /// | Xbox One      | Directional pad on the bottom left of the face
    pub const DPadUp : Buttons = Buttons(XINPUT_GAMEPAD_DPAD_UP as _);

    /// | Controller    | Where |
    /// | ------------- | ------ |
    /// | Xbox 360      | Directional pad on the bottom left of the face
    /// | Xbox One      | Directional pad on the bottom left of the face
    pub const DPadDown : Buttons = Buttons(XINPUT_GAMEPAD_DPAD_DOWN as _);

    /// | Controller    | Where |
    /// | ------------- | ----- |
    /// | Xbox 360      | Directional pad on the bottom left of the face
    /// | Xbox One      | Directional pad on the bottom left of the face
    pub const DPadLeft : Buttons = Buttons(XINPUT_GAMEPAD_DPAD_LEFT as _);

    /// | Controller    | Where |
    /// | ------------- | ----- |
    /// | Xbox 360      | Directional pad on the bottom left of the face
    /// | Xbox One      | Directional pad on the bottom left of the face
    pub const DPadRight : Buttons = Buttons(XINPUT_GAMEPAD_DPAD_RIGHT as _);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Start `ᐅ` | Right button of the middle face cluster
    /// | Xbox One      | Menu `≡`  | Right button of the middle face cluster
    pub const Start : Buttons = Buttons(XINPUT_GAMEPAD_START as _);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Back `ᐊ` | Left button of the middle face cluster
    /// | Xbox One      | View `⧉` | Left button of the middle face cluster
    pub const Back : Buttons = Buttons(XINPUT_GAMEPAD_BACK as _);

    /// | Controller    | Legend    | When  |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | `L`       | Pressing the left thumbstick into the controller
    /// | Xbox One      | `L`       | Pressing the left thumbstick into the controller
    pub const LeftThumb : Buttons = Buttons(XINPUT_GAMEPAD_LEFT_THUMB as _);

    /// | Controller    | Legend    | When  |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | `R`       | Pressing the right thumbstick into the controller
    /// | Xbox One      | `R`       | Pressing the right thumbstick into the controller
    pub const RightThumb : Buttons = Buttons(XINPUT_GAMEPAD_RIGHT_THUMB as _);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | `LB`      | Top left on the controller
    /// | Xbox One      | `LB`      | Top left on the controller
    pub const LeftShoulder : Buttons = Buttons(XINPUT_GAMEPAD_LEFT_SHOULDER as _);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | `RB`      | Top right on the controller
    /// | Xbox One      | `RB`      | Top right on the controller
    pub const RightShoulder : Buttons = Buttons(XINPUT_GAMEPAD_RIGHT_SHOULDER as _);

    #[cfg(doc)]
    /// ❌ **NOTE** ❌ Not actually exposed by XInput as [`Buttons`], see [`Gamepad::left_trigger`](crate::xinput::Gamepad::left_trigger) instead.
    ///
    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | `LT`      | Under the top left of the controller
    /// | Xbox One      | `LT`      | Under the top left of the controller
    pub const LeftTrigger : () = ();

    #[cfg(doc)]
    /// ❌ **NOTE** ❌ Not actually available as part of [`Buttons`], see [`Gamepad::right_trigger`](crate::xinput::Gamepad::right_trigger) instead.
    ///
    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | `RT`      | Under the top right on the controller
    /// | Xbox One      | `RT`      | Under the top right on the controller
    pub const RightTrigger : () = ();

    /// ⚠️ **NOTE** ⚠️ This undocumented button is not returned by most APIs, being reserved for system software.
    ///
    /// Specifically, you must use the undocumented `XInputGetStateEx` function (same API as [`XInputGetState`](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate)?) to ever retrieve this value.
    /// Additionally, this is generally meant to be reserved by the system software.
    /// Windows, Steam, and other store apps all hook this globally regardless of what app/window has focus.
    ///
    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Guide     | Center of the middle face cluster.
    /// | Xbox One      | Xbox      | Center of the middle face cluster.
    ///
    /// #### Conflicting Software: Xbox Game Bar (Windows 8+)
    ///
    /// To disable:
    /// *   Launch `Xbox Game Bar` from the start menu
    /// *   Open `Settings` (Gear icon on the right of the top menu bar)
    /// *   Under `Shortcuts` > `Controller`,
    /// *   Unselect `Open Xbox Game Bar using (X) button on a controller`
    ///
    /// #### Conflicting Software: Steam
    ///
    /// To disable, close Steam, or:
    /// *   Open Steam's Settings
    ///     *   `Settings` after right clicking the Steam system tray icon, or
    ///     *   `Steam` > `Settings` in Steam's menu bar
    /// *   Under `Controller` > `Guide Button Chord Configuration`:
    ///     *   Choose `Browse Configs`
    ///     *   Select `Templates` > `Gamepad` (inert/do-nothing template)
    /// *   Under `Controller` > `General Controller Settings`:
    ///     *   Disable `Guide Button Focuses Steam`
    ///
    #[deprecated = "This undocumented button is not returned by most APIs, being reserved for system software.  See thindx's docs for details."]
    pub const Guide : Buttons = Buttons(1 << 10);

    /// Typically synonymous select/accept in menus on Xbox 360 style controllers/games/console.
    ///
    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Green A   | Bottom button of the right face cluster
    /// | Xbox One      | A         | Bottom button of the right face cluster
    pub const A : Buttons = Buttons(XINPUT_GAMEPAD_A as _);

    /// Typically synonymous back/cancel in menus on Xbox 360 style controllers/games/console.
    ///
    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Red B     | Right button of the right face cluster
    /// | Xbox One      | B         | Right button of the right face cluster
    pub const B : Buttons = Buttons(XINPUT_GAMEPAD_B as _);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Blue X    | Left button of the right face cluster
    /// | Xbox One      | X         | Left button of the right face cluster
    pub const X : Buttons = Buttons(XINPUT_GAMEPAD_X as _);

    /// | Controller    | Legend    | Where |
    /// | ------------- | --------- | ----- |
    /// | Xbox 360      | Yellow Y  | Top button of the right face cluster
    /// | Xbox One      | Y         | Top button of the right face cluster
    pub const Y : Buttons = Buttons(XINPUT_GAMEPAD_Y as _);
}



//#cpp2rust XINPUT_GAMEPAD_DPAD_UP          = xinput::Buttons::DPadUp
//#cpp2rust XINPUT_GAMEPAD_DPAD_DOWN        = xinput::Buttons::DPadDown
//#cpp2rust XINPUT_GAMEPAD_DPAD_LEFT        = xinput::Buttons::DPadLeft
//#cpp2rust XINPUT_GAMEPAD_DPAD_RIGHT       = xinput::Buttons::DPadRight
//#cpp2rust XINPUT_GAMEPAD_START            = xinput::Buttons::Start
//#cpp2rust XINPUT_GAMEPAD_BACK             = xinput::Buttons::Back
//#cpp2rust XINPUT_GAMEPAD_LEFT_THUMB       = xinput::Buttons::LeftThumb
//#cpp2rust XINPUT_GAMEPAD_RIGHT_THUMB      = xinput::Buttons::RightThumb
//#cpp2rust XINPUT_GAMEPAD_LEFT_SHOULDER    = xinput::Buttons::LeftShoulder
//#cpp2rust XINPUT_GAMEPAD_RIGHT_SHOULDER   = xinput::Buttons::RightShoulder
//#cpp2rust XINPUT_GAMEPAD_A                = xinput::Buttons::A
//#cpp2rust XINPUT_GAMEPAD_B                = xinput::Buttons::B
//#cpp2rust XINPUT_GAMEPAD_X                = xinput::Buttons::X
//#cpp2rust XINPUT_GAMEPAD_Y                = xinput::Buttons::Y
