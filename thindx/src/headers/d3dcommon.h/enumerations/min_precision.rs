#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_min_precision)\]
/// D3D_MIN_PRECISION
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MinPrecision(D3D_MIN_PRECISION);

enumish! { MinPrecision => D3D_MIN_PRECISION; Default, Float16, Float2_8, Reserved, SInt16, UInt16, Any16, Any10 }

#[allow(non_upper_case_globals)] impl MinPrecision { // These are enum-like
    /// Default minimum precision, which is 32-bit precision.
    pub const Default     : MinPrecision = MinPrecision(D3D_MIN_PRECISION_DEFAULT);

    /// Minimum precision is min16float, which is 16-bit floating point.
    pub const Float16     : MinPrecision = MinPrecision(D3D_MIN_PRECISION_FLOAT_16);

    /// Minimum precision is min10float, which is 10-bit floating point.
    pub const Float2_8    : MinPrecision = MinPrecision(D3D_MIN_PRECISION_FLOAT_2_8);

    /// Reserved
    pub const Reserved    : MinPrecision = MinPrecision(D3D_MIN_PRECISION_RESERVED);

    /// Minimum precision is min16int, which is 16-bit signed integer.
    pub const SInt16      : MinPrecision = MinPrecision(D3D_MIN_PRECISION_SINT_16);

    /// Minimum precision is min16uint, which is 16-bit unsigned integer.
    pub const UInt16      : MinPrecision = MinPrecision(D3D_MIN_PRECISION_UINT_16);

    /// Minimum precision is any 16-bit value.
    pub const Any16       : MinPrecision = MinPrecision(D3D_MIN_PRECISION_ANY_16);

    /// Minimum precision is any 10-bit value.
    pub const Any10       : MinPrecision = MinPrecision(D3D_MIN_PRECISION_ANY_10);
}

#[doc(hidden)] impl MinPrecision { // Ctrl+C Ctrl+V support
    /// Default minimum precision, which is 32-bit precision.
    pub const DEFAULT     : MinPrecision = MinPrecision(D3D_MIN_PRECISION_DEFAULT);

    /// Minimum precision is min16float, which is 16-bit floating point.
    pub const FLOAT_16    : MinPrecision = MinPrecision(D3D_MIN_PRECISION_FLOAT_16);

    /// Minimum precision is min10float, which is 10-bit floating point.
    pub const FLOAT_2_8   : MinPrecision = MinPrecision(D3D_MIN_PRECISION_FLOAT_2_8);

    /// Reserved
    pub const RESERVED    : MinPrecision = MinPrecision(D3D_MIN_PRECISION_RESERVED);

    /// Minimum precision is min16int, which is 16-bit signed integer.
    pub const SINT_16     : MinPrecision = MinPrecision(D3D_MIN_PRECISION_SINT_16);

    /// Minimum precision is min16uint, which is 16-bit unsigned integer.
    pub const UINT_16     : MinPrecision = MinPrecision(D3D_MIN_PRECISION_UINT_16);

    /// Minimum precision is any 16-bit value.
    pub const ANY_16      : MinPrecision = MinPrecision(D3D_MIN_PRECISION_ANY_16);

    /// Minimum precision is any 10-bit value.
    pub const ANY_10      : MinPrecision = MinPrecision(D3D_MIN_PRECISION_ANY_10);
}

impl Default for MinPrecision {
    fn default() -> Self { MinPrecision(0) }
}

//#cpp2rust D3D_MIN_PRECISION           = d3d::MinPrecision
//#cpp2rust D3D_MIN_PRECISION_DEFAULT   = d3d::MinPrecision::Default
//#cpp2rust D3D_MIN_PRECISION_FLOAT_16  = d3d::MinPrecision::Float16
//#cpp2rust D3D_MIN_PRECISION_FLOAT_2_8 = d3d::MinPrecision::Float2_8
//#cpp2rust D3D_MIN_PRECISION_RESERVED  = d3d::MinPrecision::Reserved
//#cpp2rust D3D_MIN_PRECISION_SINT_16   = d3d::MinPrecision::SInt16
//#cpp2rust D3D_MIN_PRECISION_UINT_16   = d3d::MinPrecision::UInt16
//#cpp2rust D3D_MIN_PRECISION_ANY_16    = d3d::MinPrecision::Any16
//#cpp2rust D3D_MIN_PRECISION_ANY_10    = d3d::MinPrecision::Any10
