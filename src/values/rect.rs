#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;

use std::ops::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drect)\]
/// D3DRECT
#[derive(Clone, Copy)]
#[repr(transparent)] pub struct Rect(D3DRECT);

impl Rect {
    pub fn from(value: impl Into<Self>) -> Self { value.into() }
    pub fn into(self) -> D3DRECT { self.0 }

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

impl From<D3DRECT> for Rect {
    fn from(value: D3DRECT) -> Self { Self(value) }
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
