use crate::*;
use crate::xinput::*;

use std::ffi::OsString;
use std::os::windows::ffi::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids)\]
/// XInputGetAudioDeviceIds
///
/// Get XAudio2 / Windows Core Audio Device Names.
///
/// **NOTE:** This tends to succeed, even when no gamepad is connected, with empty/None paths.
///
// | XInput | State    |
// | ------ | -------- |
// | 1.4   | Available |
// | 1.3   | N/A       |
// | 9.1.0 | N/A       |
///
/// ### Errors
/// *   [ERROR::BAD_ARGUMENTS]          - Invalid [`User`] or [`User::Any`]
/// *   [ERROR::DEVICE_NOT_CONNECTED]   - **Unreliably.**
/// *   [THINERR::MISSING_DLL_EXPORT]   - XAudio2 / Windows Core Audio Device Names unavailable: XInput 1.3 or earlier
///
/// | System            | Windows `ver`     | Windows SKU           | Behavior |
/// | ----------------- | ----------------- | --------------------- | -------- |
/// | Github Actions    | 10.0.17763.2366   | Windows 2019 Server   | [ERROR::DEVICE_NOT_CONNECTED] observed.
/// | "SACRILEGE"       | 10.0.19041.1415   | Windows 10 Pro        | Succeeds when called on missing gamepads
///
/// ### See Also
/// *   [Getting Audio Device Identifiers](https://docs.microsoft.com/en-us/windows/win32/xinput/getting-started-with-xinput#getting-audio-device-identifiers)
pub fn get_audio_device_ids(user_index: impl Into<User>) -> Result<AudioDeviceIds, MethodError> {
    #[allow(non_snake_case)] let XInputGetAudioDeviceIds = Imports::get().XInputGetAudioDeviceIds.ok_or(MethodError::new("XInputGetAudioDeviceIds", THINERR::MISSING_DLL_EXPORT))?;

    let mut render_id  = [0u16; 4096];
    let mut capture_id = [0u16; 4096];
    let mut render_len  = 4096;
    let mut capture_len = 4096;

    let code = unsafe { XInputGetAudioDeviceIds(user_index.into().into(), render_id.as_mut_ptr(), &mut render_len, capture_id.as_mut_ptr(), &mut capture_len) };
    // a dynamic alloc fallback might be appropriate...? what error is returned? experiment, as it's not documented?
    check_error_success("XInputGetAudioDeviceIds", code)?;
    let render_device_id    = OsString::from_wide(render_id [..render_len  as usize].split(|c| *c==0).next().unwrap_or(&[]));
    let capture_device_id   = OsString::from_wide(capture_id[..capture_len as usize].split(|c| *c==0).next().unwrap_or(&[]));
    Ok(AudioDeviceIds {
        render_device_id:   if render_device_id .is_empty() { None } else { Some(render_device_id ) },
        capture_device_id:  if capture_device_id.is_empty() { None } else { Some(capture_device_id) },
    })
}

#[test] fn test_returns() {
    if get_audio_device_ids(User::Zero) == THINERR::MISSING_DLL_EXPORT { return }

    // May or may not succeed, even if gamepad not connected
    if let Err(err) = get_audio_device_ids(User::Zero ) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_audio_device_ids(User::One  ) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_audio_device_ids(User::Two  ) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }
    if let Err(err) = get_audio_device_ids(User::Three) { assert_eq!(ERROR::DEVICE_NOT_CONNECTED, err.kind()); }

    // Invalid User s
    assert_eq!(ERROR::BAD_ARGUMENTS, get_audio_device_ids(User::from_unchecked(4))  );
    assert_eq!(ERROR::BAD_ARGUMENTS, get_audio_device_ids(User::Any)                );
}
