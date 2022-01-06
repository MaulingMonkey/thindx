use crate::d3d::{Box, Rect};

use std::ops::{Range, RangeFull};



/// [`Range`] or [`..`](RangeFull)
pub trait IntoRangeOrFull<T> {
    /// ### Returns
    ///
    /// *   Some([Range]) to indicate a subrange
    /// *   [None] to indicate [RangeFull]
    fn into_range(self) -> Option<Range<T>>;
}

/// [`Rect`] or [`..`](RangeFull)
pub trait IntoRectOrFull {
    /// ### Returns
    ///
    /// *   Some([Rect]) to indicate a subrect
    /// *   [None] to indicate [RangeFull]
    fn into_rect(self) -> Option<Rect>;
}

/// [`Box`] or [`..`](RangeFull)
pub trait IntoBoxOrFull {
    /// ### Returns
    ///
    /// *   Some([Box]) to indicate a subbox
    /// *   [None] to indicate [RangeFull]
    fn into_box(self) -> Option<Box>;
}

impl<T> IntoRangeOrFull<T> for Range<T>     { fn into_range(self) -> Option<Range<T>> { Some(self) } }
impl<T> IntoRangeOrFull<T> for RangeFull    { fn into_range(self) -> Option<Range<T>> { None } }

impl<T: Into<Rect>> IntoRectOrFull for T    { fn into_rect(self) -> Option<Rect> { Some(self.into()) } }
impl IntoRectOrFull for RangeFull           { fn into_rect(self) -> Option<Rect> { None } }

impl<T: Into<Box>> IntoBoxOrFull for T      { fn into_box(self) -> Option<Box> { Some(self.into()) } }
impl IntoBoxOrFull for RangeFull            { fn into_box(self) -> Option<Box> { None } }
