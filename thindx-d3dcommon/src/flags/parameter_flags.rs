#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_parameter_flags)\]
/// UINT / D3D_PF_\*
///
/// Indicates semantic flags for function parameters.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ParameterFlags(D3D_PARAMETER_FLAGS);
#[doc(hidden)] pub use ParameterFlags as PF;

flags! { PF => D3D_PARAMETER_FLAGS; None, In, Out }

#[allow(non_upper_case_globals)] impl PF { // These are enum-like
    pub const None  : PF = PF(D3D_PF_NONE);
    pub const In    : PF = PF(D3D_PF_IN);
    pub const Out   : PF = PF(D3D_PF_OUT);
}

#[doc(hidden)] impl PF { // Ctrl+C Ctrl+V support
    pub const NONE  : PF = PF(D3D_PF_NONE);
    pub const IN    : PF = PF(D3D_PF_IN);
    pub const OUT   : PF = PF(D3D_PF_OUT);
}

impl Default for PF {
    fn default() -> Self { PF::None }
}
