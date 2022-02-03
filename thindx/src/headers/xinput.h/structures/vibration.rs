use bytemuck::{Pod, Zeroable};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_vibration)\]
/// XINPUT_VIBRATION
///
/// Specifies motor speed levels for the vibration function of a controller.
#[derive(Clone, Copy, Debug)]
#[derive(Default, Pod, Zeroable)]
#[repr(C)] pub struct Vibration {
    /// Speed of the left, lower-frequency (heavier?) rumble motor.
    ///
    /// | Value | Rumble    |
    /// | ----- | --------- |
    /// | 0     | None      |
    /// | 65535 | 100%      |
    pub left_motor_speed:   u16,

    /// Speed of the right, higher-frequency (lighter?) rumble motor.
    ///
    /// | Value | Rumble    |
    /// | ----- | --------- |
    /// | 0     | None      |
    /// | 65535 | 100%      |
    pub right_motor_speed:  u16,
}

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Vibration => winapi::um::xinput::XINPUT_VIBRATION {
        left_motor_speed    => wLeftMotorSpeed,
        right_motor_speed   => wRightMotorSpeed,
    }
}

#[test] fn test_traits_for_coverage() {
    let _vibration = Vibration::default();
    let _vibration = Vibration::zeroed();
    let _vibration = _vibration.clone();
    dbg!(_vibration);
}

//#cpp2rust XINPUT_VIBRATION            = xinput::Vibration
