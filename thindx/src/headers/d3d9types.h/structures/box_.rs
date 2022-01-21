#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/dibox3d9/d3dbox)\]
/// D3DBOX / BOX
///
/// Define a 3d volume with [u32]s: `{ left, top, right, bottom, front, back }`
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct Box {
    pub left:   u32,
    pub top:    u32,
    pub right:  u32,
    pub bottom: u32,
    pub front:  u32,
    pub back:   u32,
}

test_layout! { Box => D3DBOX { left => Left, top => Top, right => Right, bottom => Bottom, front => Front, back => Back } }

impl Deref    for Box { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DBOX; }
impl DerefMut for Box { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DBOX> for Box { fn from(value: D3DBOX) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Box> for D3DBOX { fn from(value: Box   ) -> Self { unsafe { std::mem::transmute(value) } } }

impl Box {
    pub fn from(value: impl Into<Self>) -> Self { value.into() }
    pub fn into<C: From<Self>>(self) -> C { C::from(self) }

    pub fn width    (&self) -> u32 { self.right - self.left }
    pub fn height   (&self) -> u32 { self.bottom - self.top }
    pub fn depth    (&self) -> u32 { self.back - self.front }
}

impl Debug for Box {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Box")
            .field("x", &(self.left .. self.right))
            .field("y", &(self.top .. self.bottom))
            .field("z", &(self.front .. self.back))
            .finish()
    }
}

impl From<(Range<u32>, Range<u32>, Range<u32>)> for Box {
    fn from(value: (Range<u32>, Range<u32>, Range<u32>)) -> Self {
        Self {
            left:   value.0.start,
            top:    value.1.start,
            right:  value.0.end,
            bottom: value.1.end,
            front:  value.2.start,
            back:   value.2.end,
        }
    }
}

impl From<Range<(u32, u32, u32)>> for Box {
    fn from(value: Range<(u32, u32, u32)>) -> Self {
        Self {
            left:   value.start.0,
            top:    value.start.1,
            right:  value.end.0,
            bottom: value.end.1,
            front:  value.start.2,
            back:   value.end.2,
        }
    }
}

impl From<[Range<u32>; 3]> for Box {
    fn from(value: [Range<u32>; 3]) -> Self {
        Self {
            left:   value[0].start,
            top:    value[1].start,
            right:  value[0].end,
            bottom: value[1].end,
            front:  value[2].start,
            back:   value[2].end,
        }
    }
}

impl From<Range<[u32; 3]>> for Box {
    fn from(value: Range<[u32; 3]>) -> Self {
        Self {
            left:   value.start[0],
            top:    value.start[1],
            right:  value.end[0],
            bottom: value.end[1],
            front:  value.start[2],
            back:   value.end[2],
        }
    }
}

//#cpp2rust D3DBOX  = d3d::Box
//#cpp2rust BOX     = d3d::Box
