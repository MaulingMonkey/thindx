#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblendop)\]
/// D3DBLENDOP
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BlendOp(D3DBLENDOP);

impl BlendOp {
    /// Convert a raw [D3DBLENDOP] value into a [BlendOp].  This is *probably* safe... probably....
    ///
    /// [D3DBLENDOP]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblendop
    pub const fn from_unchecked(blendop: D3DBLENDOP) -> Self { Self(blendop) }

    /// Convert a [BlendOp] into a raw [D3DBLENDOP].
    ///
    /// [D3DBLENDOP]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dblendop
    pub const fn into(self) -> D3DBLENDOP { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl BlendOp {
    pub const Add           : BlendOp = BlendOp(D3DBLENDOP_ADD);
    pub const Subtract      : BlendOp = BlendOp(D3DBLENDOP_SUBTRACT);
    pub const RevSubtract   : BlendOp = BlendOp(D3DBLENDOP_REVSUBTRACT);
    pub const Min           : BlendOp = BlendOp(D3DBLENDOP_MIN);
    pub const Max           : BlendOp = BlendOp(D3DBLENDOP_MAX);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for BlendOp {
    fn default() -> Self { BlendOp::Add }
}

impl Debug for BlendOp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            BlendOp::Add            => write!(f, "BlendOp::Add"),
            BlendOp::Subtract       => write!(f, "BlendOp::Subtract"),
            BlendOp::RevSubtract    => write!(f, "BlendOp::RevSubtract"),
            BlendOp::Min            => write!(f, "BlendOp::Min"),
            BlendOp::Max            => write!(f, "BlendOp::Max"),
            other                   => write!(f, "BlendOp({})", other.0),
        }
    }
}

impl From<BlendOp> for D3DBLENDOP {
    fn from(value: BlendOp) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DBLENDOP> for BlendOp {
    fn from(value: D3DBLENDOP) -> Self { Self(value) }
}
