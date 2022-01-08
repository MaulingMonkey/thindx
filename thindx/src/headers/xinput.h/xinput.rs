//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/xinput/xinput-game-controller-apis-portal)\]
//! APIs for Xbox 360 style controllers
//!
//! ### Enumerations
//!
//! | C++                       | Rust                  |
//! | ------------------------- | --------------------- |
//! | [`BATTERY_DEVTYPE_*`]     | [`BatteryDevType`]::\*
//! | [`BATTERY_LEVEL_*`]       | [`BatteryLevel`]::\*
//! | [`BATTERY_TYPE_*`]        | [`BatteryType`]::\*
//! | [`XINPUT_DEVSUBTYPE_*`]   | [`DevSubType`]::\*
//! | [`XINPUT_DEVTYPE_*`]      | [`DevType`]::\*
//! | `XUSER_*`                 | [`User`]::\*
//! | [`VK_*`]                  | [`VK`]::\*
//!
//! [`BATTERY_DEVTYPE_*`]:      https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation
//! [`BATTERY_LEVEL_*`]:        https://docs.microsoft.com/en-us/windows/win32/api/XInput/ns-xinput-xinput_battery_information
//! [`BATTERY_TYPE_*`]:         https://docs.microsoft.com/en-us/windows/win32/api/XInput/ns-xinput-xinput_battery_information
//! [`XINPUT_DEVSUBTYPE_*`]:    https://docs.microsoft.com/en-us/windows/win32/xinput/xinput-and-controller-subtypes
//! [`XINPUT_DEVTYPE_*`]:       https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities
//! [`VK_*`]:                   https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke#remarks
//!
//!
//!
//! ### Flags
//!
//! | C++                       | Rust                  |
//! | ------------------------- | --------------------- |
//! | [`XINPUT_GAMEPAD_*`]      | [`Buttons`]::\*
//! | [`XINPUT_CAPS_*`]         | [`Caps`]::\*
//! | [`XINPUT_FLAG_*`]         | [`Flag`]::\*
//!
//! [`XINPUT_GAMEPAD_*`]:       https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad#members
//! [`XINPUT_CAPS_*`]:          https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities#members
//! [`XINPUT_FLAG_*`]:          https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities#parameters
//!
//!
//!
//! ### Functions
//!
//! | C++                                   | Rust                  |
//! | ------------------------------------- | --------------------- |
//! | [`XInputEnable`]                      | [`enable`]
//! | [`XInputGetAudioDeviceIds`]           | [`get_audio_device_ids`]
//! | [`XInputGetBatteryInformation`]       | [`get_battery_information`]
//! | [`XInputGetCapabilities`]             | [`get_capabilities`]
//! | [`XInputGetDSoundAudioDeviceGuids`]   | [`get_dsound_audio_device_guids`]
//! | [`XInputGetKeystroke`]                | [`get_keystroke`]
//! | [`XInputGetState`]                    | [`get_state`]
//! | `XInputGetStateEx`                    | [`get_state_ex`]
//! | [`XInputSetState`]                    | [`set_state`]
//!
//! [`XInputEnable`]:                       https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputenable
//! [`XInputGetAudioDeviceIds`]:            https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids
//! [`XInputGetBatteryInformation`]:        https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetbatteryinformation
//! [`XInputGetCapabilities`]:              https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetcapabilities
//! [`XInputGetDSoundAudioDeviceGuids`]:    https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids
//! [`XInputGetKeystroke`]:                 https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetkeystroke
//! [`XInputGetState`]:                     https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetstate
//! [`XInputSetState`]:                     https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputsetstate
//!
//!
//!
//! ### Structures
//!
//! | C++                               | Rust                  |
//! | --------------------------------- | --------------------- |
//! | [`XINPUT_BATTERY_INFORMATION`]    | [`BatteryInformation`]
//! | [`XINPUT_CAPABILITIES`]           | [`Capabilities`]
//! | [`XINPUT_GAMEPAD`]                | [`Gamepad`]
//! | [`XINPUT_KEYSTROKE`]              | [`Keystroke`]
//! | [`XINPUT_STATE`]                  | [`State`]
//! | [`XINPUT_VIBRATION`]              | [`Vibration`]
//! | <code>std::array&lt;[std::wstring], 2&gt;</code>  | [`AudioDeviceIds`]
//! | <code>std::array&lt;[GUID], 2&gt;</code>          | [`DSoundAudioDeviceGuids`]
//!
//! [`XINPUT_BATTERY_INFORMATION`]:     https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_battery_information
//! [`XINPUT_CAPABILITIES`]:            https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_capabilities
//! [`XINPUT_GAMEPAD`]:                 https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad
//! [`XINPUT_KEYSTROKE`]:               https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_keystroke
//! [`XINPUT_STATE`]:                   https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_state
//! [`XINPUT_VIBRATION`]:               https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_vibration
//! [std::wstring]:                     https://docs.microsoft.com/en-us/cpp/standard-library/string-typedefs?view=msvc-170#wstring
//! [GUID]:                             https://docs.microsoft.com/en-us/windows/win32/api/guiddef/ns-guiddef-guid
//!
//!
//! # Alternatives
//!
//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/previous-versions/windows/desktop/ee416842(v=vs.85))\] **DirectInput**
//! *   ✔️ Supports joysticks with many more buttons and axises than XInput.
//! *   ✔️ Leverages Windows's built in support for configuring idle positions, deadzones.
//! *   ⚠️ Older, "deprecated" in favor of XInput.
//! *   ❌ Xbox 360 controllers map both triggers to a single axis in DirectInput.
//! *   [Comparison of XInput and DirectInput features](https://docs.microsoft.com/en-us/windows/win32/xinput/xinput-and-directinput)
//!     discusses how to use a hybrid approach of XInput for 360 controllers and DirectInput for non-XInput devices
//!
//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/uwp/gaming/input-for-games)\] **UWP**
//! *   ✔️ Supports Xbox One trigger rumble (XInput only supports base controller rumble)
//! *   ⚠️ Can't recieve input through UWP when the app/window is not active (useful for dev cruft.)
//! *   ⚠️ No Windows 7 support?

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

fn check_error_success(method: &'static str, err: u32) -> Result<(), MethodError> {
    if err == ERROR::SUCCESS.0 as _ {
        Ok(())
    } else {
        Err(MethodError::new(method, ErrorKind(err as _)))
    }
}
