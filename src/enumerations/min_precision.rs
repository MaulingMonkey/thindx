#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_min_precision)\]
/// D3D_MIN_PRECISION
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct MinPrecision(D3D_MIN_PRECISION);

enumish! {
    MinPrecision => D3D_MIN_PRECISION;
}

#[allow(non_upper_case_globals)] impl MinPrecision { // These are enum-like
    // TODO
}

#[doc(hidden)] impl MinPrecision { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for MinPrecision {
    fn default() -> Self { MinPrecision(0) }
}
