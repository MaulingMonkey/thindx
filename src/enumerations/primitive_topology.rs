#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_primitive_topology)\]
/// D3D_PRIMITIVE_TOPOLOGY
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY);

enumish! {
    PrimitiveTopology => D3D_PRIMITIVE_TOPOLOGY;
}

#[allow(non_upper_case_globals)] impl PrimitiveTopology { // These are enum-like
    // TODO
}

#[doc(hidden)] impl PrimitiveTopology { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for PrimitiveTopology {
    fn default() -> Self { PrimitiveTopology(0) }
}
