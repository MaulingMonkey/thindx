#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmultisample-type)\]
/// D3DMULTISAMPLE_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MultiSample(D3DMULTISAMPLE_TYPE);

impl MultiSample {
    /// Convert a raw [D3DMULTISAMPLE_TYPE] value into a [MultiSample].  This is *probably* safe... probably....
    ///
    /// [D3DMULTISAMPLE_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmultisample-type
    pub const fn from_unchecked(multisampletype: D3DMULTISAMPLE_TYPE) -> Self { Self(multisampletype) }

    /// Convert a number of samples between 1 and 16 into a [MultiSample]
    pub const fn from_samples(samples: u8) -> Option<Self> {
        match samples {
            0 | 1               => Some(MultiSample::None),
            samples @ 2 ..= 16  => Some(MultiSample(samples as u32)),
            _                   => None,
        }
    }

    /// Convert a [MultiSample] into a raw [D3DMULTISAMPLE_TYPE].
    ///
    /// [D3DMULTISAMPLE_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmultisample-type
    pub const fn into(self) -> D3DMULTISAMPLE_TYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl MultiSample {
    /// No level of full-scene multisampling is available.
    pub const None          : MultiSample = MultiSample(D3DMULTISAMPLE_NONE);

    /// Enables the multisample quality value.
    pub const NonMaskable   : MultiSample = MultiSample(D3DMULTISAMPLE_NONMASKABLE);

    pub const X2            : MultiSample = MultiSample(D3DMULTISAMPLE_2_SAMPLES);
    pub const X3            : MultiSample = MultiSample(D3DMULTISAMPLE_3_SAMPLES);
    pub const X4            : MultiSample = MultiSample(D3DMULTISAMPLE_4_SAMPLES);
    pub const X5            : MultiSample = MultiSample(D3DMULTISAMPLE_5_SAMPLES);
    pub const X6            : MultiSample = MultiSample(D3DMULTISAMPLE_6_SAMPLES);
    pub const X7            : MultiSample = MultiSample(D3DMULTISAMPLE_7_SAMPLES);
    pub const X8            : MultiSample = MultiSample(D3DMULTISAMPLE_8_SAMPLES);
    pub const X9            : MultiSample = MultiSample(D3DMULTISAMPLE_9_SAMPLES);
    pub const X10           : MultiSample = MultiSample(D3DMULTISAMPLE_10_SAMPLES);
    pub const X11           : MultiSample = MultiSample(D3DMULTISAMPLE_11_SAMPLES);
    pub const X12           : MultiSample = MultiSample(D3DMULTISAMPLE_12_SAMPLES);
    pub const X13           : MultiSample = MultiSample(D3DMULTISAMPLE_13_SAMPLES);
    pub const X14           : MultiSample = MultiSample(D3DMULTISAMPLE_14_SAMPLES);
    pub const X15           : MultiSample = MultiSample(D3DMULTISAMPLE_15_SAMPLES);
    pub const X16           : MultiSample = MultiSample(D3DMULTISAMPLE_16_SAMPLES);
}

impl Default for MultiSample {
    fn default() -> Self { MultiSample::None }
}

impl Debug for MultiSample {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            MultiSample::None           => write!(f, "MultiSample::None"),
            MultiSample::NonMaskable    => write!(f, "MultiSample::NonMaskable"),
            other                       => write!(f, "MultiSample({})", other.0),
        }
    }
}

impl From<MultiSample> for D3DMULTISAMPLE_TYPE {
    fn from(value: MultiSample) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DMULTISAMPLE_TYPE> for MultiSample {
    fn from(value: D3DMULTISAMPLE_TYPE) -> Self { Self(value) }
}
