#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_partitioning)\]
/// D3D_TESSELLATOR_PARTITIONING
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TessellatorPartitioning(D3D_TESSELLATOR_PARTITIONING);

enumish! {
    TessellatorPartitioning => D3D_TESSELLATOR_PARTITIONING;
}

#[allow(non_upper_case_globals)] impl TessellatorPartitioning { // These are enum-like
    // TODO
}

#[doc(hidden)] impl TessellatorPartitioning { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for TessellatorPartitioning {
    fn default() -> Self { TessellatorPartitioning(0) }
}
