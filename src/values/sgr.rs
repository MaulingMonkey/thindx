#[allow(unused_imports)] use crate::*;

type D3DSGR = u32;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setgammaramp)\]
/// D3DSGR_*
///
/// Indicates whether correction should be applied.
/// Gamma correction results in a more consistent display, but can incur processing overhead and should not be used frequently.
/// Short-duration effects, such as flashing the whole screen red, should not be calibrated, but long-duration gamma changes should be calibrated.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SGR(D3DSGR);

impl SGR {
    /// Convert a raw [D3DSGR] value into a [SGR].  This is *probably* safe... probably...
    ///
    /// [D3DSGR]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsgr
    pub const fn from_unchecked(sgr: D3DSGR) -> Self { Self(sgr) }

    /// Convert a [SGR] into a raw [D3DSGR].
    ///
    /// [D3DSGR]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dsgr
    pub const fn into(self) -> D3DSGR { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl SGR {
    // C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\d3d9helper.h

    /// If a gamma calibrator is installed, the ramp will be modified before being sent to the device to account for the system and monitor response curves.
    /// If a calibrator is not installed, the ramp will be passed directly to the device.
    pub const Calibrate         : SGR = SGR(0);

    /// No gamma correction is applied.
    /// The supplied gamma table is transferred directly to the device.
    pub const NoCalibration     : SGR = SGR(1);
}

impl Debug for SGR {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            SGR::Calibrate      => write!(f, "SGR::Calibrate"),
            SGR::NoCalibration  => write!(f, "SGR::NoCalibration"),
            other               => write!(f, "SGR({})", other.0 as u32),
        }
    }
}

impl From<SGR> for D3DSGR {
    fn from(value: SGR) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DSGR> for SGR {
    fn from(value: D3DSGR) -> Self { Self(value) }
}
