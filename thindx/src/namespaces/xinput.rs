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
//! | [`XInputGetDSoundAudioDeviceGuids`] ❌ | [`get_dsound_audio_device_guids`]
//! | [`XInputGetKeystroke`]                | [`get_keystroke`]
//! | [`XInputGetState`]                    | [`get_state`]
//! | `XInputGetStateEx` ⚠️ 1.3             | [`get_state_ex`]
//! | [`XInputSetState`]                    | [`set_state`]
//! | `XInputWaitForGuideButton`    ⚠️ 1.3  | <span style="opacity: 25%">TODO?</span>
//! | `XInputCancelGuideButtonWait` ⚠️ 1.3  | <span style="opacity: 25%">TODO?</span>
//! | `XInputPowerOffController`    ⚠️ 1.3  | <span style="opacity: 25%">TODO?</span>
//! | `XInputGetBaseBusInformation` ⚠️ 1.4  | <span style="opacity: 25%">TODO?</span>
//! | `XInputGetCapabilitiesEx`     ⚠️ 1.4  | <span style="opacity: 25%">TODO?</span>
//!
//! | Legend | Desc |
//! | ------ | ---- |
//! | ❌     | Deprecated
//! | ⚠️ 1.3 | Undocumented XInput 1.3+ function exported by ordinal only
//! | ⚠️ 1.4 | Undocumented XInput 1.4+ function exported by ordinal only
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

pub use crate::xinput_h::*;
