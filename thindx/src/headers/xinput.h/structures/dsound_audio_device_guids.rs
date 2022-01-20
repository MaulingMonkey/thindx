use crate::*;
use bytemuck::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/nf-xinput-xinputgetdsoundaudiodeviceguids)\]
/// DirectSound audio device [Guid]s retrieved with [get_dsound_audio_device_guids](crate::xinput::get_dsound_audio_device_guids)
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct DSoundAudioDeviceGuids {
    /// GUID of the headset sound rendering device.
    pub dsound_render_guid:     Guid,

    /// GUID of the headset sound capture device.
    pub dsound_capture_guid:    Guid,
}

#[test] fn test_traits_for_coverage() {
    let _audio = DSoundAudioDeviceGuids::default();
    let _audio = DSoundAudioDeviceGuids::zeroed();
    let _audio = _audio.clone();
    dbg!(_audio);
}
