#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::fmt::{self, Debug, Formatter};
use std::ops::*;


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9)\]
/// D3DCLIPSTATUS9
///
/// Describes the current clip status.
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct ClipStatus(winapi::shared::d3d9types::D3DCLIPSTATUS9);

impl ClipStatus {
    /// Convert a raw [D3DCLIPSTATUS9] value into a [ClipStatus].  This is *probably* safe... probably...
    ///
    /// [D3DCLIPSTATUS9]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9
    pub const fn from_unchecked(clipstatus: D3DCLIPSTATUS9) -> Self { Self(clipstatus) }

    /// Convert a [ClipStatus] into a raw [D3DCLIPSTATUS9].
    ///
    /// [D3DCLIPSTATUS9]:       https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dclipstatus9
    pub const fn into(self) -> D3DCLIPSTATUS9 { self.0 }
}

impl Debug for ClipStatus {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("ClipStatus")
            .field("ClipUnion",         &self.0.ClipUnion)
            .field("ClipIntersection",  &self.0.ClipIntersection)
            .finish()
    }
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for ClipStatus {
    fn default() -> Self { Self(D3DCLIPSTATUS9 { ClipUnion: 0, ClipIntersection: 0 }) }
}

impl Deref for ClipStatus {
    type Target = D3DCLIPSTATUS9;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ClipStatus {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<ClipStatus> for D3DCLIPSTATUS9 {
    fn from(value: ClipStatus) -> Self { value.0 }
}

#[cfg(feature = "impl-from-unchecked")]
impl From<D3DCLIPSTATUS9> for ClipStatus {
    fn from(value: D3DCLIPSTATUS9) -> Self { Self(value) }
}

//#cpp2rust D3DCLIPSTATUS9 = d3d9::ClipStatus
