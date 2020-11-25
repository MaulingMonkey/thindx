#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfogmode)\]
/// D3DFOGMODE
///
/// Defines constants that describe the fog mode.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Fog(D3DFOGMODE);
pub type FogMode = Fog;

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
