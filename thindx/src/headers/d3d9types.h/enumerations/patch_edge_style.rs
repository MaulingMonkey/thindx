#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpatchedgestyle)\]
/// D3DPATCHEDGESTYLE
///
/// Defines whether the current tessellation mode is [Discrete](crate::PatchEdge::Discrete) or [Continuous](crate::PatchEdge::Continuous).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PatchEdgeStyle(D3DPATCHEDGESTYLE);
pub use PatchEdgeStyle as PatchEdge;

enumish! { PatchEdge => D3DPATCHEDGESTYLE; Discrete, Continuous }

#[allow(non_upper_case_globals)] impl PatchEdgeStyle { // These are enum-like
    pub const Discrete      : PatchEdgeStyle = PatchEdgeStyle(D3DPATCHEDGE_DISCRETE); // 0
    pub const Continuous    : PatchEdgeStyle = PatchEdgeStyle(D3DPATCHEDGE_CONTINUOUS);
}

impl Default for PatchEdgeStyle {
    fn default() -> Self { PatchEdgeStyle::Discrete } // 0
}

//#cpp2rust D3DPATCHEDGESTYLE           = d3d::PatchEdgeStyle
//#cpp2rust D3DPATCHEDGE_DISCRETE       = d3d::PatchEdge::Discrete
//#cpp2rust D3DPATCHEDGE_CONTINUOUS     = d3d::PatchEdge::Continuous
