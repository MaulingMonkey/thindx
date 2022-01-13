use crate::xinput::*;
use bytemuck::{Pod, Zeroable};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_state)\]
/// XINPUT_STATE
///
/// Packet number + [Gamepad] state.
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct State {
    /// State packet number.
    /// The packet number indicates whether there have been any changes in the state of the controller.
    /// If `packet_number` is the same in sequentially returned [`State`] structures, the controller state has not changed.
    pub packet_number: u32,

    /// The gamepad state.
    pub gamepad:        Gamepad,
}

// TODO: deadzone fns?
// TODO: pairing fns?

impl std::ops::Deref for State {
    type Target = Gamepad;
    fn deref(&self) -> &Gamepad { &self.gamepad }
}

impl std::ops::DerefMut for State {
    fn deref_mut(&mut self) -> &mut Gamepad { &mut self.gamepad }
}

test_layout! {
    State => winapi::um::xinput::XINPUT_STATE {
        packet_number   => dwPacketNumber,
        gamepad         => Gamepad,
    }
}
