#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblendop)\]
/// D3DBLENDOP
///
/// Defines the supported blend operations.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BlendOp(D3DBLENDOP);

enumish! { BlendOp => D3DBLENDOP; Add, Subtract, RevSubtract, Min, Max }

#[allow(non_upper_case_globals)] impl BlendOp { // These are enum-like
    pub const Add           : BlendOp = BlendOp(D3DBLENDOP_ADD); // 1
    pub const Subtract      : BlendOp = BlendOp(D3DBLENDOP_SUBTRACT);
    pub const RevSubtract   : BlendOp = BlendOp(D3DBLENDOP_REVSUBTRACT);
    pub const Min           : BlendOp = BlendOp(D3DBLENDOP_MIN);
    pub const Max           : BlendOp = BlendOp(D3DBLENDOP_MAX);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for BlendOp {
    fn default() -> Self { BlendOp::Add } // 1
}
