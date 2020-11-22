#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dbackbuffer-type)\]
/// D3DBACKBUFFER_TYPE
///
/// Predefined sets of pipeline state used by state blocks (see [State Blocks Save and Restore State (Direct3D 9)]).
///
/// [State Blocks Save and Restore State (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state
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
    pub const Mono          : BackBufferType = BackBufferType(D3DSBT_ALL);
    pub const Left    : BackBufferType = BackBufferType(D3DSBT_PIXELSTATE);
    pub const Right   : BackBufferType = BackBufferType(D3DSBT_VERTEXSTATE);
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
