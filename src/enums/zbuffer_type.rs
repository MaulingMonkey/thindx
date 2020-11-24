#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dzbuffertype)\]
/// D3DZBUFFERTYPE
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct ZBufferType(D3DZBUFFERTYPE);
pub type ZB = ZBufferType;

impl ZBufferType {
    /// Convert a raw [D3DZBUFFERTYPE] value into a [ZBufferType].  This is *probably* safe... probably....
    ///
    /// [D3DZBUFFERTYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dzbuffertype
    pub const fn from_unchecked(zbuffertype: D3DZBUFFERTYPE) -> Self { Self(zbuffertype) }

    /// Convert a [ZBufferType] into a raw [D3DZBUFFERTYPE].
    ///
    /// [D3DZBUFFERTYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dzbuffertype
    pub const fn into(self) -> D3DZBUFFERTYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl ZBufferType {
    pub const False : ZBufferType = ZBufferType(D3DZB_FALSE);
    pub const True  : ZBufferType = ZBufferType(D3DZB_TRUE);
    pub const UseW  : ZBufferType = ZBufferType(D3DZB_USEW);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for ZBufferType {
    fn default() -> Self { ZBufferType::False }
}

impl Debug for ZBufferType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ZBufferType::False  => write!(f, "ZBufferType::False"),
            ZBufferType::True   => write!(f, "ZBufferType::True"),
            ZBufferType::UseW   => write!(f, "ZBufferType::UseW"),
            other               => write!(f, "ZBufferType({})", other.0),
        }
    }
}

impl From<ZBufferType> for D3DZBUFFERTYPE {
    fn from(value: ZBufferType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DZBUFFERTYPE> for ZBufferType {
    fn from(value: D3DZBUFFERTYPE) -> Self { Self(value) }
}
