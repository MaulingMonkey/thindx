#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstencilop)\]
/// D3DSTENCILOP
///
/// Defines stencil-buffer operations.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct StencilOp(D3DSTENCILOP);

enumish! { StencilOp => D3DSTENCILOP; Keep, Zero, Replace, IncrSat, DecrSat, Invert, Incr, Decr }

#[allow(non_upper_case_globals)] impl StencilOp { // These are enum-like
    pub const Keep          : StencilOp = StencilOp(D3DSTENCILOP_KEEP);
    pub const Zero          : StencilOp = StencilOp(D3DSTENCILOP_ZERO);
    pub const Replace       : StencilOp = StencilOp(D3DSTENCILOP_REPLACE);
    pub const IncrSat       : StencilOp = StencilOp(D3DSTENCILOP_INCRSAT);
    pub const DecrSat       : StencilOp = StencilOp(D3DSTENCILOP_DECRSAT);
    pub const Invert        : StencilOp = StencilOp(D3DSTENCILOP_INVERT);
    pub const Incr          : StencilOp = StencilOp(D3DSTENCILOP_INCR);
    pub const Decr          : StencilOp = StencilOp(D3DSTENCILOP_DECR);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for StencilOp {
    fn default() -> Self { StencilOp(0) }
}
