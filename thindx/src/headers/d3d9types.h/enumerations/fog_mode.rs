#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfogmode)\]
/// D3DFOGMODE
///
/// Defines constants that describe the fog mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct FogMode(D3DFOGMODE);
pub use FogMode as Fog;

enumish! { Fog => D3DFOGMODE; None, Exp, Exp2, Linear }

#[allow(non_upper_case_globals)] impl Fog { // These are enum-like
    pub const None      : Fog = Fog(D3DFOG_NONE); // 0
    pub const Exp       : Fog = Fog(D3DFOG_EXP);
    pub const Exp2      : Fog = Fog(D3DFOG_EXP2);
    pub const Linear    : Fog = Fog(D3DFOG_LINEAR);
}

impl Default for Fog {
    fn default() -> Self { Fog::None } // 0
}

//#cpp2rust D3DFOGMODE      = d3d::FogMode
//#cpp2rust D3DFOG_NONE     = d3d::Fog::None
//#cpp2rust D3DFOG_EXP      = d3d::Fog::Exp
//#cpp2rust D3DFOG_EXP2     = d3d::Fog::Exp2
//#cpp2rust D3DFOG_LINEAR   = d3d::Fog::Linear
