use crate::d3d9::*;

use winapi::shared::d3d9types::*;

use std::ops::{Deref, DerefMut};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dvolume-desc)\]
/// D3DVOLUME_DESC
///
/// Describes a [Volume].
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)] pub struct VolumeDesc {
    pub format:     Format,
    pub ty:         ResourceType,
    pub usage:      Usage,
    pub pool:       Pool,
    pub width:      u32,
    pub height:     u32,
    pub depth:      u32,
}

impl Deref    for VolumeDesc { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DVOLUME_DESC ; }
impl DerefMut for VolumeDesc { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DVOLUME_DESC > for VolumeDesc { fn from(value: D3DVOLUME_DESC ) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<VolumeDesc> for D3DVOLUME_DESC  { fn from(value: VolumeDesc     ) -> Self { unsafe { std::mem::transmute(value) } } }

test_layout! { VolumeDesc => unsafe D3DVOLUME_DESC  { format => Format, ty => Type, usage => Usage, pool => Pool, width => Width, height => Height, depth => Depth } }
