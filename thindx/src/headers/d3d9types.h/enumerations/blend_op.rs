#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dblendop)\]
/// D3DBLENDOP
///
/// Defines the supported blend operations.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct BlendOp(D3DBLENDOP);

enumish! { BlendOp => D3DBLENDOP; Add, Subtract, RevSubtract, Min, Max }

#[allow(non_upper_case_globals)] impl BlendOp { // These are enum-like
    pub const Add           : BlendOp = BlendOp(D3DBLENDOP_ADD); // 1
    pub const Subtract      : BlendOp = BlendOp(D3DBLENDOP_SUBTRACT);
    pub const RevSubtract   : BlendOp = BlendOp(D3DBLENDOP_REVSUBTRACT);
    pub const Min           : BlendOp = BlendOp(D3DBLENDOP_MIN);
    pub const Max           : BlendOp = BlendOp(D3DBLENDOP_MAX);
}

//#cpp2rust D3DBLENDOP              = d3d::BlendOp

//#cpp2rust D3DBLENDOP_ADD          = d3d::BlendOp::Add
//#cpp2rust D3DBLENDOP_SUBTRACT     = d3d::BlendOp::Subtract
//#cpp2rust D3DBLENDOP_REVSUBTRACT  = d3d::BlendOp::RevSubtract
//#cpp2rust D3DBLENDOP_MIN          = d3d::BlendOp::Min
//#cpp2rust D3DBLENDOP_MAX          = d3d::BlendOp::Max
