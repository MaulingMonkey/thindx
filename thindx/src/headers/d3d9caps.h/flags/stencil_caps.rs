use bytemuck::*;
use winapi::shared::d3d9caps::*;
use winapi::shared::minwindef::DWORD;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstencilcaps)\]
/// D3DSTENCILCAPS_*
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct StencilCaps(DWORD);

flags! {
    StencilCaps => DWORD;
    None, Keep, Zero, Replace, IncrSat, DecrSat, Invert, Incr, Decr, TwoSided,
}

#[allow(non_upper_case_globals)] impl StencilCaps {
    pub const None                          : StencilCaps = StencilCaps(0);
    pub const Keep                          : StencilCaps = StencilCaps(D3DSTENCILCAPS_KEEP);
    pub const Zero                          : StencilCaps = StencilCaps(D3DSTENCILCAPS_ZERO);
    pub const Replace                       : StencilCaps = StencilCaps(D3DSTENCILCAPS_REPLACE);
    pub const IncrSat                       : StencilCaps = StencilCaps(D3DSTENCILCAPS_INCRSAT);
    pub const DecrSat                       : StencilCaps = StencilCaps(D3DSTENCILCAPS_DECRSAT);
    pub const Invert                        : StencilCaps = StencilCaps(D3DSTENCILCAPS_INVERT);
    pub const Incr                          : StencilCaps = StencilCaps(D3DSTENCILCAPS_INCR);
    pub const Decr                          : StencilCaps = StencilCaps(D3DSTENCILCAPS_DECR);
    pub const TwoSided                      : StencilCaps = StencilCaps(D3DSTENCILCAPS_TWOSIDED);
}

//#cpp2rust D3DSTENCILCAPS_KEEP         = d3d::StencilCaps::Keep
//#cpp2rust D3DSTENCILCAPS_ZERO         = d3d::StencilCaps::Zero
//#cpp2rust D3DSTENCILCAPS_REPLACE      = d3d::StencilCaps::Replace
//#cpp2rust D3DSTENCILCAPS_INCRSAT      = d3d::StencilCaps::IncrSat
//#cpp2rust D3DSTENCILCAPS_DECRSAT      = d3d::StencilCaps::DecrSat
//#cpp2rust D3DSTENCILCAPS_INVERT       = d3d::StencilCaps::Invert
//#cpp2rust D3DSTENCILCAPS_INCR         = d3d::StencilCaps::Incr
//#cpp2rust D3DSTENCILCAPS_DECR         = d3d::StencilCaps::Decr
//#cpp2rust D3DSTENCILCAPS_TWOSIDED     = d3d::StencilCaps::TwoSided
