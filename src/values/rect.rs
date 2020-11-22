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
/// | D3DRECT   | RECT      |
/// | --------- | --------- |
/// | x1        | left      |
/// | y1        | top       |
/// | x2        | right     |
/// | y2        | bottom    |
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Rect(D3DRECT);

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

impl Deref for Rect {
    type Target = D3DRECT;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for Rect {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl From<RECT> for Rect {
    fn from(value: RECT) -> Self { Self(D3DRECT { x1: value.left, y1: value.top, x2: value.right, y2: value.bottom }) }
}

impl From<D3DRECT> for Rect {
    fn from(value: D3DRECT) -> Self { Self(value) }
}

impl From<Rect> for RECT {
    fn from(value: Rect) -> Self { RECT { left: value.0.x1, top: value.0.y1, right: value.0.x2, bottom: value.0.y2 } }
}

impl From<Rect> for D3DRECT {
    fn from(value: Rect) -> Self { value.0 }
}

impl From<(Range<i32>, Range<i32>)> for Rect {
    fn from(value: (Range<i32>, Range<i32>)) -> Self {
        Self(D3DRECT {
            x1: value.0.start,
            x2: value.0.end,
            y1: value.1.start,
            y2: value.1.end,
        })
    }
}

impl From<Range<(i32, i32)>> for Rect {
    fn from(value: Range<(i32, i32)>) -> Self {
        Self(D3DRECT {
            x1: value.start.0,
            x2: value.end.0,
            y1: value.start.1,
            y2: value.end.1,
        })
    }
}

impl From<[Range<i32>; 2]> for Rect {
    fn from(value: [Range<i32>; 2]) -> Self {
        Self(D3DRECT {
            x1: value[0].start,
            x2: value[0].end,
            y1: value[1].start,
            y2: value[1].end,
        })
    }
}

impl From<Range<[i32; 2]>> for Rect {
    fn from(value: Range<[i32; 2]>) -> Self {
        Self(D3DRECT {
            x1: value.start[0],
            x2: value.end[0],
            y1: value.start[1],
            y2: value.end[1],
        })
    }
}

#[test] fn layout() {
    use std::mem::*;

    assert_eq!(size_of::<Rect>(), size_of::<D3DRECT>());
    assert_eq!(size_of::<Rect>(), size_of::<   RECT>());

    assert_eq!(align_of::<Rect>(), align_of::<D3DRECT>());
    assert_eq!(align_of::<Rect>(), align_of::<   RECT>());
}
