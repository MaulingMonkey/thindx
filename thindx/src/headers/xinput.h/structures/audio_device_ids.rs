use std::ffi::OsString;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetaudiodeviceids)\]
/// Audio device ids retrieved with [get_audio_device_ids](crate::xinput::get_audio_device_ids)
#[derive(Clone, Debug)]
#[derive(Default)]
pub struct AudioDeviceIds {
    /// Windows Core Audio device ID string for render (speakers).
    pub render_device_id:   Option<OsString>,

    /// Windows Core Audio device ID string for capture (microphone).
    pub capture_device_id:  Option<OsString>,
}

#[test] fn test_traits_for_coverage() {
    let _audio = AudioDeviceIds::default();
    let _audio = _audio.clone();
    dbg!(_audio);
}
