#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;
type D3DGETDATA = u32; // there's no actual type

use std::fmt::{self, Debug, Formatter};
use std::ops::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlock)\]
/// DWORD / D3DGETDATA_*
///
/// Controls how [Query::get_data_inplace] behaves.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct GetData(D3DGETDATA);

impl GetData {
    pub const fn from_unchecked(lock: D3DGETDATA) -> Self { Self(lock) }
    pub const fn into(self) -> D3DGETDATA { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl GetData {
    /// No flags
    pub const None      : GetData = GetData(0);

    /// [D3DGETDATA_FLUSH](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dgetdata-flush)
    pub const Flush     : GetData = GetData(D3DGETDATA_FLUSH);
}

impl BitOrAssign for GetData {
    fn bitor_assign(&mut self, other: Self) { self.0 |= other.0 }
}

impl BitOr for GetData {
    type Output = Self;
    fn bitor(self, other: Self) -> Self { Self(self.0 | other.0) }
}

impl Debug for GetData {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            GetData::None   => write!(f, "GetData::None"),
            GetData::Flush  => write!(f, "GetData::Flush"),
            other           => write!(f, "GetData({})", other.0),
        }
    }
}

impl From<GetData> for D3DGETDATA {
    fn from(value: GetData) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DGETDATA> for GetData {
    fn from(value: D3DGETDATA) -> Self { Self(value) }
}
