#[allow(unused_imports)] use crate::*;

use bytemuck::*;

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
///
/// [D3DRECT]:  https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3drect
/// [RECT]:     https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)]
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

struct_mapping! {
    #[derive(unsafe { AsRef, AsMut, Deref, DerefMut, FromInto })]
    Rect => D3DRECT {
        x1 => x1,
        y1 => y1,
        x2 => x2,
        y2 => y2,
    }

    #[derive(unsafe { AsRef, AsMut, FromInto })]
    Rect => RECT {
        #[renamed] x1 => left,
        #[renamed] y1 => top,
        #[renamed] x2 => right,
        #[renamed] y2 => bottom
    }
}

impl Rect {
    pub const fn zeroed() -> Self { Self { x1: 0, y1: 0, x2: 0, y2: 0 } }

    /// ### Examples
    /// ```rust
    /// # use thindx::d3d::Rect;
    /// let r = Rect { x1: 0, y1: 10, x2: 100, y2: 90 };
    /// assert_eq!(r, Rect::from((0 .. 100, 10 .. 90)));
    /// assert_eq!(r, Rect::from((0, 10) .. (100, 90)));
    /// assert_eq!(r, Rect::from([0 .. 100, 10 .. 90]));
    /// assert_eq!(r, Rect::from([0, 10] .. [100, 90]));
    /// ```
    pub fn from(value: impl Into<Self>) -> Self { value.into() }

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
