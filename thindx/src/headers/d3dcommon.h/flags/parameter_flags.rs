#[allow(unused_imports)] use crate::*;

use winapi::um::d3dcommon::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3dcommon/ne-d3dcommon-d3d_parameter_flags)\]
/// UINT / D3D_PF_\*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ParameterFlags(D3D_PARAMETER_FLAGS);
#[doc(hidden)] pub use ParameterFlags as PF;

flags! { PF => D3D_PARAMETER_FLAGS; None, In, Out }

#[allow(non_upper_case_globals)] impl PF { // These are enum-like
    /// The parameter has no semantic flags.
    pub const None  : PF = PF(D3D_PF_NONE);

    /// The parameter is an input parameter (e.g. marked `in`)
    pub const In    : PF = PF(D3D_PF_IN);

    /// The parameter is an output parameter (e.g. marked `out`)
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

//#cpp2rust D3D_PARAMETER_FLAGS = d3d::ParameterFlags
//#cpp2rust D3D_PF_NONE         = d3d::PF::None
//#cpp2rust D3D_PF_IN           = d3d::PF::In
//#cpp2rust D3D_PF_OUT          = d3d::PF::Out
