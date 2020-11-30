#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_driver_type)\]
/// D3D_DRIVER_TYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DriverType(D3D_DRIVER_TYPE);

enumish! { DriverType => D3D_DRIVER_TYPE; Unknown, Hardware, Reference, Null, Software, WARP }

#[allow(non_upper_case_globals)] impl DriverType { // These are enum-like
    pub const Unknown   : DriverType = DriverType(D3D_DRIVER_TYPE_UNKNOWN);
    pub const Hardware  : DriverType = DriverType(D3D_DRIVER_TYPE_HARDWARE);
    pub const Reference : DriverType = DriverType(D3D_DRIVER_TYPE_REFERENCE);
    pub const Null      : DriverType = DriverType(D3D_DRIVER_TYPE_NULL);
    pub const Software  : DriverType = DriverType(D3D_DRIVER_TYPE_SOFTWARE);
    // WARP is correct
}

#[doc(hidden)] impl DriverType { // Ctrl+C Ctrl+V support
    pub const UNKNOWN   : DriverType = DriverType(D3D_DRIVER_TYPE_UNKNOWN);
    pub const HARDWARE  : DriverType = DriverType(D3D_DRIVER_TYPE_HARDWARE);
    pub const REFERENCE : DriverType = DriverType(D3D_DRIVER_TYPE_REFERENCE);
    pub const NULL      : DriverType = DriverType(D3D_DRIVER_TYPE_NULL);
    pub const SOFTWARE  : DriverType = DriverType(D3D_DRIVER_TYPE_SOFTWARE);
    pub const WARP      : DriverType = DriverType(D3D_DRIVER_TYPE_WARP);
}

impl Default for DriverType {
    fn default() -> Self { DriverType(0) }
}
