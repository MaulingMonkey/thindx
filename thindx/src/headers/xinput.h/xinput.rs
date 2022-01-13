#![warn(clippy::undocumented_unsafe_blocks)]

use crate::{MethodError, ErrorKind, errors::ERROR};

mods! {
    inl mod dll {
        inl mod imports;
    }

    inl mod enumerations {
        inl mod battery_devtype;
        inl mod battery_level;
        inl mod battery_type;
        inl mod devsubtype;
        inl mod devtype;
        inl mod user;
        inl mod vk;
    }

    inl mod flags {
        inl mod buttons;
        inl mod caps;
        inl mod flag;
        pub(crate) mod keystroke;
    }

    inl mod functions {
        inl mod enable_;
        inl mod get_audio_device_ids_;
        inl mod get_battery_information_;
        inl mod get_capabilities_;
        inl mod get_dsound_audio_device_guids_;
        inl mod get_keystroke_;
        inl mod get_state_;
        inl mod get_state_ex_;
        inl mod set_state_;
    }

    inl mod structures {
        inl mod audio_device_ids;
        inl mod battery_information;
        inl mod capabilities;
        inl mod dsound_audio_device_guids;
        inl mod gamepad;
        inl mod keystroke;
        inl mod state;
        inl mod vibration;
    }
}

pub use flags::keystroke::Keystroke as KeystrokeFlags;

pub(crate) fn check_error_success(method: &'static str, err: u32) -> Result<(), MethodError> {
    if err == ERROR::SUCCESS.0 as _ {
        Ok(())
    } else {
        Err(MethodError::new(method, ErrorKind(err as _)))
    }
}
