//! Test basic [xinput] APIs

use std::io::*;
use std::ptr::null_mut;
use std::time::*;

use thindx::xinput::{self, *};
use winapi::um::objbase::CoInitialize;

fn main() {
    unsafe { CoInitialize(null_mut()) };

    'vks: for vk in [
        VK::PadA, VK::PadB, VK::PadX, VK::PadY, VK::PadRShoulder, VK::PadLShoulder,
        VK::PadRTrigger, VK::PadLTrigger, VK::PadDPadUp, VK::PadDPadDown, VK::PadDPadLeft,
        VK::PadDPadRight, VK::PadStart, VK::PadBack, VK::PadLThumbPress, VK::PadRThumbPress,
        VK::PadLThumbUp, VK::PadLThumbDown, VK::PadLThumbRight, VK::PadLThumbLeft,
        VK::PadLThumbUpLeft, VK::PadLThumbUpRight, VK::PadLThumbDownRight,
        VK::PadLThumbDownLeft, VK::PadRThumbUp, VK::PadRThumbDown, VK::PadRThumbRight,
        VK::PadRThumbLeft, VK::PadRThumbUpLeft, VK::PadRThumbUpRight, VK::PadRThumbDownRight,
        VK::PadRThumbDownLeft,
    ].iter().copied() {
        print!("    press {:?}... ", vk);
        let _ = stdout().flush();

        let timeout = Instant::now() + Duration::from_secs(10);
        while Instant::now() < timeout {
            match xinput::get_keystroke(User::Any, ()) {
                Ok(Some(e)) if vk == e.virtual_key => {
                    println!("✔️ pressed");
                    continue 'vks
                },
                Ok(Some(_other)) => {},
                Ok(None) => std::thread::yield_now(),
                Err(e) => {
                    println!("❌ {:?}", e);
                    continue 'vks
                },
            }
        }
        println!("⚠️ timeout");
    }

    'buttons: for buttons in [
        Buttons::None,
        Buttons::A, Buttons::B, Buttons::A | Buttons::B,
        Buttons::X, Buttons::Y, Buttons::X | Buttons::Y,
        Buttons::RightShoulder, Buttons::LeftShoulder,
        Buttons::DPadUp, Buttons::DPadDown, Buttons::DPadLeft, Buttons::DPadRight,
        Buttons::Start,
        // #[allow(deprecated)] Buttons::Guide, // TODO: GetModuleHandleExW use seems busted
        Buttons::Back,
        Buttons::RightThumb, Buttons::LeftThumb,
    ].iter().copied() {
        print!("    press {:?}... ", buttons);
        let _ = stdout().flush();

        let timeout = Instant::now() + Duration::from_secs(10);
        while Instant::now() < timeout {
            for user in xinput::User::iter_valid() {
                #[allow(deprecated)] match xinput::get_state_ex(user) {
                    Ok(e) if buttons == e.buttons => {
                        println!("✔️ pressed");
                        continue 'buttons
                    },
                    Ok(_other) => {},
                    Err(_err) => {},
                }
            }
        }
        println!("⚠️ timeout");
    }
}
