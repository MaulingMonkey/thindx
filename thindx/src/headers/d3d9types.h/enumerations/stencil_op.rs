#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dstencilop)\]
/// D3DSTENCILOP
///
/// Defines stencil-buffer operations.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct StencilOp(D3DSTENCILOP);

enumish! { StencilOp => D3DSTENCILOP; Keep, Zero, Replace, IncrSat, DecrSat, Invert, Incr, Decr }

#[allow(non_upper_case_globals)] impl StencilOp { // These are enum-like
    pub const Keep          : StencilOp = StencilOp(D3DSTENCILOP_KEEP); // 1
    pub const Zero          : StencilOp = StencilOp(D3DSTENCILOP_ZERO);
    pub const Replace       : StencilOp = StencilOp(D3DSTENCILOP_REPLACE);
    pub const IncrSat       : StencilOp = StencilOp(D3DSTENCILOP_INCRSAT);
    pub const DecrSat       : StencilOp = StencilOp(D3DSTENCILOP_DECRSAT);
    pub const Invert        : StencilOp = StencilOp(D3DSTENCILOP_INVERT);
    pub const Incr          : StencilOp = StencilOp(D3DSTENCILOP_INCR);
    pub const Decr          : StencilOp = StencilOp(D3DSTENCILOP_DECR);
}

//#cpp2rust D3DSTENCILOP            = d3d::StencilOp

//#cpp2rust D3DSTENCILOP_KEEP       = d3d::StencilOp::Keep
//#cpp2rust D3DSTENCILOP_ZERO       = d3d::StencilOp::Zero
//#cpp2rust D3DSTENCILOP_REPLACE    = d3d::StencilOp::Replace
//#cpp2rust D3DSTENCILOP_INCRSAT    = d3d::StencilOp::IncrSat
//#cpp2rust D3DSTENCILOP_DECRSAT    = d3d::StencilOp::DecrSat
//#cpp2rust D3DSTENCILOP_INVERT     = d3d::StencilOp::Invert
//#cpp2rust D3DSTENCILOP_INCR       = d3d::StencilOp::Incr
//#cpp2rust D3DSTENCILOP_DECR       = d3d::StencilOp::Decr
