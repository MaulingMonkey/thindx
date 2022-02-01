use crate::d3d;

use std::ops::{Range, RangeFull};



/// [`std::ops::Range`] or [`..`](RangeFull)
pub trait IntoRangeOrFull<T> {
    /// ### Returns
    /// *   Some([Range]) to indicate a subrange
    /// *   [None] to indicate [RangeFull]
    fn into_range(self) -> Option<Range<T>>;
}

/// [`d3d::Rect`] or [`..`](RangeFull)
pub trait IntoRectOrFull {
    /// ### Returns
    /// *   Some([d3d::Rect]) to indicate a subrect
    /// *   [None] to indicate [RangeFull]
    fn into_rect(self) -> Option<d3d::Rect>;
}

/// [`d3d::Box`] or [`..`](RangeFull)
pub trait IntoBoxOrFull {
    /// ### Returns
    /// *   Some([d3d::Box]) to indicate a subbox
    /// *   [None] to indicate [RangeFull]
    fn into_box(self) -> Option<d3d::Box>;
}

impl<T> IntoRangeOrFull<T> for Range<T>     { fn into_range(self) -> Option<Range<T>> { Some(self) } }
impl<T> IntoRangeOrFull<T> for RangeFull    { fn into_range(self) -> Option<Range<T>> { None } }

impl<T: Into<d3d::Rect>> IntoRectOrFull for T   { fn into_rect(self) -> Option<d3d::Rect> { Some(self.into()) } }
impl IntoRectOrFull for RangeFull               { fn into_rect(self) -> Option<d3d::Rect> { None } }

impl<T: Into<d3d::Box>> IntoBoxOrFull for T     { fn into_box(self) -> Option<d3d::Box> { Some(self.into()) } }
impl IntoBoxOrFull for RangeFull                { fn into_box(self) -> Option<d3d::Box> { None } }
