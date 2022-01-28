#[allow(unused_imports)] use crate::*;

type D3DSGR = u32;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setgammaramp)\]
/// D3DSGR_*
///
/// Indicates whether correction should be applied.
/// Gamma correction results in a more consistent display, but can incur processing overhead and should not be used frequently.
/// Short-duration effects, such as flashing the whole screen red, should not be calibrated, but long-duration gamma changes should be calibrated.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SGR(D3DSGR);

enumish! { SGR => D3DSGR; Calibrate, NoCalibration }

#[allow(non_upper_case_globals)] impl SGR { // These are enum-like
    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\d3d9helper.h

    /// If a gamma calibrator is installed, the ramp will be modified before being sent to the device to account for the system and monitor response curves.
    /// If a calibrator is not installed, the ramp will be passed directly to the device.
    pub const Calibrate         : SGR = SGR(0);

    /// No gamma correction is applied.
    /// The supplied gamma table is transferred directly to the device.
    pub const NoCalibration     : SGR = SGR(1);
}

impl Default for SGR {
    fn default() -> Self { SGR::Calibrate } // 0
}

//#cpp2rust D3DSGR_CALIBRATE        = d3d::SGR::Calibrate
//#cpp2rust D3DSGR_NO_CALIBRATION   = d3d::SGR::NoCalibration
