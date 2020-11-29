#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_srv_dimension)\]
/// D3D_SRV_DIMENSION
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct SrvDimension(D3D_SRV_DIMENSION);

enumish! {
    SrvDimension => D3D_SRV_DIMENSION;
}

#[allow(non_upper_case_globals)] impl SrvDimension { // These are enum-like
    // TODO
}

#[doc(hidden)] impl SrvDimension { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for SrvDimension {
    fn default() -> Self { SrvDimension(0) }
}
