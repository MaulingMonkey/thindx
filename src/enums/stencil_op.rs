#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstencilop)\]
/// D3DSTENCILOP
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct StencilOp(D3DSTENCILOP);

impl StencilOp {
    /// Convert a raw [D3DSTENCILOP] value into a [StencilOp].  This is *probably* safe... probably....
    ///
    /// [D3DSTENCILOP]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstencilop
    pub const fn from_unchecked(stencilop: D3DSTENCILOP) -> Self { Self(stencilop) }

    /// Convert a [StencilOp] into a raw [D3DSTENCILOP].
    ///
    /// [D3DSTENCILOP]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dstencilop
    pub const fn into(self) -> D3DSTENCILOP { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl StencilOp {
    pub const Keep          : StencilOp = StencilOp(D3DSTENCILOP_KEEP);
    pub const Zero          : StencilOp = StencilOp(D3DSTENCILOP_ZERO);
    pub const Replace       : StencilOp = StencilOp(D3DSTENCILOP_REPLACE);
    pub const IncrSat       : StencilOp = StencilOp(D3DSTENCILOP_INCRSAT);
    pub const DecrSat       : StencilOp = StencilOp(D3DSTENCILOP_DECRSAT);
    pub const Invert        : StencilOp = StencilOp(D3DSTENCILOP_INVERT);
    pub const Incr          : StencilOp = StencilOp(D3DSTENCILOP_INCR);
    pub const Decr          : StencilOp = StencilOp(D3DSTENCILOP_DECR);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for StencilOp {
    fn default() -> Self { StencilOp(0) }
}

impl Debug for StencilOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            StencilOp::Keep     => write!(f, "StencilOp::Keep"),
            StencilOp::Zero     => write!(f, "StencilOp::Zero"),
            StencilOp::Replace  => write!(f, "StencilOp::Replace"),
            StencilOp::IncrSat  => write!(f, "StencilOp::IncrSat"),
            StencilOp::DecrSat  => write!(f, "StencilOp::DecrSat"),
            StencilOp::Invert   => write!(f, "StencilOp::Invert"),
            StencilOp::Incr     => write!(f, "StencilOp::Incr"),
            StencilOp::Decr     => write!(f, "StencilOp::Decr"),
            other               => write!(f, "StencilOp({})", other.0),
        }
    }
}

impl From<StencilOp> for D3DSTENCILOP {
    fn from(value: StencilOp) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DSTENCILOP> for StencilOp {
    fn from(value: D3DSTENCILOP) -> Self { Self(value) }
}
