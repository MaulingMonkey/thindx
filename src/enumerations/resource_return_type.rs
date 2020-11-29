#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_resource_return_type)\]
/// D3D_RESOURCE_RETURN_TYPE / D3D_RETURN_TYPE_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ResourceReturnType(D3D_RESOURCE_RETURN_TYPE);
#[doc(hidden)] pub use ResourceReturnType as ReturnType;

enumish! {
    ReturnType => D3D_RESOURCE_RETURN_TYPE;
}

#[allow(non_upper_case_globals)] impl ReturnType { // These are enum-like
    // TODO
}

#[doc(hidden)] impl ReturnType { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for ReturnType {
    fn default() -> Self { ReturnType(0) }
}
