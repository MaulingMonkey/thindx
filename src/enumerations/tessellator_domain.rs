#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_tessellator_domain)\]
/// D3D_TESSELLATOR_DOMAIN
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct TessellatorDomain(D3D_TESSELLATOR_DOMAIN);

enumish! {
    TessellatorDomain => D3D_TESSELLATOR_DOMAIN;
}

#[allow(non_upper_case_globals)] impl TessellatorDomain { // These are enum-like
    // TODO
}

#[doc(hidden)] impl TessellatorDomain { // Ctrl+C Ctrl+V support
    // TODO
}

impl Default for TessellatorDomain {
    fn default() -> Self { TessellatorDomain(0) }
}
