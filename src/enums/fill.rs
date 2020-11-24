#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfillmode)\]
/// D3DFILLMODE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct Fill(D3DFILLMODE);

impl Fill {
    /// Convert a raw [D3DFILLMODE] value into a [Fill].  This is *probably* safe... probably....
    ///
    /// [D3DFILLMODE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfillmode
    pub const fn from_unchecked(fillmode: D3DFILLMODE) -> Self { Self(fillmode) }

    /// Convert a [Fill] into a raw [D3DFILLMODE].
    ///
    /// [D3DFILLMODE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dfillmode
    pub const fn into(self) -> D3DFILLMODE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl Fill {
    pub const Point         : Fill = Fill(D3DFILL_POINT);
    pub const Wireframe     : Fill = Fill(D3DFILL_WIREFRAME);
    pub const Solid         : Fill = Fill(D3DFILL_SOLID);
}

impl Default for Fill {
    fn default() -> Self { Fill::Solid }
}

impl Debug for Fill {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Fill::Point     => write!(f, "Fill::Point"),
            Fill::Wireframe => write!(f, "Fill::Wireframe"),
            Fill::Solid     => write!(f, "Fill::Solid"),
            other           => write!(f, "Fill({})", other.0),
        }
    }
}

impl From<Fill> for D3DFILLMODE {
    fn from(value: Fill) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DFILLMODE> for Fill {
    fn from(value: D3DFILLMODE) -> Self { Self(value) }
}
