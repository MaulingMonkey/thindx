#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcomposerectsop)\]
/// D3DCOMPOSERECTSOP
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ComposeRects(D3DCOMPOSERECTSOP);

impl ComposeRects {
    /// Convert a raw [D3DCOMPOSERECTSOP] value into a [ComposeRects].  This is *probably* safe... probably....
    ///
    /// [D3DCOMPOSERECTSOP]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcomposerectsop
    pub const fn from_unchecked(composerects: D3DCOMPOSERECTSOP) -> Self { Self(composerects) }

    /// Convert a [ComposeRects] into a raw [D3DCOMPOSERECTSOP].
    ///
    /// [D3DCOMPOSERECTSOP]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcomposerectsop
    pub const fn into(self) -> D3DCOMPOSERECTSOP { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl ComposeRects {
    pub const Copy  : ComposeRects = ComposeRects(D3DCOMPOSERECTS_COPY);
    pub const Or    : ComposeRects = ComposeRects(D3DCOMPOSERECTS_OR);
    pub const And   : ComposeRects = ComposeRects(D3DCOMPOSERECTS_AND);
    pub const Neg   : ComposeRects = ComposeRects(D3DCOMPOSERECTS_NEG);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for ComposeRects {
    fn default() -> Self { ComposeRects::Copy }
}

impl Debug for ComposeRects {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ComposeRects::Copy  => write!(f, "ComposeRects::Copy"),
            ComposeRects::Or    => write!(f, "ComposeRects::Or"),
            ComposeRects::And   => write!(f, "ComposeRects::And"),
            ComposeRects::Neg   => write!(f, "ComposeRects::Neg"),
            other               => write!(f, "ComposeRects({})", other.0),
        }
    }
}

impl From<ComposeRects> for D3DCOMPOSERECTSOP {
    fn from(value: ComposeRects) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DCOMPOSERECTSOP> for ComposeRects {
    fn from(value: D3DCOMPOSERECTSOP) -> Self { Self(value) }
}
