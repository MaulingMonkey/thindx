#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dmultisample-type)\]
/// D3DMULTISAMPLE_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct MultiSampleType(D3DMULTISAMPLE_TYPE);
pub use MultiSampleType as MultiSample;

impl MultiSample {
    /// Convert a number of samples between 1 and 16 into a [MultiSample]
    pub const fn from_samples(samples: u8) -> Option<Self> {
        match samples {
            0 | 1               => Some(MultiSample::None),
            samples @ 2 ..= 16  => Some(MultiSample(samples as u32)),
            _                   => None,
        }
    }
}

enumish! { MultiSample => D3DMULTISAMPLE_TYPE; default: None == 0; None, NonMaskable, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16 }

#[allow(non_upper_case_globals)] impl MultiSample { // These are enum-like
    /// No level of full-scene multisampling is available.
    pub const None          : MultiSample = MultiSample(D3DMULTISAMPLE_NONE); // 0

    /// Enables the multisample quality value.
    pub const NonMaskable   : MultiSample = MultiSample(D3DMULTISAMPLE_NONMASKABLE); // 1

    pub const X2            : MultiSample = MultiSample(D3DMULTISAMPLE_2_SAMPLES); // 2
    pub const X3            : MultiSample = MultiSample(D3DMULTISAMPLE_3_SAMPLES); // 3
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

//#cpp2rust D3DMULTISAMPLE_TYPE         = d3d::MultiSampleType

//#cpp2rust D3DMULTISAMPLE_NONE         = d3d::MultiSample::None
//#cpp2rust D3DMULTISAMPLE_NONMASKABLE  = d3d::MultiSample::NonMaskable
//#cpp2rust D3DMULTISAMPLE_2_SAMPLES    = d3d::MultiSample::X2
//#cpp2rust D3DMULTISAMPLE_3_SAMPLES    = d3d::MultiSample::X3
//#cpp2rust D3DMULTISAMPLE_4_SAMPLES    = d3d::MultiSample::X4
//#cpp2rust D3DMULTISAMPLE_5_SAMPLES    = d3d::MultiSample::X5
//#cpp2rust D3DMULTISAMPLE_6_SAMPLES    = d3d::MultiSample::X6
//#cpp2rust D3DMULTISAMPLE_7_SAMPLES    = d3d::MultiSample::X7
//#cpp2rust D3DMULTISAMPLE_8_SAMPLES    = d3d::MultiSample::X8
//#cpp2rust D3DMULTISAMPLE_9_SAMPLES    = d3d::MultiSample::X9
//#cpp2rust D3DMULTISAMPLE_10_SAMPLES   = d3d::MultiSample::X10
//#cpp2rust D3DMULTISAMPLE_11_SAMPLES   = d3d::MultiSample::X11
//#cpp2rust D3DMULTISAMPLE_12_SAMPLES   = d3d::MultiSample::X12
//#cpp2rust D3DMULTISAMPLE_13_SAMPLES   = d3d::MultiSample::X13
//#cpp2rust D3DMULTISAMPLE_14_SAMPLES   = d3d::MultiSample::X14
//#cpp2rust D3DMULTISAMPLE_15_SAMPLES   = d3d::MultiSample::X15
//#cpp2rust D3DMULTISAMPLE_16_SAMPLES   = d3d::MultiSample::X16
