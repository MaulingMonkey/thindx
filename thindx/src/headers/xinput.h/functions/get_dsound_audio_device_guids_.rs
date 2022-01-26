use crate::*;
use crate::xinput::*;

use bytemuck::Zeroable;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids)\]
/// XInputGetDSoundAudioDeviceGuids
///
/// Get DirectSound Audio Device GUIDs (N/A for Windows Store apps, isn't supported by Windows 8.)
///
/// | XInput | State    |
/// | ------ | -------- |
/// | 1.4   | N/A       |
/// | 1.3   | Available |
/// | 9.1.0 | Available |
///
/// ### Errors
/// *   [THINERR::MISSING_DLL_EXPORT]   - DirectSound GUIDs unavailable: XInput 1.4 or later
/// *   [ERROR::BAD_ARGUMENTS]?         - [`User`] out of bounds?
/// *   [ERROR::DEVICE_NOT_CONNECTED]?  - [`User`] in bounds, but without a gamepad?
#[deprecated = "Deprecated in favor of xinput::get_audio_device_ids.  Unavailable for Windows Store apps, may fail on Windows 8."]
pub fn get_dsound_audio_device_guids(user_index: impl Into<u32>) -> Result<DSoundAudioDeviceGuids, MethodError> {
    #[allow(non_snake_case)] let XInputGetDSoundAudioDeviceGuids = Imports::get().XInputGetDSoundAudioDeviceGuids.ok_or(MethodError("XInputGetDSoundAudioDeviceGuids", THINERR::MISSING_DLL_EXPORT))?;

    let mut guids = DSoundAudioDeviceGuids::zeroed();
    // SAFETY: ❌ Untested (need a system actually defining XInputGetDSoundAudioDeviceGuids)
    //  * fuzzed        in `tests/fuzz-xinput.rs`
    //  * `user_index`  is well tested from 0 ..= 255 (but retest if the type of `user_index` expands to allow `u32`!)
    //  * `*_guid`      are nice and fixed-size etc.
    let code = unsafe { XInputGetDSoundAudioDeviceGuids(user_index.into().into(), &mut guids.dsound_render_guid as *mut _ as *mut _, &mut guids.dsound_capture_guid as *mut _ as *mut _) };
    check_error_success("XInputGetDSoundAudioDeviceGuids", code)?;
    Ok(guids)
}

#[test] fn test() {
    #[allow(deprecated)] let r = get_dsound_audio_device_guids(0u32);
    if r != THINERR::MISSING_DLL_EXPORT {
        mmrbi::warning!(at: file!(), line: line!() as usize,
            "xinput::get_dsound_audio_device_guids(0) returned {:?}: may be implemented on this platform: add test coverage!",
            r
        );
    }
}

//#cpp2rust XInputGetDSoundAudioDeviceGuids     = xinput::get_dsound_audio_device_guids
