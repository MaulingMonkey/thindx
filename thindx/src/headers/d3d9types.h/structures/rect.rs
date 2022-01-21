#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;
use winapi::shared::windef::RECT;

use std::ops::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drect)\]
/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)\]
/// D3DRECT / RECT
///
/// Two near-identical, ABI-compatible, but slightly differently named/labeled sets of 4 `LONG`s
///
/// | [Rect] / [D3DRECT]    | [RECT]    |
/// | --------------------- | --------- |
/// | x1                    | left      |
/// | y1                    | top       |
/// | x2                    | right     |
/// | y2                    | bottom    |
#[derive(Clone, Copy, Default)]
#[repr(C)] pub struct Rect {
    /// left
    pub x1: i32,

    /// top
    pub y1: i32,

    /// right
    pub x2: i32,

    /// bottom
    pub y2: i32,
}

#[cfg(test)] mod vs_d3drect { use super::*; test_layout_only! { Rect => D3DRECT { x1 => x1,   y1 => y1,  x2 => x2,    y2 => y2     } } }
#[cfg(test)] mod vs_rect    { use super::*; test_layout_only! { Rect =>    RECT { x1 => left, y1 => top, x2 => right, y2 => bottom } } }

impl Deref    for Rect { fn deref    (&    self) -> &    Self::Target { unsafe { std::mem::transmute(self) } } type Target = D3DRECT; }
impl DerefMut for Rect { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute(self) } } }
impl From<D3DRECT> for Rect { fn from(value: D3DRECT) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Rect> for D3DRECT { fn from(value: Rect   ) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<RECT> for Rect { fn from(value: RECT) -> Self { unsafe { std::mem::transmute(value) } } }
impl From<Rect> for RECT { fn from(value: Rect) -> Self { unsafe { std::mem::transmute(value) } } }

impl Rect {
    pub fn from(value: impl Into<Self>) -> Self { value.into() }
    pub fn into<C: From<Self>>(self) -> C { C::from(self) }

    pub fn left     (&self) -> i32 { self.x1 }
    pub fn right    (&self) -> i32 { self.x2 }
    pub fn top      (&self) -> i32 { self.y1 }
    pub fn bottom   (&self) -> i32 { self.y2 }

    pub fn width    (&self) -> i32 { self.x2 - self.x1 }
    pub fn height   (&self) -> i32 { self.y2 - self.y1 }
}

impl Debug for Rect {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Rect")
            .field("x", &(self.left() .. self.right()))
            .field("y", &(self.top() .. self.bottom()))
            .finish()
    }
}

impl From<(Range<i32>, Range<i32>)> for Rect {
    fn from(value: (Range<i32>, Range<i32>)) -> Self {
        Self {
            x1: value.0.start,
            x2: value.0.end,
            y1: value.1.start,
            y2: value.1.end,
        }
    }
}

impl From<Range<(i32, i32)>> for Rect {
    fn from(value: Range<(i32, i32)>) -> Self {
        Self {
            x1: value.start.0,
            x2: value.end.0,
            y1: value.start.1,
            y2: value.end.1,
        }
    }
}

impl From<[Range<i32>; 2]> for Rect {
    fn from(value: [Range<i32>; 2]) -> Self {
        Self {
            x1: value[0].start,
            x2: value[0].end,
            y1: value[1].start,
            y2: value[1].end,
        }
    }
}

impl From<Range<[i32; 2]>> for Rect {
    fn from(value: Range<[i32; 2]>) -> Self {
        Self {
            x1: value.start[0],
            x2: value.end[0],
            y1: value.start[1],
            y2: value.end[1],
        }
    }
}

//#cpp2rust D3DRECT = d3d::Rect
//#cpp2rust RECT    = d3d::Rect
