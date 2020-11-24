#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcull)\]
/// D3DCULL
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Cull(D3DCULL);

impl Cull {
    /// Convert a raw [D3DCULL] value into a [Cull].  This is *probably* safe... probably....
    ///
    /// [D3DCULL]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcull
    pub const fn from_unchecked(cull: D3DCULL) -> Self { Self(cull) }

    /// Convert a [Cull] into a raw [D3DCULL].
    ///
    /// [D3DCULL]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dcull
    pub const fn into(self) -> D3DCULL { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Cull {
    pub const None      : Cull = Cull(D3DCULL_NONE);
    pub const CW        : Cull = Cull(D3DCULL_CW);
    pub const CCW       : Cull = Cull(D3DCULL_CCW);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for Cull {
    fn default() -> Self { Cull::Default }
}

impl Debug for Cull {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Cull::None  => write!(f, "Cull::None"),
            Cull::CW    => write!(f, "Cull::CW"),
            Cull::CCW   => write!(f, "Cull::CCW"),
            other       => write!(f, "Cull({})", other.0),
        }
    }
}

impl From<Cull> for D3DCULL {
    fn from(value: Cull) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DCULL> for Cull {
    fn from(value: D3DCULL) -> Self { Self(value) }
}
