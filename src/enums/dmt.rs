#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddebugmonitortokens)\]
/// D3DDEBUGMONITORTOKENS
///
/// Defines the debug monitor tokens.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct DMT(D3DDEBUGMONITORTOKENS);

impl DMT {
    /// Convert a raw [D3DDEBUGMONITORTOKENS] value into a [DMT].  This is *probably* safe... probably....
    ///
    /// [D3DDEBUGMONITORTOKENS]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddebugmonitortokens
    pub const fn from_unchecked(debugmonitortokens: D3DDEBUGMONITORTOKENS) -> Self { Self(debugmonitortokens) }

    /// Convert a [DMT] into a raw [D3DDEBUGMONITORTOKENS].
    ///
    /// [D3DDEBUGMONITORTOKENS]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3ddebugmonitortokens
    pub const fn into(self) -> D3DDEBUGMONITORTOKENS { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl DMT {
    pub const Enable        : DMT = DMT(D3DDMT_ENABLE); // 0
    pub const Disable       : DMT = DMT(D3DDMT_DISABLE); // 1
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for DMT {
    fn default() -> Self { DMT::Disable }
}

impl Debug for DMT {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DMT::Enable     => write!(f, "DMT::Enable"),
            DMT::Disable    => write!(f, "DMT::Disable"),
            other           => write!(f, "DMT({})", other.0),
        }
    }
}

impl From<DMT> for D3DDEBUGMONITORTOKENS {
    fn from(value: DMT) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DDEBUGMONITORTOKENS> for DMT {
    fn from(value: D3DDEBUGMONITORTOKENS) -> Self { Self(value) }
}
