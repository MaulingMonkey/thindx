#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpatchedgestyle)\]
/// D3DPATCHEDGESTYLE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct PatchEdgeStyle(D3DPATCHEDGESTYLE);
pub type PatchEdge = PatchEdgeStyle;

impl PatchEdgeStyle {
    /// Convert a raw [D3DPATCHEDGESTYLE] value into a [PatchEdgeStyle].  This is *probably* safe... probably....
    ///
    /// [D3DPATCHEDGESTYLE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpatchedgestyle
    pub const fn from_unchecked(patchedgestyle: D3DPATCHEDGESTYLE) -> Self { Self(patchedgestyle) }

    /// Convert a [PatchEdgeStyle] into a raw [D3DPATCHEDGESTYLE].
    ///
    /// [D3DPATCHEDGESTYLE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dpatchedgestyle
    pub const fn into(self) -> D3DPATCHEDGESTYLE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl PatchEdgeStyle {
    pub const Discrete      : PatchEdgeStyle = PatchEdgeStyle(D3DPATCHEDGE_DISCRETE);
    pub const Continuous    : PatchEdgeStyle = PatchEdgeStyle(D3DPATCHEDGE_CONTINUOUS);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for PatchEdgeStyle {
    fn default() -> Self { PatchEdgeStyle::Default }
}

impl Debug for PatchEdgeStyle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            PatchEdgeStyle::Discrete    => write!(f, "PatchEdgeStyle::Discrete"),
            PatchEdgeStyle::Continuous  => write!(f, "PatchEdgeStyle::Continuous"),
            other                       => write!(f, "PatchEdgeStyle({})", other.0),
        }
    }
}

impl From<PatchEdgeStyle> for D3DPATCHEDGESTYLE {
    fn from(value: PatchEdgeStyle) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DPATCHEDGESTYLE> for PatchEdgeStyle {
    fn from(value: D3DPATCHEDGESTYLE) -> Self { Self(value) }
}
