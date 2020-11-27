#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcomposerectsop)\]
/// D3DCOMPOSERECTSOP
///
/// Specifies how to combine the glyph data from the source and destination surfaces in a call to [DeviceEx::compose_rects]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ComposeRectsOp(D3DCOMPOSERECTSOP);
pub use ComposeRectsOp as ComposeRects;

enumish! { ComposeRects => D3DCOMPOSERECTSOP; Copy, Or, And, Neg }

#[allow(non_upper_case_globals)] impl ComposeRectsOp { // These are enum-like
    pub const Copy  : ComposeRectsOp = ComposeRectsOp(D3DCOMPOSERECTS_COPY); // 1
    pub const Or    : ComposeRectsOp = ComposeRectsOp(D3DCOMPOSERECTS_OR);
    pub const And   : ComposeRectsOp = ComposeRectsOp(D3DCOMPOSERECTS_AND);
    pub const Neg   : ComposeRectsOp = ComposeRectsOp(D3DCOMPOSERECTS_NEG);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for ComposeRects {
    fn default() -> Self { ComposeRects::Copy } // 1
}
