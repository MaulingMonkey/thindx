use crate::d3d;
use bytemuck::*;



/// [u16] or [u32] - an index type safe for use with Direct3D 9
///
/// ### ⚠️ Safety ⚠️
///
/// By implementing this trait, you assert that:
///
/// *   You're returning a valid [d3d::Format] for an index buffer (e.g. [d3d::Format::Index16] or [d3d::Format::Index32])
/// *   The size/stride of the [d3d::Format] matches the size/stride of `Self` (undefined behavior may result if e.g. the alignment or size differs)
pub unsafe trait Index : Pod {
    /// The [d3d::Format] corresponding to `Self`
    fn format() -> d3d::Format;
}

unsafe impl Index for u16 { fn format() -> d3d::Format { d3d::Format::Index16 } }
unsafe impl Index for u32 { fn format() -> d3d::Format { d3d::Format::Index32 } }
