#[allow(unused_imports)] use crate::*;

use bytemuck::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dscanlineordering)\]
/// D3DSCANLINEORDERING
///
/// Flags indicating the method the rasterizer uses to create an image on a surface.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ScanlineOrdering(D3DSCANLINEORDERING);

enumish! { ScanlineOrdering => D3DSCANLINEORDERING; Progressive, Interlaced, Unknown }

#[allow(non_upper_case_globals)] impl ScanlineOrdering { // These are enum-like
    pub const Progressive   : ScanlineOrdering = ScanlineOrdering(D3DSCANLINEORDERING_PROGRESSIVE);
    pub const Interlaced    : ScanlineOrdering = ScanlineOrdering(D3DSCANLINEORDERING_INTERLACED);
    pub const Unknown       : ScanlineOrdering = ScanlineOrdering(D3DSCANLINEORDERING_UNKNOWN);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for ScanlineOrdering {
    fn default() -> Self { ScanlineOrdering::Progressive } // 1
}

//#cpp2rust D3DSCANLINEORDERING             = d3d::ScanlineOrdering
//#cpp2rust D3DSCANLINEORDERING_PROGRESSIVE = d3d::ScanlineOrdering::Progressive
//#cpp2rust D3DSCANLINEORDERING_INTERLACED  = d3d::ScanlineOrdering::Interlaced
//#cpp2rust D3DSCANLINEORDERING_UNKNOWN     = d3d::ScanlineOrdering::Unknown
