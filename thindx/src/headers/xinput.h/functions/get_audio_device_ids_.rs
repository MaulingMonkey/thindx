use crate::THINERR;
use crate::xinput::*;

use std::ffi::OsString;
use std::os::windows::ffi::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids)\]
/// XInputGetAudioDeviceIds
///
/// Get XAudio2 / Windows Core Audio Device Names.
///
/// | XInput | State    |
/// | ------ | -------- |
/// | 1.4   | Available |
/// | 1.3   | N/A       |
/// | 9.1.0 | N/A       |
///
/// ### Errors
/// *   [ERROR::DEVICE_NOT_CONNECTED]
/// *   [THINERR::MISSING_DLL_EXPORT]
pub fn get_audio_device_ids(user_index: impl Into<User>) -> Result<AudioDeviceIds, MethodError> {
    // https://docs.microsoft.com/en-us/windows/win32/xinput/getting-started-with-xinput#getting-audio-device-identifiers

    #[allow(non_snake_case)] let XInputGetAudioDeviceIds = Imports::get().XInputGetAudioDeviceIds.ok_or(MethodError::new("XInputGetAudioDeviceIds", THINERR::MISSING_DLL_EXPORT))?;

    let mut render_id  = [0u16; 4096];
    let mut capture_id = [0u16; 4096];
    let mut render_len  = 4096;
    let mut capture_len = 4096;

    let code = unsafe { XInputGetAudioDeviceIds(user_index.into().into(), render_id.as_mut_ptr(), &mut render_len, capture_id.as_mut_ptr(), &mut capture_len) };
    // a dynamic alloc fallback might be appropriate...? what error is returned? experiment, as it's not documented?
    check_error_success("XInputGetAudioDeviceIds", code)?;
    Ok(AudioDeviceIds {
        render_device_id:   if render_len  == 0 { None } else { Some(OsString::from_wide(&render_id [..render_len  as usize])) },
        capture_device_id:  if capture_len == 0 { None } else { Some(OsString::from_wide(&capture_id[..capture_len as usize])) },
    })
}
