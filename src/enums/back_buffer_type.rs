#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbackbuffer-type)\]
/// D3DBACKBUFFER_TYPE
///
/// Defines constants that describe the type of back buffer.
///
/// Direct3D 9 does not support stereo view, so Direct3D does not use the D3DBACKBUFFER_TYPE_LEFT and D3DBACKBUFFER_TYPE_RIGHT values of this enumerated type.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct BackBufferType(D3DBACKBUFFER_TYPE);

impl BackBufferType {
    /// Convert a raw [D3DBACKBUFFER_TYPE] value into a [BackBufferType].  This is *probably* safe... probably....
    ///
    /// [D3DBACKBUFFER_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbackbuffer-type
    pub const fn from_unchecked(backbuffertype: D3DBACKBUFFER_TYPE) -> Self { Self(backbuffertype) }

    /// Convert a [BackBufferType] into a raw [D3DBACKBUFFER_TYPE].
    ///
    /// [D3DBACKBUFFER_TYPE]:      https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbackbuffer-type
    pub const fn into(self) -> D3DBACKBUFFER_TYPE { self.0 }
}

#[allow(non_upper_case_globals)] // These are enum-like
impl BackBufferType {
    /// Specifies a nonstereo swap chain.
    pub const Mono  : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_MONO);

    /// Specifies the left side of a stereo pair in a swap chain.
    pub const Left  : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_LEFT);

    /// Specifies the right side of a stereo pair in a swap chain.
    pub const Right : BackBufferType = BackBufferType(D3DBACKBUFFER_TYPE_RIGHT);
}

impl Default for BackBufferType {
    fn default() -> Self { BackBufferType::Mono }
}

impl Debug for BackBufferType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            BackBufferType::Mono    => write!(f, "BackBufferType::Mono"),
            BackBufferType::Left    => write!(f, "BackBufferType::Left"),
            BackBufferType::Right   => write!(f, "BackBufferType::Right"),
            other                   => write!(f, "BackBufferType({})", other.0),
        }
    }
}

impl From<BackBufferType> for D3DBACKBUFFER_TYPE {
    fn from(value: BackBufferType) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DBACKBUFFER_TYPE> for BackBufferType {
    fn from(value: D3DBACKBUFFER_TYPE) -> Self { Self(value) }
}
