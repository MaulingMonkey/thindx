#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcomposerectsop)\]
/// D3DCOMPOSERECTSOP
///
/// Specifies how to combine the glyph data from the source and destination surfaces in a call to [DeviceEx::compose_rects]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ComposeRects(D3DCOMPOSERECTSOP);

enumish! { ComposeRects => D3DCOMPOSERECTSOP; Copy, Or, And, Neg }

#[allow(non_upper_case_globals)] impl ComposeRects { // These are enum-like
    pub const Copy  : ComposeRects = ComposeRects(D3DCOMPOSERECTS_COPY); // 1
    pub const Or    : ComposeRects = ComposeRects(D3DCOMPOSERECTS_OR);
    pub const And   : ComposeRects = ComposeRects(D3DCOMPOSERECTS_AND);
    pub const Neg   : ComposeRects = ComposeRects(D3DCOMPOSERECTS_NEG);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for ComposeRects {
    fn default() -> Self { ComposeRects::Copy } // 1
}
