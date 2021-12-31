#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcull)\]
/// D3DCULL
///
/// Defines the supported culling modes ([None](crate::Cull::None), [CW](crate::Cull::CW), [CCW](crate::Cull::CCW))
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Cull(D3DCULL);

enumish! { Cull => D3DCULL; None, CW, CCW }

#[allow(non_upper_case_globals)] impl Cull { // These are enum-like
    pub const None      : Cull = Cull(D3DCULL_NONE); // 1
    pub const CW        : Cull = Cull(D3DCULL_CW);
    pub const CCW       : Cull = Cull(D3DCULL_CCW);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for Cull {
    fn default() -> Self { Cull::None } // 1
}
