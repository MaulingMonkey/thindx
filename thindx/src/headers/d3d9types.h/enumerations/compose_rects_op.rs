#[allow(unused_imports)] use crate::*;
use bytemuck::*;
use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcomposerectsop)\]
/// D3DCOMPOSERECTSOP
///
/// Specifies how to combine the glyph data from the source and destination surfaces in a call to [IDirect3DDevice9ExExt::compose_rects]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Pod, Zeroable)]
#[repr(transparent)] pub struct ComposeRectsOp(D3DCOMPOSERECTSOP);
pub use ComposeRectsOp as ComposeRects;

enumish! { ComposeRects => D3DCOMPOSERECTSOP; Copy, Or, And, Neg }

#[allow(non_upper_case_globals)] impl ComposeRectsOp { // These are enum-like
    pub const Copy  : ComposeRectsOp = ComposeRectsOp(D3DCOMPOSERECTS_COPY); // 1
    pub const Or    : ComposeRectsOp = ComposeRectsOp(D3DCOMPOSERECTS_OR);
    pub const And   : ComposeRectsOp = ComposeRectsOp(D3DCOMPOSERECTS_AND);
    pub const Neg   : ComposeRectsOp = ComposeRectsOp(D3DCOMPOSERECTS_NEG);
}

//#cpp2rust D3DCOMPOSERECTSOP           = d3d::ComposeRectsOp

//#cpp2rust D3DCOMPOSERECTS_COPY        = d3d::ComposeRects::Copy
//#cpp2rust D3DCOMPOSERECTS_OR          = d3d::ComposeRects::Or
//#cpp2rust D3DCOMPOSERECTS_AND         = d3d::ComposeRects::And
//#cpp2rust D3DCOMPOSERECTS_NEG         = d3d::ComposeRects::Neg
