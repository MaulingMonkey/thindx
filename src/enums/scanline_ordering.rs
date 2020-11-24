#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dscanlineordering)\]
/// D3DSCANLINEORDERING
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ScanlineOrdering(D3DSCANLINEORDERING);

impl ScanlineOrdering {
    /// Convert a raw [D3DSCANLINEORDERING] value into a [ScanlineOrdering].  This is *probably* safe... probably....
    ///
    /// [D3DSCANLINEORDERING]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dscanlineordering
    pub const fn from_unchecked(scanlineordering: D3DSCANLINEORDERING) -> Self { Self(scanlineordering) }

    /// Convert a [ScanlineOrdering] into a raw [D3DSCANLINEORDERING].
    ///
    /// [D3DSCANLINEORDERING]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dscanlineordering
    pub const fn into(self) -> D3DSCANLINEORDERING { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl ScanlineOrdering {
    pub const Progressive   : ScanlineOrdering = ScanlineOrdering(D3DSCANLINEORDERING_PROGRESSIVE);
    pub const Interlaced    : ScanlineOrdering = ScanlineOrdering(D3DSCANLINEORDERING_INTERLACED);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for ScanlineOrdering {
    fn default() -> Self { ScanlineOrdering::Progressive }
}

impl Debug for ScanlineOrdering {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ScanlineOrdering::Progressive   => write!(f, "ScanlineOrdering::Progressive"),
            ScanlineOrdering::Interlaced    => write!(f, "ScanlineOrdering::Interlaced"),
            other                           => write!(f, "ScanlineOrdering({})", other.0),
        }
    }
}

impl From<ScanlineOrdering> for D3DSCANLINEORDERING {
    fn from(value: ScanlineOrdering) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DSCANLINEORDERING> for ScanlineOrdering {
    fn from(value: D3DSCANLINEORDERING) -> Self { Self(value) }
}
