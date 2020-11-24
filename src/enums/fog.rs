#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfogmode)\]
/// D3DFOGMODE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Fog(D3DFOGMODE);

impl Fog {
    /// Convert a raw [D3DFOGMODE] value into a [Fog].  This is *probably* safe... probably....
    ///
    /// [D3DFOGMODE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfogmode
    pub const fn from_unchecked(fogmode: D3DFOGMODE) -> Self { Self(fogmode) }

    /// Convert a [Fog] into a raw [D3DFOGMODE].
    ///
    /// [D3DFOGMODE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfogmode
    pub const fn into(self) -> D3DFOGMODE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Fog {
    pub const None      : Fog = Fog(D3DFOG_NONE);
    pub const Exp       : Fog = Fog(D3DFOG_EXP);
    pub const Exp2      : Fog = Fog(D3DFOG_EXP2);
    pub const Linear    : Fog = Fog(D3DFOG_LINEAR);
}

#[cfg(feature = "impl-poor-defaults")] // Actually this seems like a pretty sane default?
impl Default for Fog {
    fn default() -> Self { Fog::None }
}

impl Debug for Fog {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Fog::None       => write!(f, "Fog::None"),
            Fog::Exp        => write!(f, "Fog::Exp"),
            Fog::Exp2       => write!(f, "Fog::Exp2"),
            Fog::Linear     => write!(f, "Fog::Linear"),
            other           => write!(f, "Fog({})", other.0),
        }
    }
}

impl From<Fog> for D3DFOGMODE {
    fn from(value: Fog) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DFOGMODE> for Fog {
    fn from(value: D3DFOGMODE) -> Self { Self(value) }
}
